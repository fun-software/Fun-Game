// use crate::fbs::{
//   ClientMessages::clientmessages::{self, ClientMessagePayload},
//   ServerMessages::servermessages::{
//     JoinGameResponsePayload, JoinGameResponsePayloadArgs, LeaveGameResponsePayload,
//     LeaveGameResponsePayloadArgs, ResponseCode, ServerMessage, ServerMessageArgs,
//     ServerMessagePayload,
//   },
// };

// use std::{
//   collections::HashMap,
//   net::SocketAddr,
//   sync::{Arc, RwLock},
// };

// use webrtc_unreliable::MessageType;

// use super::state::ArcState;

// type PlayerMap = HashMap<SocketAddr, String>;

// pub fn handle_msg(
//   message_buf: &Vec<u8>,
//   message_type: MessageType,
//   remote_addr: SocketAddr,
//   state: ArcState,
// ) -> Vec<u8> {
//   let players = state.read().unwrap().players.clone();

//   match message_type {
//     MessageType::Binary => {
//       if let Ok(msg) = clientmessages::root_as_client_message(&message_buf) {
//         return match msg.payload_type() {
//           ClientMessagePayload::JoinGamePayload => {
//             let join_game_payload = msg
//               .payload_as_join_game_payload()
//               .expect("could not parse join payload");
//             let user = join_game_payload.user();

//             match user {
//               Some(user) => handle_join(user.username().unwrap(), remote_addr, players),
//               None => vec![0u8; 0],
//             }
//           }
//           ClientMessagePayload::LeaveGamePayload => {
//             let leave_game_payload = msg
//               .payload_as_leave_game_payload()
//               .expect("could not parse leave payload");
//             let user = leave_game_payload.user();

//             match user {
//               Some(user) => handle_leave(user.username().unwrap(), remote_addr, players),
//               None => vec![0u8; 0],
//             }
//           }
//           ClientMessagePayload::ChatPayload => {
//             let chat_payload = msg
//               .payload_as_chat_payload()
//               .expect("could not parse chat payload");
//             let user = chat_payload.user();
//             let message = chat_payload.message();

//             match (user, message) {
//               _ => vec![0u8; 0],
//             }
//           }
//           ClientMessagePayload::QueryStatePayload => {
//             let query_payload = msg
//               .payload_as_query_state_payload()
//               .expect("could not parse query payload");
//             let user = query_payload.user();

//             match user {
//               Some(user) => handle_query_state(user.username().unwrap(), remote_addr, players),
//               None => vec![0u8; 0],
//             }
//           }
//           ClientMessagePayload::InputPayload => {
//             let input_payload = msg
//               .payload_as_input_payload()
//               .expect("could not parse input payload");
//             let user = input_payload.user();

//             match user {
//               Some(user) => handle_input(user.username().unwrap(), remote_addr, players),
//               None => vec![0u8; 0],
//             }
//           }
//           _ => vec![0u8; 0],
//         };
//       } else {
//         log::warn!("could not parse binary client message");
//       }
//     }
//     MessageType::Text => {
//       log::warn!("received string message: {:#?}", message_buf);
//     }
//   }

//   return vec![0u8; 0];
// }

// fn handle_join(name: &str, remote_addr: SocketAddr, players: Arc<RwLock<PlayerMap>>) -> Vec<u8> {
//   let mut builder = flatbuffers::FlatBufferBuilder::new();
//   if players.read().unwrap().contains_key(&remote_addr) {
//     log::warn!("{} tried to join twice", name);
//     return vec![0u8; 0];
//   }
//   log::info!("{} joined", name);
//   players
//     .write()
//     .unwrap()
//     .insert(remote_addr, name.to_string());

//   let payload = JoinGameResponsePayload::create(
//     &mut builder,
//     &JoinGameResponsePayloadArgs {
//       game: None,
//       code: ResponseCode::OK,
//       ..Default::default()
//     },
//   );

//   let res = ServerMessage::create(
//     &mut builder,
//     &ServerMessageArgs {
//       timestamp: 0,
//       payload_type: ServerMessagePayload::JoinGameResponsePayload,
//       payload: Some(payload.as_union_value()),
//     },
//   );

//   builder.finish(res, None);

//   return builder.finished_data().to_vec();
// }

// #[allow(unused_variables)]
// fn handle_query_state(
//   name: &str,
//   remote_addr: SocketAddr,
//   players: Arc<RwLock<PlayerMap>>,
// ) -> Vec<u8> {
//   vec![0u8; 0]
// }

// #[allow(unused_variables)]
// fn handle_input(name: &str, remote_addr: SocketAddr, players: Arc<RwLock<PlayerMap>>) -> Vec<u8> {
//   vec![0u8; 0]
// }

// fn handle_leave(name: &str, remote_addr: SocketAddr, players: Arc<RwLock<PlayerMap>>) -> Vec<u8> {
//   let mut builder = flatbuffers::FlatBufferBuilder::new();
//   if !players.read().unwrap().contains_key(&remote_addr) {
//     log::warn!("{} tried to leave, but is not playing.", name);
//     return vec![0u8; 0];
//   }
//   log::info!("{} left", name);
//   players.write().unwrap().remove(&remote_addr);

//   let payload = LeaveGameResponsePayload::create(
//     &mut builder,
//     &LeaveGameResponsePayloadArgs {
//       game: None,
//       code: ResponseCode::OK,
//     },
//   );

//   let res = ServerMessage::create(
//     &mut builder,
//     &ServerMessageArgs {
//       timestamp: 0,
//       payload_type: ServerMessagePayload::LeaveGameResponsePayload,
//       payload: Some(payload.as_union_value()),
//     },
//   );

//   builder.finish(res, None);

//   return builder.finished_data().to_vec();
// }
