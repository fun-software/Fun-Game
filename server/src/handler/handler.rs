#[allow(non_snake_case, unused_imports)]
#[path = "../../flatbuffers/Chat.rs"]
mod Chat;

#[allow(non_snake_case, unused_imports)]
#[path = "../../flatbuffers/ClientMessages.rs"]
mod ClientMessages;
use ClientMessages::client_messages::{self, ClientMessagePayload};

#[allow(non_snake_case, unused_imports)]
#[path = "../../flatbuffers/ServerMessages.rs"]
mod ServerMessages;
use ServerMessages::server_messages::{
    JoinGameResponsePayload, JoinGameResponsePayloadArgs, LeaveGameResponsePayload, LeaveGameResponsePayloadArgs,
    ServerMessage, ServerMessageArgs, ServerMessagePayload, ResponseCode,
};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::{
    sync::RwLock,
};
use webrtc_unreliable::{MessageType};

type PlayerMap = HashMap<SocketAddr, String>;

pub async fn handle_msg(
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
                        let query_payload = msg
                            .payload_as_query_state_payload()
                            .expect("could not parse query payload");
                        let user = query_payload.user();

                        match user {
                            Some(user) => handle_query_state(user.username().unwrap(), remote_addr, players).await,
                            None => vec![0u8; 0]
                        }
                    }
                    ClientMessagePayload::InputPayload => {
                        let input_payload = msg
                            .payload_as_input_payload()
                            .expect("could not parse input payload");
                        let user = input_payload.user();

                        match user {
                            Some(user) => handle_input(user.username().unwrap(), remote_addr, players).await,
                            None => vec![0u8; 0]
                        }
                    }
                    _ => vec![0u8; 0]
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

#[allow(unused_variables)]
async fn handle_query_state(
    name: &str,
    remote_addr: SocketAddr,
    players: Arc<RwLock<PlayerMap>>
) -> Vec<u8> {
    vec![0u8; 0]
}

#[allow(unused_variables)]
async fn handle_input(
    name: &str,
    remote_addr: SocketAddr,
    players: Arc<RwLock<PlayerMap>>
) -> Vec<u8> {
    vec![0u8; 0]
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
