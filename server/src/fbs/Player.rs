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

  pub fn unpack(&self) -> Vec3T {
    Vec3T {
      x: self.x(),
      y: self.y(),
      z: self.z(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vec3T {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}
impl Vec3T {
  pub fn pack(&self) -> Vec3 {
    Vec3::new(
      self.x,
      self.y,
      self.z,
    )
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

  pub fn unpack(&self) -> PlayerT {
    let position = self.position().map(|x| {
      x.unpack()
    });
    let velocity = self.velocity().map(|x| {
      x.unpack()
    });
    let look_direction = self.look_direction().map(|x| {
      x.unpack()
    });
    let hp = self.hp();
    let speed = self.speed();
    let sprinting = self.sprinting();
    let sneaking = self.sneaking();
    PlayerT {
      position,
      velocity,
      look_direction,
      hp,
      speed,
      sprinting,
      sneaking,
    }
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
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerT {
  pub position: Option<super::math::Vec3T>,
  pub velocity: Option<super::math::Vec3T>,
  pub look_direction: Option<super::math::Vec3T>,
  pub hp: i8,
  pub speed: i8,
  pub sprinting: bool,
  pub sneaking: bool,
}
impl Default for PlayerT {
  fn default() -> Self {
    Self {
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
impl PlayerT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Player<'b>> {
    let position_tmp = self.position.as_ref().map(|x| x.pack());
    let position = position_tmp.as_ref();
    let velocity_tmp = self.velocity.as_ref().map(|x| x.pack());
    let velocity = velocity_tmp.as_ref();
    let look_direction_tmp = self.look_direction.as_ref().map(|x| x.pack());
    let look_direction = look_direction_tmp.as_ref();
    let hp = self.hp;
    let speed = self.speed;
    let sprinting = self.sprinting;
    let sneaking = self.sneaking;
    Player::create(_fbb, &PlayerArgs{
      position,
      velocity,
      look_direction,
      hp,
      speed,
      sprinting,
      sneaking,
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

