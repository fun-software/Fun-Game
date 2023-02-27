#[allow(non_snake_case, unused_imports)]
#[path = "../flatbuffers/Chat.rs"]
mod Chat;

#[allow(non_snake_case, unused_imports)]
#[path = "../flatbuffers/ClientMessages.rs"]
mod ClientMessages;
use ClientMessages::client_messages::{self, ClientMessagePayload};

#[allow(non_snake_case, unused_imports)]
#[path = "../flatbuffers/ServerMessages.rs"]
mod ServerMessages;
use ServerMessages::server_messages::{
    JoinGameResponsePayload, JoinGameResponsePayloadArgs, LeaveGameResponsePayload, LeaveGameResponsePayloadArgs,
    ServerMessage, ServerMessageArgs, ServerMessagePayload, ResponseCode,
};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use hyper::{
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Error, Method, Response, StatusCode,
};
use tokio::{
    runtime,
    sync::RwLock,
    time::{self, Duration},
};
use webrtc_unreliable::{MessageType, Server};

use dotenv::dotenv;

type PlayerMap = HashMap<SocketAddr, String>;

#[allow(non_snake_case)]
fn main() {
    dotenv().ok();

    let PUBLIC_PORT: String = std::env::var("PUBLIC_PORT").expect("PUBLIC_PORT must be set.");
    let WEBRTC_PORT: String = std::env::var("WEBRTC_PORT").expect("WEBRTC_PORT must be set.");
    let LISTEN_PORT: String = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set.");

    let SERVER_URL: String = std::env::var("SERVER_URL").expect("SERVER_URL must be set.");

    let TICK_RATE: u64 = std::env::var("TICK_RATE").unwrap().parse::<u64>().unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("could not create tokio runtime");

    let public_webrtc_addr: SocketAddr = format!("{SERVER_URL}:{PUBLIC_PORT}").parse().unwrap();
    let webrtc_listen_addr: SocketAddr = format!("{SERVER_URL}:{WEBRTC_PORT}").parse().unwrap();
    let session_listen_addr: SocketAddr = format!("{SERVER_URL}:{LISTEN_PORT}").parse().unwrap();

    rt.block_on(async {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(TICK_RATE));

            loop {
                interval.tick().await;

                // flush the queue, make state updates
                println!("Tick")
            }
        });

        let players = Arc::new(RwLock::new(PlayerMap::new()));

        let mut rtc_server = Server::new(webrtc_listen_addr, public_webrtc_addr)
            .await
            .expect("could not start RTC server");

        let session_endpoint = rtc_server.session_endpoint();
        let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
            let session_endpoint = session_endpoint.clone();
            let remote_addr = addr_stream.remote_addr();

            async move {
                Ok::<_, Error>(service_fn(move |req| {
                    let mut session_endpoint = session_endpoint.clone();
                    async move {
                        if req.uri().path() == "/offer" && req.method() == Method::POST {
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
                                    Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("error: {:?}", err)))
                                }
                            }
                        } else {
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from("not found"))
                        }
                    }
                }))
            }
        });

        tokio::spawn(async move {
            hyper::server::Server::bind(&session_listen_addr)
                .serve(make_svc)
                .await
                .expect("HTTP session server has died");
        });

        let mut message_buf = Vec::<u8>::new();
        loop {
            let received = match rtc_server.recv().await {
                Ok(received) => {
                    message_buf.clear();
                    message_buf.extend(received.message.as_ref());
                    Some((received.message_type, received.remote_addr))
                }
                Err(err) => {
                    log::warn!("could not receive RTC message: {:?}", err);
                    None
                }
            };

            if let Some((message_type, remote_addr)) = received {
                let res =
                    handle_msg(&message_buf, message_type, remote_addr, players.clone()).await;
                message_buf.clear();

                message_buf.extend(res);

                match rtc_server
                    .send(&message_buf, message_type, &remote_addr)
                    .await
                {
                    Ok(_) => {}
                    Err(err) => {
                        log::warn!("could not send RTC message: {:?}", err);
                    }
                }
            }
        }
    });
}

async fn handle_msg(
    message_buf: &Vec<u8>,
    message_type: MessageType,
    remote_addr: SocketAddr,
    players: Arc<RwLock<PlayerMap>>,
) -> Vec<u8> {
    match message_type {
        MessageType::Binary => {
            if let Ok(msg) = client_messages::root_as_client_message(&message_buf) {
                return match msg.payload_type() {
                    ClientMessagePayload::JoinGamePayload => {
                        let join_game_payload = msg
                            .payload_as_join_game_payload()
                            .expect("could not parse join payload");
                        let user = join_game_payload.user();

                        match user {
                            Some(user) => handle_join(user.username().unwrap(), remote_addr, players).await,
                            None => vec![0u8; 0],
                        }
                    }
                    ClientMessagePayload::LeaveGamePayload => {
                        let leave_game_payload = msg
                            .payload_as_leave_game_payload()
                            .expect("could not parse leave payload");
                        let user = leave_game_payload.user();

                        match user {
                            Some(user) => handle_leave(user.username().unwrap(), remote_addr, players).await,
                            None => vec![0u8; 0],
                        }
                    }
                    ClientMessagePayload::ChatPayload => {
                        let chat_payload = msg
                            .payload_as_chat_payload()
                            .expect("could not parse chat payload");
                        let user = chat_payload.user();
                        let message = chat_payload.message();

                        match (user, message) {
                            _ => vec![0u8; 0],
                        }
                    }
                    ClientMessagePayload::QueryStatePayload => {
                        println!("User queried for state!");
                        vec![0u8; 0]
                    }
                    _ => vec![0u8; 0],
                };
            } else {
                log::warn!("could not parse binary client message");
            }
        }
        MessageType::Text => {
            log::warn!("received string message: {:#?}", message_buf);
        }
    }

    return vec![0u8; 0];
}

async fn handle_join(
    name: &str,
    remote_addr: SocketAddr,
    players: Arc<RwLock<PlayerMap>>,
) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    if players.read().await.contains_key(&remote_addr) {
        log::warn!("{} tried to join twice", name);
        return vec![0u8; 0];
    }
    log::info!("{} joined", name);
    players.write().await.insert(remote_addr, name.to_string());

    let payload = JoinGameResponsePayload::create(
        &mut builder,
        &JoinGameResponsePayloadArgs {
            game: None, code: ResponseCode::OK,
        },
    );

    let res = ServerMessage::create(
        &mut builder,
        &ServerMessageArgs {
            timestamp: 0,
            payload_type: ServerMessagePayload::JoinGameResponsePayload,
            payload: Some(payload.as_union_value()),
        },
    );

    builder.finish(res, None);

    return builder.finished_data().to_vec();
}

async fn handle_leave(
    name: &str,
    remote_addr: SocketAddr,
    players: Arc<RwLock<PlayerMap>>,
) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    if !players.read().await.contains_key(&remote_addr) {
        log::warn!("{} tried to leave, but is not playing.", name);
        return vec![0u8; 0];
    }
    log::info!("{} left", name);
    players.write().await.remove(&remote_addr);

    let payload = LeaveGameResponsePayload::create(
        &mut builder,
        &LeaveGameResponsePayloadArgs {
            game: None, code: ResponseCode::OK,
        },
    );

    let res = ServerMessage::create(
        &mut builder,
        &ServerMessageArgs {
            timestamp: 0,
            payload_type: ServerMessagePayload::LeaveGameResponsePayload,
            payload: Some(payload.as_union_value()),
        },
    );

    builder.finish(res, None);

    return builder.finished_data().to_vec();
}
