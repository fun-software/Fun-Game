// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::fbs::Math::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod player {

  use crate::fbs::Math::*;
  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ROLE: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ROLE: i8 = 4;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ROLE: [Role; 5] = [
  Role::Blue,
  Role::Default,
  Role::Green,
  Role::Red,
  Role::Yellow,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Role(pub i8);
#[allow(non_upper_case_globals)]
impl Role {
  pub const Blue: Self = Self(0);
  pub const Default: Self = Self(1);
  pub const Green: Self = Self(2);
  pub const Red: Self = Self(3);
  pub const Yellow: Self = Self(4);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 4;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Blue,
    Self::Default,
    Self::Green,
    Self::Red,
    Self::Yellow,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Blue => Some("Blue"),
      Self::Default => Some("Default"),
      Self::Green => Some("Green"),
      Self::Red => Some("Red"),
      Self::Yellow => Some("Yellow"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for Role {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Role {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Role {
    type Output = Role;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Role {
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

impl<'a> flatbuffers::Verifiable for Role {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Role {}
pub enum PlayerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Player<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Player<'a> {
  type Inner = Player<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Player<'a> {
  pub const VT_HP: flatbuffers::VOffsetT = 4;
  pub const VT_ID: flatbuffers::VOffsetT = 6;
  pub const VT_LOOK_DIRECTION: flatbuffers::VOffsetT = 8;
  pub const VT_POSITION: flatbuffers::VOffsetT = 10;
  pub const VT_SPEED: flatbuffers::VOffsetT = 12;
  pub const VT_SPRINTING: flatbuffers::VOffsetT = 14;
  pub const VT_SNEAKING: flatbuffers::VOffsetT = 16;
  pub const VT_ROLE: flatbuffers::VOffsetT = 18;
  pub const VT_VELOCITY: flatbuffers::VOffsetT = 20;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Player { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PlayerArgs<'args>
  ) -> flatbuffers::WIPOffset<Player<'bldr>> {
    let mut builder = PlayerBuilder::new(_fbb);
    if let Some(x) = args.velocity { builder.add_velocity(x); }
    if let Some(x) = args.position { builder.add_position(x); }
    if let Some(x) = args.look_direction { builder.add_look_direction(x); }
    if let Some(x) = args.id { builder.add_id(x); }
    builder.add_role(args.role);
    builder.add_sneaking(args.sneaking);
    builder.add_sprinting(args.sprinting);
    builder.add_speed(args.speed);
    builder.add_hp(args.hp);
    builder.finish()
  }

  pub fn unpack(&self) -> PlayerT {
    let hp = self.hp();
    let id = self.id().map(|x| {
      x.to_string()
    });
    let look_direction = self.look_direction().map(|x| {
      x.unpack()
    });
    let position = self.position().map(|x| {
      x.unpack()
    });
    let speed = self.speed();
    let sprinting = self.sprinting();
    let sneaking = self.sneaking();
    let role = self.role();
    let velocity = self.velocity().map(|x| {
      x.unpack()
    });
    PlayerT {
      hp,
      id,
      look_direction,
      position,
      speed,
      sprinting,
      sneaking,
      role,
      velocity,
    }
  }

  #[inline]
  pub fn hp(&self) -> i8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i8>(Player::VT_HP, Some(100)).unwrap()}
  }
  #[inline]
  pub fn id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Player::VT_ID, None)}
  }
  #[inline]
  pub fn look_direction(&self) -> Option<&'a super::math::Vec3> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::math::Vec3>(Player::VT_LOOK_DIRECTION, None)}
  }
  #[inline]
  pub fn position(&self) -> Option<&'a super::math::Vec3> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::math::Vec3>(Player::VT_POSITION, None)}
  }
  #[inline]
  pub fn speed(&self) -> i8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i8>(Player::VT_SPEED, Some(100)).unwrap()}
  }
  #[inline]
  pub fn sprinting(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(Player::VT_SPRINTING, Some(false)).unwrap()}
  }
  #[inline]
  pub fn sneaking(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(Player::VT_SNEAKING, Some(false)).unwrap()}
  }
  #[inline]
  pub fn role(&self) -> Role {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Role>(Player::VT_ROLE, Some(Role::Blue)).unwrap()}
  }
  #[inline]
  pub fn velocity(&self) -> Option<&'a super::math::Vec3> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::math::Vec3>(Player::VT_VELOCITY, None)}
  }
}

impl flatbuffers::Verifiable for Player<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i8>("hp", Self::VT_HP, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("id", Self::VT_ID, false)?
     .visit_field::<super::math::Vec3>("look_direction", Self::VT_LOOK_DIRECTION, false)?
     .visit_field::<super::math::Vec3>("position", Self::VT_POSITION, false)?
     .visit_field::<i8>("speed", Self::VT_SPEED, false)?
     .visit_field::<bool>("sprinting", Self::VT_SPRINTING, false)?
     .visit_field::<bool>("sneaking", Self::VT_SNEAKING, false)?
     .visit_field::<Role>("role", Self::VT_ROLE, false)?
     .visit_field::<super::math::Vec3>("velocity", Self::VT_VELOCITY, false)?
     .finish();
    Ok(())
  }
}
pub struct PlayerArgs<'a> {
    pub hp: i8,
    pub id: Option<flatbuffers::WIPOffset<&'a str>>,
    pub look_direction: Option<&'a super::math::Vec3>,
    pub position: Option<&'a super::math::Vec3>,
    pub speed: i8,
    pub sprinting: bool,
    pub sneaking: bool,
    pub role: Role,
    pub velocity: Option<&'a super::math::Vec3>,
}
impl<'a> Default for PlayerArgs<'a> {
  #[inline]
  fn default() -> Self {
    PlayerArgs {
      hp: 100,
      id: None,
      look_direction: None,
      position: None,
      speed: 100,
      sprinting: false,
      sneaking: false,
      role: Role::Blue,
      velocity: None,
    }
  }
}

pub struct PlayerBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PlayerBuilder<'a, 'b> {
  #[inline]
  pub fn add_hp(&mut self, hp: i8) {
    self.fbb_.push_slot::<i8>(Player::VT_HP, hp, 100);
  }
  #[inline]
  pub fn add_id(&mut self, id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Player::VT_ID, id);
  }
  #[inline]
  pub fn add_look_direction(&mut self, look_direction: &super::math::Vec3) {
    self.fbb_.push_slot_always::<&super::math::Vec3>(Player::VT_LOOK_DIRECTION, look_direction);
  }
  #[inline]
  pub fn add_position(&mut self, position: &super::math::Vec3) {
    self.fbb_.push_slot_always::<&super::math::Vec3>(Player::VT_POSITION, position);
  }
  #[inline]
  pub fn add_speed(&mut self, speed: i8) {
    self.fbb_.push_slot::<i8>(Player::VT_SPEED, speed, 100);
  }
  #[inline]
  pub fn add_sprinting(&mut self, sprinting: bool) {
    self.fbb_.push_slot::<bool>(Player::VT_SPRINTING, sprinting, false);
  }
  #[inline]
  pub fn add_sneaking(&mut self, sneaking: bool) {
    self.fbb_.push_slot::<bool>(Player::VT_SNEAKING, sneaking, false);
  }
  #[inline]
  pub fn add_role(&mut self, role: Role) {
    self.fbb_.push_slot::<Role>(Player::VT_ROLE, role, Role::Blue);
  }
  #[inline]
  pub fn add_velocity(&mut self, velocity: &super::math::Vec3) {
    self.fbb_.push_slot_always::<&super::math::Vec3>(Player::VT_VELOCITY, velocity);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PlayerBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PlayerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Player<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Player<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Player");
      ds.field("hp", &self.hp());
      ds.field("id", &self.id());
      ds.field("look_direction", &self.look_direction());
      ds.field("position", &self.position());
      ds.field("speed", &self.speed());
      ds.field("sprinting", &self.sprinting());
      ds.field("sneaking", &self.sneaking());
      ds.field("role", &self.role());
      ds.field("velocity", &self.velocity());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerT {
  pub hp: i8,
  pub id: Option<String>,
  pub look_direction: Option<super::math::Vec3T>,
  pub position: Option<super::math::Vec3T>,
  pub speed: i8,
  pub sprinting: bool,
  pub sneaking: bool,
  pub role: Role,
  pub velocity: Option<super::math::Vec3T>,
}
impl Default for PlayerT {
  fn default() -> Self {
    Self {
      hp: 100,
      id: None,
      look_direction: None,
      position: None,
      speed: 100,
      sprinting: false,
      sneaking: false,
      role: Role::Blue,
      velocity: None,
    }
  }
}
impl PlayerT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Player<'b>> {
    let hp = self.hp;
    let id = self.id.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let look_direction_tmp = self.look_direction.as_ref().map(|x| x.pack());
    let look_direction = look_direction_tmp.as_ref();
    let position_tmp = self.position.as_ref().map(|x| x.pack());
    let position = position_tmp.as_ref();
    let speed = self.speed;
    let sprinting = self.sprinting;
    let sneaking = self.sneaking;
    let role = self.role;
    let velocity_tmp = self.velocity.as_ref().map(|x| x.pack());
    let velocity = velocity_tmp.as_ref();
    Player::create(_fbb, &PlayerArgs{
      hp,
      id,
      look_direction,
      position,
      speed,
      sprinting,
      sneaking,
      role,
      velocity,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Player`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_player_unchecked`.
pub fn root_as_player(buf: &[u8]) -> Result<Player, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Player>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Player` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_player_unchecked`.
pub fn size_prefixed_root_as_player(buf: &[u8]) -> Result<Player, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Player>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Player` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_player_unchecked`.
pub fn root_as_player_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Player<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Player<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Player` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_player_unchecked`.
pub fn size_prefixed_root_as_player_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Player<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Player<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Player and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Player`.
pub unsafe fn root_as_player_unchecked(buf: &[u8]) -> Player {
  flatbuffers::root_unchecked::<Player>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Player and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Player`.
pub unsafe fn size_prefixed_root_as_player_unchecked(buf: &[u8]) -> Player {
  flatbuffers::size_prefixed_root_unchecked::<Player>(buf)
}
#[inline]
pub fn finish_player_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Player<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_player_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Player<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Player

