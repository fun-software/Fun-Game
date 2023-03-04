// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from "flatbuffers";

import { User, UserT } from "../user/user";

export class LeaveGamePayload implements flatbuffers.IUnpackableObject<LeaveGamePayloadT> {
  bb: flatbuffers.ByteBuffer | null = null;
  bb_pos = 0;
  __init(i: number, bb: flatbuffers.ByteBuffer): LeaveGamePayload {
    this.bb_pos = i;
    this.bb = bb;
    return this;
  }

  static getRootAsLeaveGamePayload(
    bb: flatbuffers.ByteBuffer,
    obj?: LeaveGamePayload,
  ): LeaveGamePayload {
    return (obj || new LeaveGamePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  static getSizePrefixedRootAsLeaveGamePayload(
    bb: flatbuffers.ByteBuffer,
    obj?: LeaveGamePayload,
  ): LeaveGamePayload {
    bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
    return (obj || new LeaveGamePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  user(obj?: User): User | null {
    const offset = this.bb!.__offset(this.bb_pos, 4);
    return offset
      ? (obj || new User()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!)
      : null;
  }

  static startLeaveGamePayload(builder: flatbuffers.Builder) {
    builder.startObject(1);
  }

  static addUser(builder: flatbuffers.Builder, userOffset: flatbuffers.Offset) {
    builder.addFieldOffset(0, userOffset, 0);
  }

  static endLeaveGamePayload(builder: flatbuffers.Builder): flatbuffers.Offset {
    const offset = builder.endObject();
    return offset;
  }

  static createLeaveGamePayload(
    builder: flatbuffers.Builder,
    userOffset: flatbuffers.Offset,
  ): flatbuffers.Offset {
    LeaveGamePayload.startLeaveGamePayload(builder);
    LeaveGamePayload.addUser(builder, userOffset);
    return LeaveGamePayload.endLeaveGamePayload(builder);
  }

  unpack(): LeaveGamePayloadT {
    return new LeaveGamePayloadT(this.user() !== null ? this.user()!.unpack() : null);
  }

  unpackTo(_o: LeaveGamePayloadT): void {
    _o.user = this.user() !== null ? this.user()!.unpack() : null;
  }
}

export class LeaveGamePayloadT implements flatbuffers.IGeneratedObject {
  constructor(public user: UserT | null = null) {}

  pack(builder: flatbuffers.Builder): flatbuffers.Offset {
    const user = this.user !== null ? this.user!.pack(builder) : 0;

    return LeaveGamePayload.createLeaveGamePayload(builder, user);
  }
}
