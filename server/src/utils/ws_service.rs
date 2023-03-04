use std::{
  collections::HashMap,
  net::SocketAddr,
  sync::{Arc, RwLock},
};

use futures::{pin_mut, StreamExt, TryStreamExt};
use futures_channel::mpsc::{self, UnboundedSender};
use futures_util::future;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

use crate::{
  fbs::Chat::chat::{self, ChatMessageT, ChatSource},
  utils::state::Lobby,
};

use super::state::ArcState;

type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<RwLock<HashMap<SocketAddr, Tx>>>;

pub async fn ws_service(listener: TcpListener, state: ArcState, game_id: String) {
  let peers = PeerMap::new(RwLock::new(HashMap::new()));

  if state
    .read()
    .unwrap()
    .lobbies
    .read()
    .unwrap()
    .contains_key(&game_id)
  {
    log::error!(
      "Lobby for game {} already exists, not creating new one",
      game_id
    );
    return;
  }

  // add the game to the state
  state.read().unwrap().lobbies.write().unwrap().insert(
    game_id,
    Lobby {
      addr: listener.local_addr().unwrap(),
      peers: peers.clone(),
    },
  );

  log::debug!("Listening on {}", listener.local_addr().unwrap());

  while let Ok((stream, addr)) = listener.accept().await {
    let peers = peers.clone();
    log::debug!("New connection from {}", addr);
    tokio::spawn(async move { handle_connection(stream, addr, peers).await });
  }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, peers: PeerMap) {
  let ws_stream = tokio_tungstenite::accept_async(stream)
    .await
    .expect("Websocket handshake failed");

  let (channel_out, channel_in) = mpsc::unbounded::<Message>();
  peers.write().unwrap().insert(addr, channel_out);

  let (outgoing, incoming) = ws_stream.split();

  let incoming_msg = incoming.try_for_each(|raw_msg| {
    if raw_msg.is_binary() {
      let msg_payload = deserialize_message(raw_msg.clone().into_data());
      let msg = msg_payload.message.unwrap();

      let source_details = match msg_payload.source {
        ChatSource::System => " system",
        _ => " chat",
      };

      log::info!(
        "Received{} message from {}: {:?}",
        source_details,
        addr,
        msg
      );

      let peers = peers.read().unwrap();

      if msg_is_valid(msg) {
        peers.iter().for_each(|(_, peer_tx)| {
          peer_tx.unbounded_send(raw_msg.clone()).unwrap();
        });
      }
    }

    future::ok(())
  });

  let rcv_from_channel = channel_in.map(Ok).forward(outgoing);
  pin_mut!(incoming_msg, rcv_from_channel);

  match future::select(incoming_msg, rcv_from_channel).await {
    future::Either::Left((incoming_msg, _)) => {
      if let Err(e) = incoming_msg {
        log::error!("Error while receiving message from {}: {}", addr, e);
      }
    }
    future::Either::Right((rcv_from_channel, _)) => {
      if let Err(e) = rcv_from_channel {
        log::error!("Error while sending message to {}: {}", addr, e);
      }
    }
  }

  log::info!("{} disconnected", addr);
  peers.write().unwrap().remove(&addr);
}

fn msg_is_valid(msg: String) -> bool {
  msg.len() > 0 && msg.len() <= 256
}

fn deserialize_message(msg: Vec<u8>) -> ChatMessageT {
  let chat = chat::root_as_chat_message(&msg.as_slice());

  match chat {
    Ok(chat) => chat.unpack(),
    _ => panic!("failed to parse message"),
  }
}
