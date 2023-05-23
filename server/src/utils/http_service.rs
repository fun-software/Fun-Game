use chrono::Utc;
use flatbuffers::FlatBufferBuilder;

use hyper::{
  header::{self, HeaderValue},
  http::Error,
  Body, Method, Request, Response, StatusCode,
};

use serde_derive::Deserialize;
use tokio::net::TcpListener;
use uuid::Uuid;

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use crate::{
  fbs::{
    ClientMessages::clientmessages::root_as_client_message,
    Game::game::{GamePhase, GameT},
    GameState::gamestate::GameStateT,
    Player::player::{PlayerT, Role},
    ServerMessages::servermessages::{
      JoinGameResponsePayload, JoinGameResponsePayloadArgs, NewGameResponsePayload,
      NewGameResponsePayloadArgs, ServerMessage, ServerMessageArgs, ServerMessagePayload,
    },
    Math::math::Vec3T,
  },
  utils::web_rtc_service::web_rtc_service,
};

use super::{
  state::{AsyncState, Lobby},
  ws_service::ws_service,
};

#[derive(Debug, Clone)]
pub struct UserError(pub String);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SDPOffer {
  sdp: String,
  game_id: String,
}

pub async fn http_service(
  req: Request<Body>,
  remote_addr: SocketAddr,
  state: AsyncState,
) -> Result<Response<Body>, hyper::http::Error> {
  match (req.uri().path(), req.method()) {
    // endpoint for WebRTC session requests
    ("/offer", &Method::POST) => {
      log::info!("WebRTC session request from {}", remote_addr);

      return handle_offer(req, state).await;
    }

    ("/new_game", &Method::POST) => {
      let (_, body) = req.into_parts();
      let body = hyper::body::to_bytes(body).await;

      match body {
        Ok(body) => {
          log::debug!("new game request from {}", remote_addr);
          let message = root_as_client_message(&body).expect("failed to parse message");
          let payload = message
            .payload_as_new_game_payload()
            .expect("failed to parse payload");

          let user = payload.user().expect("failed to parse user");
          let user_id = user.id().expect("failed to parse user id");

          if let Some(_) = state.read().await.player_in_game(user_id) {
            return make_error_response(format!("user {} is already in a game", user_id));
          }

          // construct server message bytes
          let response = create_game(state.clone()).await;

          make_response(response)
        }
        Err(err) => make_error_response(err),
      }
    }

    ("/join_game", &Method::POST) => {
      let (_, body) = req.into_parts();
      let body = hyper::body::to_bytes(body).await;

      match body {
        Ok(body) => {
          let message = root_as_client_message(&body).unwrap();
          let payload = message.payload_as_join_game_payload().unwrap().unpack();
          let user = payload.user.unwrap();
          let user_id = user.id.unwrap();
          let game_id = payload.game_id.unwrap().to_string();

          let cur_game_id = state.read().await.player_in_game(&user_id);

          if let Some(cur_game_id) = cur_game_id {
            return make_response(return_player_to_game(cur_game_id, state.clone()).await);
          }

          if !state.read().await.game_states.contains_key(&game_id) {
            return make_error_response(format!("game {} does not exist", game_id));
          }

          if state.read().await.player_count(game_id.clone()) >= 4 {
            return make_error_response(format!("game {} is full", game_id));
          }

          let response = join_game(user_id, game_id, state.clone()).await;

          make_response(response)
        }
        Err(err) => make_error_response(err),
      }
    }

    ("/leave_game", &Method::POST) => {
      let (_, body) = req.into_parts();
      let body = hyper::body::to_bytes(body).await;
      if let Err(err) = body {
        return make_error_response(err);
      }

      let body = body.unwrap();
      let message = root_as_client_message(&body).unwrap();
      let payload = message.payload_as_leave_game_payload().unwrap().unpack();
      let user_id = payload.user.unwrap().id.unwrap();
      let game_id = state.read().await.player_in_game(&user_id);

      match game_id {
        Some(game_id) => {
          let response = leave_game(user_id, game_id, state).await;

          make_response(response)
        }
        None => make_error_response("user is not in a game"),
      }
    }

    _ => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::from("not found")),
  }
}

fn make_response(res: Vec<u8>) -> Result<Response<Body>, Error> {
  Response::builder()
    .header(
      // TODO: make stricter CORS policy
      header::ACCESS_CONTROL_ALLOW_ORIGIN,
      HeaderValue::from_static("*"),
    )
    .status(StatusCode::OK)
    .body(Body::from(res))
}

fn make_error_response<T: std::fmt::Debug>(err: T) -> Result<Response<Body>, Error> {
  log::error!("error: {:?}", err);
  Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .body(Body::from(format!("error: {:?}", err)))
}

async fn handle_offer(req: Request<Body>, state: AsyncState) -> Result<Response<Body>, Error> {
  let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
  let offer: SDPOffer = serde_json::from_slice(&body).unwrap();

  let state = state.read().await;
  let web_rtc_servers = &state.web_rtc_sessions;
  let session_endpoint = web_rtc_servers
    .get(&offer.game_id)
    .expect("no WebRTC server found for game");

  let mut session_endpoint = session_endpoint.clone();
  let sdp_req = Body::from(offer.sdp.clone());

  match session_endpoint.http_session_request(sdp_req).await {
    Ok(mut resp) => {
      resp.headers_mut().insert(
        // TODO: make stricter CORS policy
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
      );

      Ok(resp.map(Body::from))
    }
    Err(err) => {
      log::warn!("bad rtc session request: {:?}", err);

      make_error_response(err)
    }
  }
}

async fn create_game(state: AsyncState) -> Vec<u8> {
  // TODO: get ID from DB
  let game_id = Uuid::new_v4().to_string();
  let mut builder = FlatBufferBuilder::new();

  let game_id_offset = builder.create_string(&game_id.to_string());
  let payload = NewGameResponsePayload::create(
    &mut builder,
    &NewGameResponsePayloadArgs {
      game_id: Some(game_id_offset),
    },
  );

  let message = ServerMessage::create(
    &mut builder,
    &ServerMessageArgs {
      payload_type: ServerMessagePayload::NewGameResponsePayload,
      payload: Some(payload.as_union_value()),
      timestamp: Utc::now().timestamp_millis() as u64,
    },
  );

  builder.finish(message, None);
  let bytes = builder.finished_data();

  let game = GameT {
    id: Some(game_id.clone()),
    phase: GamePhase::Lobby,
    start_time: Utc::now().timestamp_millis() as u64,
    players: Some(vec![]),
    ..Default::default()
  };

  let game_state = GameStateT {
      game: Some(Box::new(game)),
      players: Some(vec![]),
  };

  // get some available port for the websocket listener
  let ws_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
  let webrtc_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

  // create WS service for the game chat and system messages
  let ws_game_id = game_id.clone();

  if state.read().await.lobbies.contains_key(&game_id) {
    log::debug!(
      "Lobby for game {} already exists, not creating new one",
      game_id
    );
    return "Lobby already exists".as_bytes().to_vec();
  }

  // add the game to the state, do here rather than
  // in the ws_service thread so that there
  // is no race condition between ws_service and the
  // imminent /join_game request
  let mut inner_state = state.write().await;
  inner_state.game_states.insert(game_id.clone(), game_state);
  inner_state.lobbies.insert(
    game_id.clone(),
    Lobby {
      addr: ws_listener.local_addr().unwrap(),
      peers: Arc::new(std::sync::RwLock::new(HashMap::new())),
    },
  );
  drop(inner_state);

  tokio::spawn(async move {
    ws_service(ws_listener, ws_game_id).await;
  });

  // create webRTC session endpoint for the game
  let web_rtc_state = state.clone();
  let web_rtc_game_id = game_id.clone();
  tokio::spawn(async move {
    web_rtc_service(webrtc_listener, web_rtc_state, &web_rtc_game_id).await;
  });

  return bytes.to_vec();
}

async fn join_game(user_id: String, game_id: String, state: AsyncState) -> Vec<u8> {
  let ws_port = state
    .read()
    .await
    .lobbies
    .get(&game_id)
    .unwrap()
    .addr
    .port();

  let mut inner_state = state.write().await;
  inner_state
    .player_games
    .insert(user_id.clone(), game_id.clone());

  let mut game = None;
  let mut builder = FlatBufferBuilder::new();

  let player = PlayerT  {
    hp: 100,
    id: Some(user_id.clone()),
    look_direction: Some(Vec3T::default()),
    position: Some(Vec3T::default()),
    sneaking: false,
    speed: 100,
    sprinting: false,
    role: Role::Default,
    velocity: Some(Vec3T::default())
  };

  if let Some(game_state) = inner_state.game_states.get_mut(&game_id) {
      let game_inner = game_state.game.as_ref();
      game = Some(game_inner.expect("Game metadata not found").pack(&mut builder));

      if let Some(players) = game_state.players.as_mut() {
          players.push(player);
      }
  }
  else {
      println!("Oopsie");
  }

  // drop the write lock
  drop(inner_state);

  let payload = JoinGameResponsePayload::create(
    &mut builder,
    &JoinGameResponsePayloadArgs {
      game: game,
      ws_port,
    },
  );

  let message = ServerMessage::create(
    &mut builder,
    &ServerMessageArgs {
      payload_type: ServerMessagePayload::JoinGameResponsePayload,
      payload: Some(payload.as_union_value()),
      timestamp: Utc::now().timestamp_millis() as u64,
    },
  );

  builder.finish(message, None);
  log::debug!("user {} joined game {}", user_id, game_id);
  return builder.finished_data().to_vec();
}

async fn leave_game(user_id: String, game_id: String, state: AsyncState) -> Vec<u8> {
  let mut inner_state = state.write().await;

  inner_state
    .game_states
    .get_mut(&game_id)
    .unwrap()
    .players
    .as_mut()
    .unwrap()
    .retain(|player| player.id.as_ref() != Some(&user_id));

  inner_state.player_games.remove(&user_id);

  drop(inner_state);

  return "ok".as_bytes().to_vec();
}

async fn return_player_to_game(game_id: String, state: AsyncState) -> Vec<u8> {
  let lobbies = &state.read().await.lobbies;
  let game_states = &state.read().await.game_states;
  let cur_lobby = lobbies.get(&game_id).unwrap();
  let cur_game_state = game_states.get(&game_id).unwrap().clone();
  let cur_game = cur_game_state.game;

  let ws_port = cur_lobby.addr.port();

  let mut builder = FlatBufferBuilder::new();

  let game_offset = cur_game.expect("No game found for the current game state").pack(&mut builder);
  let payload_offset = JoinGameResponsePayload::create(
    &mut builder,
    &JoinGameResponsePayloadArgs {
      game: Some(game_offset),
      ws_port,
    },
  );

  let message_offset = ServerMessage::create(
    &mut builder,
    &ServerMessageArgs {
      timestamp: chrono::Utc::now().timestamp() as u64,
      payload_type: ServerMessagePayload::JoinGameResponsePayload,
      payload: Some(payload_offset.as_union_value()),
    },
  );

  builder.finish(message_offset, None);

  return builder.finished_data().to_vec();
}
