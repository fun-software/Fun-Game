use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::sync::RwLock;

use crate::fbs::Game::game::Game;

// for testing/POC only, will need to map users to
// permanent identifiers (UUIDs, likely) once DB is up
type PlayerMap = HashMap<SocketAddr, String>;
pub type Players = Arc<RwLock<PlayerMap>>;

type GameMap<'a> = HashMap<u64, Game<'a>>;
pub type Games<'a> = Arc<RwLock<GameMap<'a>>>;

pub struct State<'a> {
  pub players: Players,
  pub games: Games<'a>,
}

pub fn new_state<'a>() -> State<'a> {
  State {
    players: Arc::new(RwLock::new(HashMap::new())),
    games: Arc::new(RwLock::new(HashMap::new())),
  }
}
