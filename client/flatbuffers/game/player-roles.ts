// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { User } from '../user/user';


export class PlayerRoles {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):PlayerRoles {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsPlayerRoles(bb:flatbuffers.ByteBuffer, obj?:PlayerRoles):PlayerRoles {
  return (obj || new PlayerRoles()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsPlayerRoles(bb:flatbuffers.ByteBuffer, obj?:PlayerRoles):PlayerRoles {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new PlayerRoles()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

red(obj?:User):User|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new User()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

blue(obj?:User):User|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? (obj || new User()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

green(obj?:User):User|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? (obj || new User()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

yellow(obj?:User):User|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? (obj || new User()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

static startPlayerRoles(builder:flatbuffers.Builder) {
  builder.startObject(4);
}

static addRed(builder:flatbuffers.Builder, redOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, redOffset, 0);
}

static addBlue(builder:flatbuffers.Builder, blueOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, blueOffset, 0);
}

static addGreen(builder:flatbuffers.Builder, greenOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, greenOffset, 0);
}

static addYellow(builder:flatbuffers.Builder, yellowOffset:flatbuffers.Offset) {
  builder.addFieldOffset(3, yellowOffset, 0);
}

static endPlayerRoles(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

}
