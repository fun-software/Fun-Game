// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod game {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_GAME_PHASE: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_GAME_PHASE: i8 = 3;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_GAME_PHASE: [GamePhase; 4] = [
  GamePhase::Lobby,
  GamePhase::InGame,
  GamePhase::Post,
  GamePhase::Ended,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct GamePhase(pub i8);
#[allow(non_upper_case_globals)]
impl GamePhase {
  pub const Lobby: Self = Self(0);
  pub const InGame: Self = Self(1);
  pub const Post: Self = Self(2);
  pub const Ended: Self = Self(3);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 3;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Lobby,
    Self::InGame,
    Self::Post,
    Self::Ended,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Lobby => Some("Lobby"),
      Self::InGame => Some("InGame"),
      Self::Post => Some("Post"),
      Self::Ended => Some("Ended"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for GamePhase {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for GamePhase {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for GamePhase {
    type Output = GamePhase;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for GamePhase {
  type Scalar = i8;
  #[inline]
  fn to_little_endian(self) -> i8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i8) -> Self {
    let b = i8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for GamePhase {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for GamePhase {}
pub enum PlayerRolesOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct PlayerRoles<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for PlayerRoles<'a> {
  type Inner = PlayerRoles<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> PlayerRoles<'a> {
  pub const VT_RED: flatbuffers::VOffsetT = 4;
  pub const VT_BLUE: flatbuffers::VOffsetT = 6;
  pub const VT_GREEN: flatbuffers::VOffsetT = 8;
  pub const VT_YELLOW: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    PlayerRoles { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PlayerRolesArgs
  ) -> flatbuffers::WIPOffset<PlayerRoles<'bldr>> {
    let mut builder = PlayerRolesBuilder::new(_fbb);
    builder.add_yellow(args.yellow);
    builder.add_green(args.green);
    builder.add_blue(args.blue);
    builder.add_red(args.red);
    builder.finish()
  }

  pub fn unpack(&self) -> PlayerRolesT {
    let red = self.red();
    let blue = self.blue();
    let green = self.green();
    let yellow = self.yellow();
    PlayerRolesT {
      red,
      blue,
      green,
      yellow,
    }
  }

  #[inline]
  pub fn red(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(PlayerRoles::VT_RED, Some(0)).unwrap()}
  }
  #[inline]
  pub fn blue(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(PlayerRoles::VT_BLUE, Some(0)).unwrap()}
  }
  #[inline]
  pub fn green(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(PlayerRoles::VT_GREEN, Some(0)).unwrap()}
  }
  #[inline]
  pub fn yellow(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(PlayerRoles::VT_YELLOW, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for PlayerRoles<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("red", Self::VT_RED, false)?
     .visit_field::<u64>("blue", Self::VT_BLUE, false)?
     .visit_field::<u64>("green", Self::VT_GREEN, false)?
     .visit_field::<u64>("yellow", Self::VT_YELLOW, false)?
     .finish();
    Ok(())
  }
}
pub struct PlayerRolesArgs {
    pub red: u64,
    pub blue: u64,
    pub green: u64,
    pub yellow: u64,
}
impl<'a> Default for PlayerRolesArgs {
  #[inline]
  fn default() -> Self {
    PlayerRolesArgs {
      red: 0,
      blue: 0,
      green: 0,
      yellow: 0,
    }
  }
}

pub struct PlayerRolesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PlayerRolesBuilder<'a, 'b> {
  #[inline]
  pub fn add_red(&mut self, red: u64) {
    self.fbb_.push_slot::<u64>(PlayerRoles::VT_RED, red, 0);
  }
  #[inline]
  pub fn add_blue(&mut self, blue: u64) {
    self.fbb_.push_slot::<u64>(PlayerRoles::VT_BLUE, blue, 0);
  }
  #[inline]
  pub fn add_green(&mut self, green: u64) {
    self.fbb_.push_slot::<u64>(PlayerRoles::VT_GREEN, green, 0);
  }
  #[inline]
  pub fn add_yellow(&mut self, yellow: u64) {
    self.fbb_.push_slot::<u64>(PlayerRoles::VT_YELLOW, yellow, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PlayerRolesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PlayerRolesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<PlayerRoles<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for PlayerRoles<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("PlayerRoles");
      ds.field("red", &self.red());
      ds.field("blue", &self.blue());
      ds.field("green", &self.green());
      ds.field("yellow", &self.yellow());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerRolesT {
  pub red: u64,
  pub blue: u64,
  pub green: u64,
  pub yellow: u64,
}
impl Default for PlayerRolesT {
  fn default() -> Self {
    Self {
      red: 0,
      blue: 0,
      green: 0,
      yellow: 0,
    }
  }
}
impl PlayerRolesT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<PlayerRoles<'b>> {
    let red = self.red;
    let blue = self.blue;
    let green = self.green;
    let yellow = self.yellow;
    PlayerRoles::create(_fbb, &PlayerRolesArgs{
      red,
      blue,
      green,
      yellow,
    })
  }
}
pub enum GameOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Game<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Game<'a> {
  type Inner = Game<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Game<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_PHASE: flatbuffers::VOffsetT = 6;
  pub const VT_PLAYERS: flatbuffers::VOffsetT = 8;
  pub const VT_STARTTIME: flatbuffers::VOffsetT = 10;
  pub const VT_ENDTIME: flatbuffers::VOffsetT = 12;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Game { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args GameArgs<'args>
  ) -> flatbuffers::WIPOffset<Game<'bldr>> {
    let mut builder = GameBuilder::new(_fbb);
    builder.add_endtime(args.endtime);
    builder.add_starttime(args.starttime);
    builder.add_id(args.id);
    if let Some(x) = args.players { builder.add_players(x); }
    builder.add_phase(args.phase);
    builder.finish()
  }

  pub fn unpack(&self) -> GameT {
    let id = self.id();
    let phase = self.phase();
    let players = self.players().map(|x| {
      Box::new(x.unpack())
    });
    let starttime = self.starttime();
    let endtime = self.endtime();
    GameT {
      id,
      phase,
      players,
      starttime,
      endtime,
    }
  }

  #[inline]
  pub fn id(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Game::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn phase(&self) -> GamePhase {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<GamePhase>(Game::VT_PHASE, Some(GamePhase::Lobby)).unwrap()}
  }
  #[inline]
  pub fn players(&self) -> Option<PlayerRoles<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<PlayerRoles>>(Game::VT_PLAYERS, None)}
  }
  #[inline]
  pub fn starttime(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Game::VT_STARTTIME, Some(0)).unwrap()}
  }
  #[inline]
  pub fn endtime(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Game::VT_ENDTIME, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Game<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("id", Self::VT_ID, false)?
     .visit_field::<GamePhase>("phase", Self::VT_PHASE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<PlayerRoles>>("players", Self::VT_PLAYERS, false)?
     .visit_field::<u64>("starttime", Self::VT_STARTTIME, false)?
     .visit_field::<u64>("endtime", Self::VT_ENDTIME, false)?
     .finish();
    Ok(())
  }
}
pub struct GameArgs<'a> {
    pub id: u64,
    pub phase: GamePhase,
    pub players: Option<flatbuffers::WIPOffset<PlayerRoles<'a>>>,
    pub starttime: u64,
    pub endtime: u64,
}
impl<'a> Default for GameArgs<'a> {
  #[inline]
  fn default() -> Self {
    GameArgs {
      id: 0,
      phase: GamePhase::Lobby,
      players: None,
      starttime: 0,
      endtime: 0,
    }
  }
}

pub struct GameBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> GameBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: u64) {
    self.fbb_.push_slot::<u64>(Game::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_phase(&mut self, phase: GamePhase) {
    self.fbb_.push_slot::<GamePhase>(Game::VT_PHASE, phase, GamePhase::Lobby);
  }
  #[inline]
  pub fn add_players(&mut self, players: flatbuffers::WIPOffset<PlayerRoles<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<PlayerRoles>>(Game::VT_PLAYERS, players);
  }
  #[inline]
  pub fn add_starttime(&mut self, starttime: u64) {
    self.fbb_.push_slot::<u64>(Game::VT_STARTTIME, starttime, 0);
  }
  #[inline]
  pub fn add_endtime(&mut self, endtime: u64) {
    self.fbb_.push_slot::<u64>(Game::VT_ENDTIME, endtime, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> GameBuilder<'a, 'b> {
    let start = _fbb.start_table();
    GameBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Game<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Game<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Game");
      ds.field("id", &self.id());
      ds.field("phase", &self.phase());
      ds.field("players", &self.players());
      ds.field("starttime", &self.starttime());
      ds.field("endtime", &self.endtime());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct GameT {
  pub id: u64,
  pub phase: GamePhase,
  pub players: Option<Box<PlayerRolesT>>,
  pub starttime: u64,
  pub endtime: u64,
}
impl Default for GameT {
  fn default() -> Self {
    Self {
      id: 0,
      phase: GamePhase::Lobby,
      players: None,
      starttime: 0,
      endtime: 0,
    }
  }
}
impl GameT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Game<'b>> {
    let id = self.id;
    let phase = self.phase;
    let players = self.players.as_ref().map(|x|{
      x.pack(_fbb)
    });
    let starttime = self.starttime;
    let endtime = self.endtime;
    Game::create(_fbb, &GameArgs{
      id,
      phase,
      players,
      starttime,
      endtime,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Game`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_game_unchecked`.
pub fn root_as_game(buf: &[u8]) -> Result<Game, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Game>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Game` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_game_unchecked`.
pub fn size_prefixed_root_as_game(buf: &[u8]) -> Result<Game, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Game>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Game` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_game_unchecked`.
pub fn root_as_game_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Game<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Game<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Game` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_game_unchecked`.
pub fn size_prefixed_root_as_game_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Game<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Game<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Game and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Game`.
pub unsafe fn root_as_game_unchecked(buf: &[u8]) -> Game {
  flatbuffers::root_unchecked::<Game>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Game and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Game`.
pub unsafe fn size_prefixed_root_as_game_unchecked(buf: &[u8]) -> Game {
  flatbuffers::size_prefixed_root_unchecked::<Game>(buf)
}
#[inline]
pub fn finish_game_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Game<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_game_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Game<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Game
