use std::{
  collections::HashMap,
  net::SocketAddr,
  sync::{Arc, RwLock},
};

use crate::fbs::Game::game::GameT;

use super::ws_service::PeerMap;

// for testing/POC only, will need to map users to
// permanent identifiers (UUIDs, likely) once DB is up
type PlayerMap = HashMap<SocketAddr, String>;
pub type Players = Arc<RwLock<PlayerMap>>;

type GameMap = HashMap<u64, GameT>;
pub type Games = Arc<RwLock<GameMap>>;

// map game IDs to PeerMaps that contain the addresses of
// all players in the lobby
#[derive(Debug)]
pub struct Lobby {
  pub addr: SocketAddr,
  pub peers: PeerMap,
}
type LobbyMap = HashMap<u64, Lobby>;
pub type Lobbies = Arc<RwLock<LobbyMap>>;

pub type ArcState = Arc<RwLock<State>>;
#[derive(Clone)]
pub struct State {
  pub players: Players,
  pub games: Games,
  pub lobbies: Lobbies,
}

pub fn new_state() -> ArcState {
  Arc::new(RwLock::new(State {
    players: Arc::new(RwLock::new(HashMap::new())),
    games: Arc::new(RwLock::new(HashMap::new())),
    lobbies: Arc::new(RwLock::new(HashMap::new())),
  }))
}
