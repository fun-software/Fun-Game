// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from "flatbuffers";

import { ChatMessage, ChatMessageT } from "../chat/chat-message";
import { User, UserT } from "../user/user";

export class ChatPayload implements flatbuffers.IUnpackableObject<ChatPayloadT> {
  bb: flatbuffers.ByteBuffer | null = null;
  bb_pos = 0;
  __init(i: number, bb: flatbuffers.ByteBuffer): ChatPayload {
    this.bb_pos = i;
    this.bb = bb;
    return this;
  }

  static getRootAsChatPayload(bb: flatbuffers.ByteBuffer, obj?: ChatPayload): ChatPayload {
    return (obj || new ChatPayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  static getSizePrefixedRootAsChatPayload(
    bb: flatbuffers.ByteBuffer,
    obj?: ChatPayload,
  ): ChatPayload {
    bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
    return (obj || new ChatPayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  user(obj?: User): User | null {
    const offset = this.bb!.__offset(this.bb_pos, 4);
    return offset
      ? (obj || new User()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!)
      : null;
  }

  message(obj?: ChatMessage): ChatMessage | null {
    const offset = this.bb!.__offset(this.bb_pos, 6);
    return offset
      ? (obj || new ChatMessage()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!)
      : null;
  }

  static startChatPayload(builder: flatbuffers.Builder) {
    builder.startObject(2);
  }

  static addUser(builder: flatbuffers.Builder, userOffset: flatbuffers.Offset) {
    builder.addFieldOffset(0, userOffset, 0);
  }

  static addMessage(builder: flatbuffers.Builder, messageOffset: flatbuffers.Offset) {
    builder.addFieldOffset(1, messageOffset, 0);
  }

  static endChatPayload(builder: flatbuffers.Builder): flatbuffers.Offset {
    const offset = builder.endObject();
    return offset;
  }

  unpack(): ChatPayloadT {
    return new ChatPayloadT(
      this.user() !== null ? this.user()!.unpack() : null,
      this.message() !== null ? this.message()!.unpack() : null,
    );
  }

  unpackTo(_o: ChatPayloadT): void {
    _o.user = this.user() !== null ? this.user()!.unpack() : null;
    _o.message = this.message() !== null ? this.message()!.unpack() : null;
  }
}

export class ChatPayloadT implements flatbuffers.IGeneratedObject {
  constructor(public user: UserT | null = null, public message: ChatMessageT | null = null) {}

  pack(builder: flatbuffers.Builder): flatbuffers.Offset {
    const user = this.user !== null ? this.user!.pack(builder) : 0;
    const message = this.message !== null ? this.message!.pack(builder) : 0;

    ChatPayload.startChatPayload(builder);
    ChatPayload.addUser(builder, user);
    ChatPayload.addMessage(builder, message);

    return ChatPayload.endChatPayload(builder);
  }
}
