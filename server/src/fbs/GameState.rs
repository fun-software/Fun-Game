// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::fbs::Game::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod gamestate {

  use crate::fbs::Game::*;
  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum GameStateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GameState<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GameState<'a> {
  type Inner = GameState<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> GameState<'a> {
  pub const VT_GAME: flatbuffers::VOffsetT = 4;
  pub const VT_PLAYERS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GameState { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args GameStateArgs<'args>
  ) -> flatbuffers::WIPOffset<GameState<'bldr>> {
    let mut builder = GameStateBuilder::new(_fbb);
    if let Some(x) = args.players { builder.add_players(x); }
    if let Some(x) = args.game { builder.add_game(x); }
    builder.finish()
  }

  pub fn unpack(&self) -> GameStateT {
    let game = self.game().map(|x| {
      Box::new(x.unpack())
    });
    let players = self.players().map(|x| {
      Box::new(x.unpack())
    });
    GameStateT {
      game,
      players,
    }
  }

  #[inline]
  pub fn game(&self) -> Option<super::game::Game<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<super::game::Game>>(GameState::VT_GAME, None)}
  }
  #[inline]
  pub fn players(&self) -> Option<super::game::PlayerRoles<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<super::game::PlayerRoles>>(GameState::VT_PLAYERS, None)}
  }
}

impl flatbuffers::Verifiable for GameState<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::game::Game>>("game", Self::VT_GAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::game::PlayerRoles>>("players", Self::VT_PLAYERS, false)?
     .finish();
    Ok(())
  }
}
pub struct GameStateArgs<'a> {
    pub game: Option<flatbuffers::WIPOffset<super::game::Game<'a>>>,
    pub players: Option<flatbuffers::WIPOffset<super::game::PlayerRoles<'a>>>,
}
impl<'a> Default for GameStateArgs<'a> {
  #[inline]
  fn default() -> Self {
    GameStateArgs {
      game: None,
      players: None,
    }
  }
}

pub struct GameStateBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> GameStateBuilder<'a, 'b> {
  #[inline]
  pub fn add_game(&mut self, game: flatbuffers::WIPOffset<super::game::Game<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::game::Game>>(GameState::VT_GAME, game);
  }
  #[inline]
  pub fn add_players(&mut self, players: flatbuffers::WIPOffset<super::game::PlayerRoles<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::game::PlayerRoles>>(GameState::VT_PLAYERS, players);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> GameStateBuilder<'a, 'b> {
    let start = _fbb.start_table();
    GameStateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GameState<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GameState<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GameState");
      ds.field("game", &self.game());
      ds.field("players", &self.players());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct GameStateT {
  pub game: Option<Box<super::game::GameT>>,
  pub players: Option<Box<super::game::PlayerRolesT>>,
}
impl Default for GameStateT {
  fn default() -> Self {
    Self {
      game: None,
      players: None,
    }
  }
}
impl GameStateT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<GameState<'b>> {
    let game = self.game.as_ref().map(|x|{
      x.pack(_fbb)
    });
    let players = self.players.as_ref().map(|x|{
      x.pack(_fbb)
    });
    GameState::create(_fbb, &GameStateArgs{
      game,
      players,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `GameState`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_game_state_unchecked`.
pub fn root_as_game_state(buf: &[u8]) -> Result<GameState, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<GameState>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `GameState` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_game_state_unchecked`.
pub fn size_prefixed_root_as_game_state(buf: &[u8]) -> Result<GameState, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<GameState>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `GameState` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_game_state_unchecked`.
pub fn root_as_game_state_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<GameState<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<GameState<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `GameState` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_game_state_unchecked`.
pub fn size_prefixed_root_as_game_state_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<GameState<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<GameState<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a GameState and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `GameState`.
pub unsafe fn root_as_game_state_unchecked(buf: &[u8]) -> GameState {
  flatbuffers::root_unchecked::<GameState>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed GameState and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `GameState`.
pub unsafe fn size_prefixed_root_as_game_state_unchecked(buf: &[u8]) -> GameState {
  flatbuffers::size_prefixed_root_unchecked::<GameState>(buf)
}
#[inline]
pub fn finish_game_state_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<GameState<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_game_state_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<GameState<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Gamestate

