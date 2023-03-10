#![allow(dead_code)]
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use super::ws_service::PeerMap;
use crate::fbs::Game::game::GameT;
use tokio::sync::RwLock;
use webrtc_unreliable::SessionEndpoint;

// map Player IDs to their current game's ID
pub type PlayerCurrentGames = HashMap<String, String>;

// map game IDs to their respective game objects
pub type Games = HashMap<String, GameT>;

// map game IDs to PeerMaps that contain the addresses of
// all players in the lobby
#[derive(Debug, Clone)]
pub struct Lobby {
  pub addr: SocketAddr,
  pub peers: PeerMap,
}

pub type Lobbies = HashMap<String, Lobby>;

// map game IDs to WebRTC server addresses
pub type WebRTCSessions = HashMap<String, SessionEndpoint>;

pub type AsyncState = Arc<RwLock<InnerState>>;

#[derive(Clone)]
pub struct InnerState {
  pub games: Games,
  pub lobbies: Lobbies,
  pub player_games: PlayerCurrentGames,
  pub web_rtc_sessions: WebRTCSessions,
}

impl InnerState {
  pub fn new() -> Self {
    Self {
      games: Games::new(),
      lobbies: Lobbies::new(),
      player_games: PlayerCurrentGames::new(),
      web_rtc_sessions: WebRTCSessions::new(),
    }
  }

  pub fn player_in_game(&self, player_id: &str) -> Option<String> {
    return self.player_games.get(player_id).cloned();
  }

  pub fn player_count(&self, game_id: String) -> usize {
    return self
      .games
      .get(&game_id)
      .unwrap()
      .players
      .as_ref()
      .unwrap()
      .len();
  }
}

pub trait State {
  fn new() -> Self;
}

impl State for AsyncState {
  fn new() -> Self {
    Arc::new(RwLock::new(InnerState::new()))
  }
}
