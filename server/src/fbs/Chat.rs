// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod chat {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_CHAT_SOURCE: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_CHAT_SOURCE: i8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_CHAT_SOURCE: [ChatSource; 2] = [
  ChatSource::System,
  ChatSource::Player,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ChatSource(pub i8);
#[allow(non_upper_case_globals)]
impl ChatSource {
  pub const System: Self = Self(0);
  pub const Player: Self = Self(1);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::System,
    Self::Player,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::System => Some("System"),
      Self::Player => Some("Player"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ChatSource {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ChatSource {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ChatSource {
    type Output = ChatSource;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ChatSource {
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

impl<'a> flatbuffers::Verifiable for ChatSource {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ChatSource {}
pub enum ChatMessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ChatMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ChatMessage<'a> {
  type Inner = ChatMessage<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ChatMessage<'a> {
  pub const VT_SOURCE: flatbuffers::VOffsetT = 4;
  pub const VT_MESSAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ChatMessage { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ChatMessageArgs<'args>
  ) -> flatbuffers::WIPOffset<ChatMessage<'bldr>> {
    let mut builder = ChatMessageBuilder::new(_fbb);
    if let Some(x) = args.message { builder.add_message(x); }
    builder.add_source(args.source);
    builder.finish()
  }

  pub fn unpack(&self) -> ChatMessageT {
    let source = self.source();
    let message = self.message().map(|x| {
      x.to_string()
    });
    ChatMessageT {
      source,
      message,
    }
  }

  #[inline]
  pub fn source(&self) -> ChatSource {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ChatSource>(ChatMessage::VT_SOURCE, Some(ChatSource::System)).unwrap()}
  }
  #[inline]
  pub fn message(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ChatMessage::VT_MESSAGE, None)}
  }
}

impl flatbuffers::Verifiable for ChatMessage<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<ChatSource>("source", Self::VT_SOURCE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("message", Self::VT_MESSAGE, false)?
     .finish();
    Ok(())
  }
}
pub struct ChatMessageArgs<'a> {
    pub source: ChatSource,
    pub message: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for ChatMessageArgs<'a> {
  #[inline]
  fn default() -> Self {
    ChatMessageArgs {
      source: ChatSource::System,
      message: None,
    }
  }
}

pub struct ChatMessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ChatMessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_source(&mut self, source: ChatSource) {
    self.fbb_.push_slot::<ChatSource>(ChatMessage::VT_SOURCE, source, ChatSource::System);
  }
  #[inline]
  pub fn add_message(&mut self, message: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ChatMessage::VT_MESSAGE, message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ChatMessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ChatMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ChatMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ChatMessage<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ChatMessage");
      ds.field("source", &self.source());
      ds.field("message", &self.message());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessageT {
  pub source: ChatSource,
  pub message: Option<String>,
}
impl Default for ChatMessageT {
  fn default() -> Self {
    Self {
      source: ChatSource::System,
      message: None,
    }
  }
}
impl ChatMessageT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<ChatMessage<'b>> {
    let source = self.source;
    let message = self.message.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    ChatMessage::create(_fbb, &ChatMessageArgs{
      source,
      message,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `ChatMessage`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_chat_message_unchecked`.
pub fn root_as_chat_message(buf: &[u8]) -> Result<ChatMessage, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<ChatMessage>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `ChatMessage` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_chat_message_unchecked`.
pub fn size_prefixed_root_as_chat_message(buf: &[u8]) -> Result<ChatMessage, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<ChatMessage>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `ChatMessage` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_chat_message_unchecked`.
pub fn root_as_chat_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ChatMessage<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<ChatMessage<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `ChatMessage` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_chat_message_unchecked`.
pub fn size_prefixed_root_as_chat_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ChatMessage<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<ChatMessage<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a ChatMessage and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `ChatMessage`.
pub unsafe fn root_as_chat_message_unchecked(buf: &[u8]) -> ChatMessage {
  flatbuffers::root_unchecked::<ChatMessage>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed ChatMessage and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `ChatMessage`.
pub unsafe fn size_prefixed_root_as_chat_message_unchecked(buf: &[u8]) -> ChatMessage {
  flatbuffers::size_prefixed_root_unchecked::<ChatMessage>(buf)
}
#[inline]
pub fn finish_chat_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ChatMessage<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_chat_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<ChatMessage<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Chat

