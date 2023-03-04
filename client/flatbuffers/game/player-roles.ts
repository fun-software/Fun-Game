// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from "flatbuffers";

export class PlayerRoles implements flatbuffers.IUnpackableObject<PlayerRolesT> {
  bb: flatbuffers.ByteBuffer | null = null;
  bb_pos = 0;
  __init(i: number, bb: flatbuffers.ByteBuffer): PlayerRoles {
    this.bb_pos = i;
    this.bb = bb;
    return this;
  }

  static getRootAsPlayerRoles(bb: flatbuffers.ByteBuffer, obj?: PlayerRoles): PlayerRoles {
    return (obj || new PlayerRoles()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  static getSizePrefixedRootAsPlayerRoles(
    bb: flatbuffers.ByteBuffer,
    obj?: PlayerRoles,
  ): PlayerRoles {
    bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
    return (obj || new PlayerRoles()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  red(): string | null;
  red(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  red(optionalEncoding?: any): string | Uint8Array | null {
    const offset = this.bb!.__offset(this.bb_pos, 4);
    return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
  }

  blue(): string | null;
  blue(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  blue(optionalEncoding?: any): string | Uint8Array | null {
    const offset = this.bb!.__offset(this.bb_pos, 6);
    return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
  }

  green(): string | null;
  green(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  green(optionalEncoding?: any): string | Uint8Array | null {
    const offset = this.bb!.__offset(this.bb_pos, 8);
    return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
  }

  yellow(): string | null;
  yellow(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  yellow(optionalEncoding?: any): string | Uint8Array | null {
    const offset = this.bb!.__offset(this.bb_pos, 10);
    return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
  }

  static startPlayerRoles(builder: flatbuffers.Builder) {
    builder.startObject(4);
  }

  static addRed(builder: flatbuffers.Builder, redOffset: flatbuffers.Offset) {
    builder.addFieldOffset(0, redOffset, 0);
  }

  static addBlue(builder: flatbuffers.Builder, blueOffset: flatbuffers.Offset) {
    builder.addFieldOffset(1, blueOffset, 0);
  }

  static addGreen(builder: flatbuffers.Builder, greenOffset: flatbuffers.Offset) {
    builder.addFieldOffset(2, greenOffset, 0);
  }

  static addYellow(builder: flatbuffers.Builder, yellowOffset: flatbuffers.Offset) {
    builder.addFieldOffset(3, yellowOffset, 0);
  }

  static endPlayerRoles(builder: flatbuffers.Builder): flatbuffers.Offset {
    const offset = builder.endObject();
    return offset;
  }

  static createPlayerRoles(
    builder: flatbuffers.Builder,
    redOffset: flatbuffers.Offset,
    blueOffset: flatbuffers.Offset,
    greenOffset: flatbuffers.Offset,
    yellowOffset: flatbuffers.Offset,
  ): flatbuffers.Offset {
    PlayerRoles.startPlayerRoles(builder);
    PlayerRoles.addRed(builder, redOffset);
    PlayerRoles.addBlue(builder, blueOffset);
    PlayerRoles.addGreen(builder, greenOffset);
    PlayerRoles.addYellow(builder, yellowOffset);
    return PlayerRoles.endPlayerRoles(builder);
  }

  unpack(): PlayerRolesT {
    return new PlayerRolesT(this.red(), this.blue(), this.green(), this.yellow());
  }

  unpackTo(_o: PlayerRolesT): void {
    _o.red = this.red();
    _o.blue = this.blue();
    _o.green = this.green();
    _o.yellow = this.yellow();
  }
}

export class PlayerRolesT implements flatbuffers.IGeneratedObject {
  constructor(
    public red: string | Uint8Array | null = null,
    public blue: string | Uint8Array | null = null,
    public green: string | Uint8Array | null = null,
    public yellow: string | Uint8Array | null = null,
  ) {}

  pack(builder: flatbuffers.Builder): flatbuffers.Offset {
    const red = this.red !== null ? builder.createString(this.red!) : 0;
    const blue = this.blue !== null ? builder.createString(this.blue!) : 0;
    const green = this.green !== null ? builder.createString(this.green!) : 0;
    const yellow = this.yellow !== null ? builder.createString(this.yellow!) : 0;

    return PlayerRoles.createPlayerRoles(builder, red, blue, green, yellow);
  }
}
