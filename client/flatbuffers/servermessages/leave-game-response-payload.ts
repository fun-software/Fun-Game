// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { Game, GameT } from '../game/game';
import { ResponseCode } from '../servermessages/response-code';


export class LeaveGameResponsePayload implements flatbuffers.IUnpackableObject<LeaveGameResponsePayloadT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):LeaveGameResponsePayload {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsLeaveGameResponsePayload(bb:flatbuffers.ByteBuffer, obj?:LeaveGameResponsePayload):LeaveGameResponsePayload {
  return (obj || new LeaveGameResponsePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsLeaveGameResponsePayload(bb:flatbuffers.ByteBuffer, obj?:LeaveGameResponsePayload):LeaveGameResponsePayload {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new LeaveGameResponsePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

code():ResponseCode {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readInt8(this.bb_pos + offset) : ResponseCode.OK;
}

mutate_code(value:ResponseCode):boolean {
  const offset = this.bb!.__offset(this.bb_pos, 4);

  if (offset === 0) {
    return false;
  }

  this.bb!.writeInt8(this.bb_pos + offset, value);
  return true;
}

game(obj?:Game):Game|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? (obj || new Game()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

static startLeaveGameResponsePayload(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addCode(builder:flatbuffers.Builder, code:ResponseCode) {
  builder.addFieldInt8(0, code, ResponseCode.OK);
}

static addGame(builder:flatbuffers.Builder, gameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, gameOffset, 0);
}

static endLeaveGameResponsePayload(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): LeaveGameResponsePayloadT {
  return new LeaveGameResponsePayloadT(
    this.code(),
    (this.game() !== null ? this.game()!.unpack() : null)
  );
}


unpackTo(_o: LeaveGameResponsePayloadT): void {
  _o.code = this.code();
  _o.game = (this.game() !== null ? this.game()!.unpack() : null);
}
}

export class LeaveGameResponsePayloadT implements flatbuffers.IGeneratedObject {
constructor(
  public code: ResponseCode = ResponseCode.OK,
  public game: GameT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const game = (this.game !== null ? this.game!.pack(builder) : 0);

  LeaveGameResponsePayload.startLeaveGameResponsePayload(builder);
  LeaveGameResponsePayload.addCode(builder, this.code);
  LeaveGameResponsePayload.addGame(builder, game);

  return LeaveGameResponsePayload.endLeaveGameResponsePayload(builder);
}
}
