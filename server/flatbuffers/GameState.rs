// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod math {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

// struct Vec3, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(pub [u8; 12]);
impl Default for Vec3 { 
  fn default() -> Self { 
    Self([0; 12])
  }
}
impl core::fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Vec3")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vec3 {}
impl<'a> flatbuffers::Follow<'a> for Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vec3>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vec3>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vec3 {
    type Output = Vec3;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Vec3 as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Vec3 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Vec3 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f32,
    y: f32,
    z: f32,
  ) -> Self {
    let mut s = Self([0; 12]);
    s.set_x(x);
    s.set_y(y);
    s.set_z(z);
    s
  }

  pub fn x(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_x(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn y(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_y(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn z(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_z(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

}  // pub mod Math

#[allow(unused_imports, dead_code)]
pub mod player {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

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
  pub const VT_POSITION: flatbuffers::VOffsetT = 4;
  pub const VT_VELOCITY: flatbuffers::VOffsetT = 6;
  pub const VT_LOOK_DIRECTION: flatbuffers::VOffsetT = 8;
  pub const VT_HP: flatbuffers::VOffsetT = 10;
  pub const VT_SPEED: flatbuffers::VOffsetT = 12;
  pub const VT_SPRINTING: flatbuffers::VOffsetT = 14;
  pub const VT_SNEAKING: flatbuffers::VOffsetT = 16;

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
    if let Some(x) = args.look_direction { builder.add_look_direction(x); }
    if let Some(x) = args.velocity { builder.add_velocity(x); }
    if let Some(x) = args.position { builder.add_position(x); }
    builder.add_sneaking(args.sneaking);
    builder.add_sprinting(args.sprinting);
    builder.add_speed(args.speed);
    builder.add_hp(args.hp);
    builder.finish()
  }


  #[inline]
  pub fn position(&self) -> Option<&'a super::math::Vec3> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::math::Vec3>(Player::VT_POSITION, None)}
  }
  #[inline]
  pub fn velocity(&self) -> Option<&'a super::math::Vec3> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::math::Vec3>(Player::VT_VELOCITY, None)}
  }
  #[inline]
  pub fn look_direction(&self) -> Option<&'a super::math::Vec3> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::math::Vec3>(Player::VT_LOOK_DIRECTION, None)}
  }
  #[inline]
  pub fn hp(&self) -> i8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i8>(Player::VT_HP, Some(100)).unwrap()}
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
}

impl flatbuffers::Verifiable for Player<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<super::math::Vec3>("position", Self::VT_POSITION, false)?
     .visit_field::<super::math::Vec3>("velocity", Self::VT_VELOCITY, false)?
     .visit_field::<super::math::Vec3>("look_direction", Self::VT_LOOK_DIRECTION, false)?
     .visit_field::<i8>("hp", Self::VT_HP, false)?
     .visit_field::<i8>("speed", Self::VT_SPEED, false)?
     .visit_field::<bool>("sprinting", Self::VT_SPRINTING, false)?
     .visit_field::<bool>("sneaking", Self::VT_SNEAKING, false)?
     .finish();
    Ok(())
  }
}
pub struct PlayerArgs<'a> {
    pub position: Option<&'a super::math::Vec3>,
    pub velocity: Option<&'a super::math::Vec3>,
    pub look_direction: Option<&'a super::math::Vec3>,
    pub hp: i8,
    pub speed: i8,
    pub sprinting: bool,
    pub sneaking: bool,
}
impl<'a> Default for PlayerArgs<'a> {
  #[inline]
  fn default() -> Self {
    PlayerArgs {
      position: None,
      velocity: None,
      look_direction: None,
      hp: 100,
      speed: 100,
      sprinting: false,
      sneaking: false,
    }
  }
}

pub struct PlayerBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PlayerBuilder<'a, 'b> {
  #[inline]
  pub fn add_position(&mut self, position: &super::math::Vec3) {
    self.fbb_.push_slot_always::<&super::math::Vec3>(Player::VT_POSITION, position);
  }
  #[inline]
  pub fn add_velocity(&mut self, velocity: &super::math::Vec3) {
    self.fbb_.push_slot_always::<&super::math::Vec3>(Player::VT_VELOCITY, velocity);
  }
  #[inline]
  pub fn add_look_direction(&mut self, look_direction: &super::math::Vec3) {
    self.fbb_.push_slot_always::<&super::math::Vec3>(Player::VT_LOOK_DIRECTION, look_direction);
  }
  #[inline]
  pub fn add_hp(&mut self, hp: i8) {
    self.fbb_.push_slot::<i8>(Player::VT_HP, hp, 100);
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
      ds.field("position", &self.position());
      ds.field("velocity", &self.velocity());
      ds.field("look_direction", &self.look_direction());
      ds.field("hp", &self.hp());
      ds.field("speed", &self.speed());
      ds.field("sprinting", &self.sprinting());
      ds.field("sneaking", &self.sneaking());
      ds.finish()
  }
}
}  // pub mod Player

#[allow(unused_imports, dead_code)]
pub mod user {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum UserOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct User<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for User<'a> {
  type Inner = User<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> User<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_USERNAME: flatbuffers::VOffsetT = 6;
  pub const VT_EMAIL: flatbuffers::VOffsetT = 8;
  pub const VT_CREATED_AT: flatbuffers::VOffsetT = 10;
  pub const VT_UPDATED_AT: flatbuffers::VOffsetT = 12;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    User { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args UserArgs<'args>
  ) -> flatbuffers::WIPOffset<User<'bldr>> {
    let mut builder = UserBuilder::new(_fbb);
    builder.add_updated_at(args.updated_at);
    builder.add_created_at(args.created_at);
    builder.add_id(args.id);
    if let Some(x) = args.email { builder.add_email(x); }
    if let Some(x) = args.username { builder.add_username(x); }
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(User::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn username(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(User::VT_USERNAME, None)}
  }
  #[inline]
  pub fn email(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(User::VT_EMAIL, None)}
  }
  #[inline]
  pub fn created_at(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(User::VT_CREATED_AT, Some(0)).unwrap()}
  }
  #[inline]
  pub fn updated_at(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(User::VT_UPDATED_AT, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for User<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("username", Self::VT_USERNAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("email", Self::VT_EMAIL, false)?
     .visit_field::<u64>("created_at", Self::VT_CREATED_AT, false)?
     .visit_field::<u64>("updated_at", Self::VT_UPDATED_AT, false)?
     .finish();
    Ok(())
  }
}
pub struct UserArgs<'a> {
    pub id: u64,
    pub username: Option<flatbuffers::WIPOffset<&'a str>>,
    pub email: Option<flatbuffers::WIPOffset<&'a str>>,
    pub created_at: u64,
    pub updated_at: u64,
}
impl<'a> Default for UserArgs<'a> {
  #[inline]
  fn default() -> Self {
    UserArgs {
      id: 0,
      username: None,
      email: None,
      created_at: 0,
      updated_at: 0,
    }
  }
}

pub struct UserBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> UserBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: u64) {
    self.fbb_.push_slot::<u64>(User::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_username(&mut self, username: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(User::VT_USERNAME, username);
  }
  #[inline]
  pub fn add_email(&mut self, email: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(User::VT_EMAIL, email);
  }
  #[inline]
  pub fn add_created_at(&mut self, created_at: u64) {
    self.fbb_.push_slot::<u64>(User::VT_CREATED_AT, created_at, 0);
  }
  #[inline]
  pub fn add_updated_at(&mut self, updated_at: u64) {
    self.fbb_.push_slot::<u64>(User::VT_UPDATED_AT, updated_at, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> UserBuilder<'a, 'b> {
    let start = _fbb.start_table();
    UserBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<User<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for User<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("User");
      ds.field("id", &self.id());
      ds.field("username", &self.username());
      ds.field("email", &self.email());
      ds.field("created_at", &self.created_at());
      ds.field("updated_at", &self.updated_at());
      ds.finish()
  }
}
}  // pub mod User

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


  #[inline]
  pub fn red(&self) -> Option<super::user::User<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<super::user::User>>(PlayerRoles::VT_RED, None)}
  }
  #[inline]
  pub fn blue(&self) -> Option<super::user::User<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<super::user::User>>(PlayerRoles::VT_BLUE, None)}
  }
  #[inline]
  pub fn green(&self) -> Option<super::user::User<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<super::user::User>>(PlayerRoles::VT_GREEN, None)}
  }
  #[inline]
  pub fn yellow(&self) -> Option<super::user::User<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<super::user::User>>(PlayerRoles::VT_YELLOW, None)}
  }
}

impl flatbuffers::Verifiable for PlayerRoles<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::user::User>>("red", Self::VT_RED, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::user::User>>("blue", Self::VT_BLUE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::user::User>>("green", Self::VT_GREEN, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::user::User>>("yellow", Self::VT_YELLOW, false)?
     .finish();
    Ok(())
  }
}
pub struct PlayerRolesArgs<'a> {
    pub red: Option<flatbuffers::WIPOffset<super::user::User<'a>>>,
    pub blue: Option<flatbuffers::WIPOffset<super::user::User<'a>>>,
    pub green: Option<flatbuffers::WIPOffset<super::user::User<'a>>>,
    pub yellow: Option<flatbuffers::WIPOffset<super::user::User<'a>>>,
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
  pub fn add_red(&mut self, red: flatbuffers::WIPOffset<super::user::User<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::user::User>>(PlayerRoles::VT_RED, red);
  }
  #[inline]
  pub fn add_blue(&mut self, blue: flatbuffers::WIPOffset<super::user::User<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::user::User>>(PlayerRoles::VT_BLUE, blue);
  }
  #[inline]
  pub fn add_green(&mut self, green: flatbuffers::WIPOffset<super::user::User<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::user::User>>(PlayerRoles::VT_GREEN, green);
  }
  #[inline]
  pub fn add_yellow(&mut self, yellow: flatbuffers::WIPOffset<super::user::User<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::user::User>>(PlayerRoles::VT_YELLOW, yellow);
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
}  // pub mod Game

#[allow(unused_imports, dead_code)]
pub mod game_state {

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
}  // pub mod GameState

