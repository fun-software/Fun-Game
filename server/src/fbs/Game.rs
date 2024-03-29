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
    args: &'args PlayerRolesArgs<'args>
  ) -> flatbuffers::WIPOffset<PlayerRoles<'bldr>> {
    let mut builder = PlayerRolesBuilder::new(_fbb);
    if let Some(x) = args.yellow { builder.add_yellow(x); }
    if let Some(x) = args.green { builder.add_green(x); }
    if let Some(x) = args.blue { builder.add_blue(x); }
    if let Some(x) = args.red { builder.add_red(x); }
    builder.finish()
  }

  pub fn unpack(&self) -> PlayerRolesT {
    let red = self.red().map(|x| {
      x.to_string()
    });
    let blue = self.blue().map(|x| {
      x.to_string()
    });
    let green = self.green().map(|x| {
      x.to_string()
    });
    let yellow = self.yellow().map(|x| {
      x.to_string()
    });
    PlayerRolesT {
      red,
      blue,
      green,
      yellow,
    }
  }

  #[inline]
  pub fn red(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PlayerRoles::VT_RED, None)}
  }
  #[inline]
  pub fn blue(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PlayerRoles::VT_BLUE, None)}
  }
  #[inline]
  pub fn green(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PlayerRoles::VT_GREEN, None)}
  }
  #[inline]
  pub fn yellow(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PlayerRoles::VT_YELLOW, None)}
  }
}

impl flatbuffers::Verifiable for PlayerRoles<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("red", Self::VT_RED, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("blue", Self::VT_BLUE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("green", Self::VT_GREEN, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("yellow", Self::VT_YELLOW, false)?
     .finish();
    Ok(())
  }
}
pub struct PlayerRolesArgs<'a> {
    pub red: Option<flatbuffers::WIPOffset<&'a str>>,
    pub blue: Option<flatbuffers::WIPOffset<&'a str>>,
    pub green: Option<flatbuffers::WIPOffset<&'a str>>,
    pub yellow: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for PlayerRolesArgs<'a> {
  #[inline]
  fn default() -> Self {
    PlayerRolesArgs {
      red: None,
      blue: None,
      green: None,
      yellow: None,
    }
  }
}

pub struct PlayerRolesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PlayerRolesBuilder<'a, 'b> {
  #[inline]
  pub fn add_red(&mut self, red: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PlayerRoles::VT_RED, red);
  }
  #[inline]
  pub fn add_blue(&mut self, blue: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PlayerRoles::VT_BLUE, blue);
  }
  #[inline]
  pub fn add_green(&mut self, green: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PlayerRoles::VT_GREEN, green);
  }
  #[inline]
  pub fn add_yellow(&mut self, yellow: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PlayerRoles::VT_YELLOW, yellow);
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
  pub red: Option<String>,
  pub blue: Option<String>,
  pub green: Option<String>,
  pub yellow: Option<String>,
}
impl Default for PlayerRolesT {
  fn default() -> Self {
    Self {
      red: None,
      blue: None,
      green: None,
      yellow: None,
    }
  }
}
impl PlayerRolesT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<PlayerRoles<'b>> {
    let red = self.red.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let blue = self.blue.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let green = self.green.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let yellow = self.yellow.as_ref().map(|x|{
      _fbb.create_string(x)
    });
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
  pub const VT_PLAYER_ROLES: flatbuffers::VOffsetT = 10;
  pub const VT_START_TIME: flatbuffers::VOffsetT = 12;
  pub const VT_END_TIME: flatbuffers::VOffsetT = 14;

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
    builder.add_end_time(args.end_time);
    builder.add_start_time(args.start_time);
    if let Some(x) = args.player_roles { builder.add_player_roles(x); }
    if let Some(x) = args.players { builder.add_players(x); }
    if let Some(x) = args.id { builder.add_id(x); }
    builder.add_phase(args.phase);
    builder.finish()
  }

  pub fn unpack(&self) -> GameT {
    let id = self.id().map(|x| {
      x.to_string()
    });
    let phase = self.phase();
    let players = self.players().map(|x| {
      x.iter().map(|s| s.to_string()).collect()
    });
    let player_roles = self.player_roles().map(|x| {
      Box::new(x.unpack())
    });
    let start_time = self.start_time();
    let end_time = self.end_time();
    GameT {
      id,
      phase,
      players,
      player_roles,
      start_time,
      end_time,
    }
  }

  #[inline]
  pub fn id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Game::VT_ID, None)}
  }
  #[inline]
  pub fn phase(&self) -> GamePhase {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<GamePhase>(Game::VT_PHASE, Some(GamePhase::Lobby)).unwrap()}
  }
  #[inline]
  pub fn players(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(Game::VT_PLAYERS, None)}
  }
  #[inline]
  pub fn player_roles(&self) -> Option<PlayerRoles<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<PlayerRoles>>(Game::VT_PLAYER_ROLES, None)}
  }
  #[inline]
  pub fn start_time(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Game::VT_START_TIME, Some(0)).unwrap()}
  }
  #[inline]
  pub fn end_time(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Game::VT_END_TIME, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Game<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("id", Self::VT_ID, false)?
     .visit_field::<GamePhase>("phase", Self::VT_PHASE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("players", Self::VT_PLAYERS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<PlayerRoles>>("player_roles", Self::VT_PLAYER_ROLES, false)?
     .visit_field::<u64>("start_time", Self::VT_START_TIME, false)?
     .visit_field::<u64>("end_time", Self::VT_END_TIME, false)?
     .finish();
    Ok(())
  }
}
pub struct GameArgs<'a> {
    pub id: Option<flatbuffers::WIPOffset<&'a str>>,
    pub phase: GamePhase,
    pub players: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
    pub player_roles: Option<flatbuffers::WIPOffset<PlayerRoles<'a>>>,
    pub start_time: u64,
    pub end_time: u64,
}
impl<'a> Default for GameArgs<'a> {
  #[inline]
  fn default() -> Self {
    GameArgs {
      id: None,
      phase: GamePhase::Lobby,
      players: None,
      player_roles: None,
      start_time: 0,
      end_time: 0,
    }
  }
}

pub struct GameBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> GameBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Game::VT_ID, id);
  }
  #[inline]
  pub fn add_phase(&mut self, phase: GamePhase) {
    self.fbb_.push_slot::<GamePhase>(Game::VT_PHASE, phase, GamePhase::Lobby);
  }
  #[inline]
  pub fn add_players(&mut self, players: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Game::VT_PLAYERS, players);
  }
  #[inline]
  pub fn add_player_roles(&mut self, player_roles: flatbuffers::WIPOffset<PlayerRoles<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<PlayerRoles>>(Game::VT_PLAYER_ROLES, player_roles);
  }
  #[inline]
  pub fn add_start_time(&mut self, start_time: u64) {
    self.fbb_.push_slot::<u64>(Game::VT_START_TIME, start_time, 0);
  }
  #[inline]
  pub fn add_end_time(&mut self, end_time: u64) {
    self.fbb_.push_slot::<u64>(Game::VT_END_TIME, end_time, 0);
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
      ds.field("player_roles", &self.player_roles());
      ds.field("start_time", &self.start_time());
      ds.field("end_time", &self.end_time());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct GameT {
  pub id: Option<String>,
  pub phase: GamePhase,
  pub players: Option<Vec<String>>,
  pub player_roles: Option<Box<PlayerRolesT>>,
  pub start_time: u64,
  pub end_time: u64,
}
impl Default for GameT {
  fn default() -> Self {
    Self {
      id: None,
      phase: GamePhase::Lobby,
      players: None,
      player_roles: None,
      start_time: 0,
      end_time: 0,
    }
  }
}
impl GameT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Game<'b>> {
    let id = self.id.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let phase = self.phase;
    let players = self.players.as_ref().map(|x|{
      let w: Vec<_> = x.iter().map(|s| _fbb.create_string(s)).collect();_fbb.create_vector(&w)
    });
    let player_roles = self.player_roles.as_ref().map(|x|{
      x.pack(_fbb)
    });
    let start_time = self.start_time;
    let end_time = self.end_time;
    Game::create(_fbb, &GameArgs{
      id,
      phase,
      players,
      player_roles,
      start_time,
      end_time,
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

