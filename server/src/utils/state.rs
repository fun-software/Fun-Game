use std::{
  collections::HashMap,
  net::SocketAddr,
  sync::{Arc, RwLock},
};

use crate::fbs::Game::game::Game;

// for testing/POC only, will need to map users to
// permanent identifiers (UUIDs, likely) once DB is up
type PlayerMap = HashMap<SocketAddr, String>;
pub type Players = Arc<RwLock<PlayerMap>>;

type GameMap<'a> = HashMap<u64, Game<'a>>;
pub type Games<'a> = Arc<RwLock<GameMap<'a>>>;

// map lobby IDs to websocket addresses
type LobbyMap = HashMap<u64, String>;
pub type LobbyAddresses = Arc<RwLock<LobbyMap>>;

pub type ArcState<'a> = Arc<RwLock<State<'a>>>;
#[derive(Clone)]
pub struct State<'a> {
  pub players: Players,
  pub games: Games<'a>,
  pub lobby_addresses: LobbyAddresses,
}

pub fn new_state<'a>() -> ArcState<'a> {
  Arc::new(RwLock::new(State {
    players: Arc::new(RwLock::new(HashMap::new())),
    games: Arc::new(RwLock::new(HashMap::new())),
    lobby_addresses: Arc::new(RwLock::new(HashMap::new())),
  }))
}
