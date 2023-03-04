use std::{
  collections::HashMap,
  net::SocketAddr,
  sync::{Arc, RwLock},
};

use crate::fbs::Game::game::GameT;

use super::ws_service::PeerMap;

// for testing/POC only, will need to map users to
// permanent identifiers (UUIDs, likely) once DB is up
// type PlayerMap = HashMap<SocketAddr, String>;
// pub type Players = Arc<RwLock<PlayerMap>>;

// map Player IDs to their current game's ID
type PlayerGameMap = HashMap<String, String>;
pub type PlayerGames = Arc<RwLock<PlayerGameMap>>;

type GameMap = HashMap<String, GameT>;
pub type Games = Arc<RwLock<GameMap>>;

// map game IDs to PeerMaps that contain the addresses of
// all players in the lobby
#[derive(Debug)]
pub struct Lobby {
  pub addr: SocketAddr,
  pub peers: PeerMap,
}
type LobbyMap = HashMap<String, Lobby>;
pub type Lobbies = Arc<RwLock<LobbyMap>>;

pub type ArcState = Arc<RwLock<State>>;
#[derive(Clone)]
pub struct State {
  pub games: Games,
  pub lobbies: Lobbies,
  pub player_games: PlayerGames,
}

pub fn new_state() -> ArcState {
  Arc::new(RwLock::new(State {
    games: Arc::new(RwLock::new(HashMap::new())),
    lobbies: Arc::new(RwLock::new(HashMap::new())),
    player_games: Arc::new(RwLock::new(HashMap::new())),
  }))
}
