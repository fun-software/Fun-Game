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

use crate::utils::state::Lobby;

use super::state::ArcState;

type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<RwLock<HashMap<SocketAddr, Tx>>>;

pub async fn ws_service(listener: TcpListener, state: ArcState, game_id: u64) {
  let peers = PeerMap::new(RwLock::new(HashMap::new()));

  // add the game to the state
  state.write().unwrap().lobbies.write().unwrap().insert(
    game_id,
    Lobby {
      addr: listener.local_addr().unwrap(),
      peers: peers.clone(),
    },
  );

  log::info!("lobbies: {:#?}", state.read().unwrap().lobbies);

  log::info!("Listening on {}", listener.local_addr().unwrap());

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

  let incoming_msg = incoming.try_for_each(|msg| {
    log::debug!("Received message from {}: {:?}", addr, msg);
    let peers = peers.read().unwrap();

    peers.iter().for_each(|(peer_addr, peer_tx)| {
      if peer_addr != &addr {
        peer_tx.unbounded_send(msg.clone()).unwrap();
      }
    });

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
