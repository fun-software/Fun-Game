// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class MovePayload {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):MovePayload {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsMovePayload(bb:flatbuffers.ByteBuffer, obj?:MovePayload):MovePayload {
  return (obj || new MovePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsMovePayload(bb:flatbuffers.ByteBuffer, obj?:MovePayload):MovePayload {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new MovePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

x():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

y():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

z():number {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

static startMovePayload(builder:flatbuffers.Builder) {
  builder.startObject(3);
}

static addX(builder:flatbuffers.Builder, x:number) {
  builder.addFieldFloat32(0, x, 0.0);
}

static addY(builder:flatbuffers.Builder, y:number) {
  builder.addFieldFloat32(1, y, 0.0);
}

static addZ(builder:flatbuffers.Builder, z:number) {
  builder.addFieldFloat32(2, z, 0.0);
}

static endMovePayload(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createMovePayload(builder:flatbuffers.Builder, x:number, y:number, z:number):flatbuffers.Offset {
  MovePayload.startMovePayload(builder);
  MovePayload.addX(builder, x);
  MovePayload.addY(builder, y);
  MovePayload.addZ(builder, z);
  return MovePayload.endMovePayload(builder);
}
}
