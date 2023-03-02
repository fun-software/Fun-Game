use chrono::Utc;
use flatbuffers::FlatBufferBuilder;

use hyper::{
  header::{self, HeaderValue},
  http::Error,
  Body, Method, Request, Response, StatusCode,
};
use tokio::net::TcpListener;

use std::net::SocketAddr;
use webrtc_unreliable::SessionEndpoint;

use crate::fbs::{
  ClientMessages::clientmessages,
  ServerMessages::{
    game::{Game, GameArgs, GamePhase, PlayerRoles, PlayerRolesArgs},
    servermessages::{
      NewGameResponsePayload, NewGameResponsePayloadArgs, ResponseCode, ServerMessage,
      ServerMessageArgs, ServerMessagePayload,
    },
  },
};

use super::{state::ArcState, ws_service::ws_service};

#[derive(Debug, Clone)]
pub struct UserError(pub String);

pub async fn http_service(
  req: Request<Body>,
  remote_addr: SocketAddr,
  session_endpoint: &mut SessionEndpoint,
  state: ArcState<'_>,
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

          let mut builder = FlatBufferBuilder::new();

          let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind websocket listener");

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
              id: 1,
              phase: GamePhase::Lobby,
              starttime: Utc::now().timestamp_millis() as u64,
              players: Some(players),
              ..Default::default()
            },
          );

          let socket_addr =
            builder.create_string(&format!("ws://{}", listener.local_addr().unwrap()));

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
          let bytes = builder.finished_data().to_vec();

          // create WS service for the game chat and system messages
          tokio::spawn(async move {
            ws_service(listener).await;
          });

          Response::builder()
            .header(
              header::ACCESS_CONTROL_ALLOW_ORIGIN,
              HeaderValue::from_static("*"),
            )
            .status(StatusCode::OK)
            .body(Body::from(bytes))
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
