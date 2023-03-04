use chrono::Utc;
use flatbuffers::FlatBufferBuilder;

use hyper::{
  header::{self, HeaderValue},
  http::Error,
  Body, Method, Request, Response, StatusCode,
};
use tokio::net::TcpListener;
use uuid::Uuid;

use std::net::SocketAddr;
use webrtc_unreliable::SessionEndpoint;

use crate::fbs::{
  ClientMessages::clientmessages::root_as_client_message,
  Game::game::{GamePhase, GameT, PlayerRolesT},
  ServerMessages::servermessages::{
    JoinGameResponsePayload, JoinGameResponsePayloadArgs, NewGameResponsePayload,
    NewGameResponsePayloadArgs, ResponseCode, ServerMessage, ServerMessageArgs,
    ServerMessagePayload,
  },
};

use super::{state::ArcState, ws_service::ws_service};

#[derive(Debug, Clone)]
pub struct UserError(pub String);

pub async fn http_service(
  req: Request<Body>,
  remote_addr: SocketAddr,
  session_endpoint: &mut SessionEndpoint,
  state: ArcState,
) -> Result<Response<Body>, hyper::http::Error> {
  match (req.uri().path(), req.method()) {
    // endpoint for WebRTC session requests
    ("/offer", &Method::POST) => {
      log::info!("WebRTC session request from {}", remote_addr);
      match session_endpoint.http_session_request(req.into_body()).await {
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

          make_error_res(err)
        }
      }
    }

    ("/ping", _) => Response::builder()
      .header(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
      )
      .status(StatusCode::OK)
      .body(Body::from("pong")),

    /*
     * New game request
     *
     * Client sends: ClientMessage with NewGamePayload + User payload
     * Server responds: Game payload
     */
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

          if let Some(_) = player_in_game(user_id, state.clone()) {
            return make_error_res(format!("user {} is already in a game", user_id));
          }

          // construct server message bytes
          let (response, game_id) = create_game(state.clone());

          // get some available port for the websocket listener
          let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind websocket listener");

          // create WS service for the game chat and system messages
          tokio::spawn(async move {
            ws_service(listener, state, game_id).await;
          });

          Response::builder()
            .header(
              header::ACCESS_CONTROL_ALLOW_ORIGIN,
              HeaderValue::from_static("*"),
            )
            .status(StatusCode::OK)
            .body(Body::from(response))
        }
        Err(err) => make_error_res(err),
      }
    }

    ("/join_game", &Method::POST) => {
      let (_, body) = req.into_parts();
      let body = hyper::body::to_bytes(body).await;

      match body {
        Ok(body) => {
          log::debug!("join game request from {}", remote_addr);
          let message = root_as_client_message(&body).expect("failed to parse message");
          let payload = message
            .payload_as_join_game_payload()
            .expect("failed to parse payload")
            .unpack();

          let user = payload.user.expect("failed to parse user");
          let user_id = user.id.expect("failed to parse user id");
          let game_id = payload
            .game_id
            .expect("failed to parse game id")
            .to_string();

          let local_state = state.read().unwrap();
          let lobbies = local_state.lobbies.read().unwrap();

          let state = state.clone();
          if let Some(cur_game_id) = player_in_game(&user_id, state.clone()) {
            log::debug!("user {} is already in a game", user_id);
            return return_player_to_game(cur_game_id, state);
          }

          let mut games = local_state.games.write().unwrap();
          let game = games.get_mut(&game_id).expect("failed to get game");
          let lobby = lobbies.get(&game_id).expect("failed to get lobby");

          let players = game.players.as_mut().expect("failed to parse players");

          let user_id_local = Some(user_id.clone());
          if players.red == None {
            players.red = user_id_local;
          } else if players.blue == None {
            players.blue = user_id_local;
          } else if players.green == None {
            players.green = user_id_local;
          } else if players.yellow == None {
            players.yellow = user_id_local;
          } else {
            // TODO: return proper error response via ServerMessage
            // using ResponseCode::Error
            return make_error_res("game is full");
          }

          let response = join_game(game, lobby.addr.to_string());

          state
            .read()
            .expect("could not write to state")
            .player_games
            .write()
            .expect("could not write to player_games")
            .insert(user_id.clone(), game_id);

          Response::builder()
            .header(
              header::ACCESS_CONTROL_ALLOW_ORIGIN,
              HeaderValue::from_static("*"),
            )
            .status(StatusCode::OK)
            .body(Body::from(response))
        }
        Err(err) => make_error_res(err),
      }
    }

    _ => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::from("not found")),
  }
}

fn make_error_res<T: std::fmt::Debug>(err: T) -> Result<Response<Body>, Error> {
  return Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .body(Body::from(format!("error: {:?}", err)));
}

fn create_game(state: ArcState) -> (Vec<u8>, String) {
  // TODO: get ID from DB
  let game_id = Uuid::new_v4().to_string();
  let mut builder = FlatBufferBuilder::new();

  let game_id_offset = builder.create_string(&game_id.to_string());

  let payload = NewGameResponsePayload::create(
    &mut builder,
    &NewGameResponsePayloadArgs {
      game_id: Some(game_id_offset),
      code: ResponseCode::OK,
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
    starttime: Utc::now().timestamp_millis() as u64,
    players: Some(Box::new(PlayerRolesT {
      ..Default::default()
    })),
    ..Default::default()
  };

  state
    .read()
    .unwrap()
    .games
    .write()
    .unwrap()
    .insert(game_id.clone(), game);

  return (bytes.to_vec(), game_id);
}

fn join_game(game: &mut GameT, socket_addr: String) -> Vec<u8> {
  let mut builder = FlatBufferBuilder::new();

  let socket_addr = builder.create_string(&format!("ws://{}", socket_addr));
  let game = game.pack(&mut builder);

  let payload = JoinGameResponsePayload::create(
    &mut builder,
    &JoinGameResponsePayloadArgs {
      code: ResponseCode::OK,
      game: Some(game),
      socket_address: Some(socket_addr),
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
  return builder.finished_data().to_vec();
}

fn player_in_game(user_id: &str, state: ArcState) -> Option<String> {
  state
    .read()
    .unwrap()
    .player_games
    .read()
    .unwrap()
    .get(user_id)
    .cloned()
}

fn return_player_to_game(
  game_id: String,
  state: ArcState,
) -> Result<Response<Body>, hyper::http::Error> {
  let inner_state = state.read().unwrap();
  let lobbies = inner_state.lobbies.read().unwrap();
  let mut games = inner_state.games.write().unwrap();
  let cur_lobby = lobbies.get(&game_id).expect("failed to get lobby");
  let cur_game = games.get_mut(&game_id).expect("failed to get game");
  let response = join_game(cur_game, cur_lobby.addr.to_string());

  return Response::builder()
    .header(
      header::ACCESS_CONTROL_ALLOW_ORIGIN,
      HeaderValue::from_static("*"),
    )
    .status(StatusCode::OK)
    .body(Body::from(response));
}
