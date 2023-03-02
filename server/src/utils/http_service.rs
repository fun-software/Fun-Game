use chrono::Utc;
use flatbuffers::FlatBufferBuilder;

use hyper::{
  header::{self, HeaderValue},
  http::Error,
  Body, Method, Request, Response, StatusCode,
};
use rand::random;
use tokio::net::TcpListener;

use std::net::SocketAddr;
use webrtc_unreliable::SessionEndpoint;

use crate::fbs::{
  ClientMessages::clientmessages,
  Game::game::{Game, GameArgs, GamePhase, GameT, PlayerRoles, PlayerRolesArgs},
  ServerMessages::servermessages::{
    root_as_server_message, JoinGameResponsePayload, JoinGameResponsePayloadArgs,
    NewGameResponsePayload, NewGameResponsePayloadArgs, ResponseCode, ServerMessage,
    ServerMessageArgs, ServerMessagePayload,
  },
  User::user::User,
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
          log::info!("new game request from {}", remote_addr);

          let message =
            clientmessages::root_as_client_message(&body).expect("failed to parse message");

          let payload = message
            .payload_as_new_game_payload()
            .expect("failed to parse payload");
          let user = payload.user().expect("failed to parse user");

          let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind websocket listener");

          let id: u64 = 1; //random();

          // construct server message bytes
          let (response, game) = create_game(user, id, listener.local_addr().unwrap().to_string());

          state
            .write()
            .unwrap()
            .games
            .write()
            .unwrap()
            .insert(id, game);

          // create WS service for the game chat and system messages
          tokio::spawn(async move {
            ws_service(listener, state, id).await;
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
          log::info!("join game request from {}", remote_addr);

          let message =
            clientmessages::root_as_client_message(&body).expect("failed to parse message");

          let payload = message
            .payload_as_join_game_payload()
            .expect("failed to parse payload");
          let user = payload.user().expect("failed to parse user");
          let game_id = payload.game_id();

          let state = state.read().unwrap();
          let mut games = state.games.write().unwrap();
          let game = games.get_mut(&game_id);

          let lobbies = state.lobbies.read().unwrap();
          let lobby = lobbies.get(&game_id);

          match (lobby, game) {
            (Some(lobby), Some(game)) => {
              let players = game.players.as_mut().unwrap();

              if players.red == 0 {
                players.red = user.id();
              } else if players.blue == 0 {
                players.blue = user.id();
              } else if players.green == 0 {
                players.green = user.id();
              } else if players.yellow == 0 {
                players.yellow = user.id();
              } else {
                return make_error_res("game is full");
              }

              let response = join_game(game, lobby.addr.to_string());

              log::info!("game joined: {:?}", game);

              Response::builder()
                .header(
                  header::ACCESS_CONTROL_ALLOW_ORIGIN,
                  HeaderValue::from_static("*"),
                )
                .status(StatusCode::OK)
                .body(Body::from(response))
            }
            _ => {
              log::warn!("game/lobby not found: {:?}", game_id);
              make_error_res("game not found")
            }
          }
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

fn create_game(user: User, game_id: u64, socket_addr: String) -> (Vec<u8>, GameT) {
  let mut builder = FlatBufferBuilder::new();

  // create a new game with the user as the red player
  let players = PlayerRoles::create(
    &mut builder,
    &PlayerRolesArgs {
      red: user.id(),
      ..Default::default()
    },
  );

  let game = Game::create(
    &mut builder,
    &GameArgs {
      id: game_id,
      phase: GamePhase::Lobby,
      starttime: Utc::now().timestamp_millis() as u64,
      players: Some(players),
      ..Default::default()
    },
  );

  let socket_addr = builder.create_string(&format!("ws://{}", socket_addr));

  let payload = NewGameResponsePayload::create(
    &mut builder,
    &NewGameResponsePayloadArgs {
      game: Some(game),
      code: ResponseCode::OK,
      socket_address: Some(socket_addr),
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

  // need to write de-serialized game to state, so must
  // de-serialize it here from the raw buffer :^)
  let game_msg = root_as_server_message(bytes)
    .unwrap()
    .payload_as_new_game_response_payload()
    .unwrap()
    .game();

  let game_obj = game_msg.unwrap().unpack();

  return (bytes.to_vec(), game_obj);
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
