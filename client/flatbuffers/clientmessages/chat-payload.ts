// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { ChatMessage } from '../chat/chat-message';
import { User } from '../user/user';


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

user(obj?:User):User|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new User()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

message(obj?:ChatMessage):ChatMessage|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? (obj || new ChatMessage()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

static startChatPayload(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addUser(builder:flatbuffers.Builder, userOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, userOffset, 0);
}

static addMessage(builder:flatbuffers.Builder, messageOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, messageOffset, 0);
}

static endChatPayload(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

}