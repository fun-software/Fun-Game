// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { GamePhase } from '../game/game-phase';
import { PlayerRoles, PlayerRolesT } from '../game/player-roles';


export class Game implements flatbuffers.IUnpackableObject<GameT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):Game {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsGame(bb:flatbuffers.ByteBuffer, obj?:Game):Game {
  return (obj || new Game()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsGame(bb:flatbuffers.ByteBuffer, obj?:Game):Game {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new Game()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

id():string|null
id(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
id(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

phase():GamePhase {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readInt8(this.bb_pos + offset) : GamePhase.Lobby;
}

players(index: number):string
players(index: number,optionalEncoding:flatbuffers.Encoding):string|Uint8Array
players(index: number,optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__string(this.bb!.__vector(this.bb_pos + offset) + index * 4, optionalEncoding) : null;
}

playersLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

playerRoles(obj?:PlayerRoles):PlayerRoles|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? (obj || new PlayerRoles()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

startTime():bigint {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.readUint64(this.bb_pos + offset) : BigInt('0');
}

endTime():bigint {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.readUint64(this.bb_pos + offset) : BigInt('0');
}

static startGame(builder:flatbuffers.Builder) {
  builder.startObject(6);
}

static addId(builder:flatbuffers.Builder, idOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, idOffset, 0);
}

static addPhase(builder:flatbuffers.Builder, phase:GamePhase) {
  builder.addFieldInt8(1, phase, GamePhase.Lobby);
}

static addPlayers(builder:flatbuffers.Builder, playersOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, playersOffset, 0);
}

static createPlayersVector(builder:flatbuffers.Builder, data:flatbuffers.Offset[]):flatbuffers.Offset {
  builder.startVector(4, data.length, 4);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addOffset(data[i]!);
  }
  return builder.endVector();
}

static startPlayersVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(4, numElems, 4);
}

static addPlayerRoles(builder:flatbuffers.Builder, playerRolesOffset:flatbuffers.Offset) {
  builder.addFieldOffset(3, playerRolesOffset, 0);
}

static addStartTime(builder:flatbuffers.Builder, startTime:bigint) {
  builder.addFieldInt64(4, startTime, BigInt('0'));
}

static addEndTime(builder:flatbuffers.Builder, endTime:bigint) {
  builder.addFieldInt64(5, endTime, BigInt('0'));
}

static endGame(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static finishGameBuffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset);
}

static finishSizePrefixedGameBuffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset, undefined, true);
}


unpack(): GameT {
  return new GameT(
    this.id(),
    this.phase(),
    this.bb!.createScalarList<string>(this.players.bind(this), this.playersLength()),
    (this.playerRoles() !== null ? this.playerRoles()!.unpack() : null),
    this.startTime(),
    this.endTime()
  );
}


unpackTo(_o: GameT): void {
  _o.id = this.id();
  _o.phase = this.phase();
  _o.players = this.bb!.createScalarList<string>(this.players.bind(this), this.playersLength());
  _o.playerRoles = (this.playerRoles() !== null ? this.playerRoles()!.unpack() : null);
  _o.startTime = this.startTime();
  _o.endTime = this.endTime();
}
}

export class GameT implements flatbuffers.IGeneratedObject {
constructor(
  public id: string|Uint8Array|null = null,
  public phase: GamePhase = GamePhase.Lobby,
  public players: (string)[] = [],
  public playerRoles: PlayerRolesT|null = null,
  public startTime: bigint = BigInt('0'),
  public endTime: bigint = BigInt('0')
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const id = (this.id !== null ? builder.createString(this.id!) : 0);
  const players = Game.createPlayersVector(builder, builder.createObjectOffsetList(this.players));
  const playerRoles = (this.playerRoles !== null ? this.playerRoles!.pack(builder) : 0);

  Game.startGame(builder);
  Game.addId(builder, id);
  Game.addPhase(builder, this.phase);
  Game.addPlayers(builder, players);
  Game.addPlayerRoles(builder, playerRoles);
  Game.addStartTime(builder, this.startTime);
  Game.addEndTime(builder, this.endTime);

  return Game.endGame(builder);
}
}
