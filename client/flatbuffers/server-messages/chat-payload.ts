// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class ChatPayload {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):ChatPayload {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsChatPayload(bb:flatbuffers.ByteBuffer, obj?:ChatPayload):ChatPayload {
  return (obj || new ChatPayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsChatPayload(bb:flatbuffers.ByteBuffer, obj?:ChatPayload):ChatPayload {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new ChatPayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

name():string|null
name(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
name(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

message():string|null
message(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
message(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startChatPayload(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addName(builder:flatbuffers.Builder, nameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, nameOffset, 0);
}

static addMessage(builder:flatbuffers.Builder, messageOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, messageOffset, 0);
}

static endChatPayload(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createChatPayload(builder:flatbuffers.Builder, nameOffset:flatbuffers.Offset, messageOffset:flatbuffers.Offset):flatbuffers.Offset {
  ChatPayload.startChatPayload(builder);
  ChatPayload.addName(builder, nameOffset);
  ChatPayload.addMessage(builder, messageOffset);
  return ChatPayload.endChatPayload(builder);
}
}
