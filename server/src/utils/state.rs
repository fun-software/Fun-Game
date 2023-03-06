use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use super::ws_service::PeerMap;
use crate::fbs::Game::game::GameT;
use tokio::sync::RwLock;
use webrtc_unreliable::SessionEndpoint;

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

// map game IDs to WebRTC server addresses
type WebRTCSessionMap = HashMap<String, SessionEndpoint>;
pub type WebRTCSessions = Arc<RwLock<WebRTCSessionMap>>;

pub type ArcState = Arc<RwLock<State>>;
#[derive(Clone)]
pub struct State {
  pub games: Games,
  pub lobbies: Lobbies,
  pub player_games: PlayerGames,
  pub web_rtc_sessions: WebRTCSessions,
}

pub fn new_state() -> ArcState {
  Arc::new(RwLock::new(State {
    games: Arc::new(RwLock::new(HashMap::new())),
    lobbies: Arc::new(RwLock::new(HashMap::new())),
    player_games: Arc::new(RwLock::new(HashMap::new())),
    web_rtc_sessions: Arc::new(RwLock::new(HashMap::new())),
  }))
}
