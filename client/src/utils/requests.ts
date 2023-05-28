import {
  ClientMessage,
  ClientMessagePayload,
  JoinGamePayload,
  LeaveGamePayload,
  NewGamePayload,
} from "@fb/ClientMessages";
import {
  JoinGameResponsePayloadT,
  NewGameResponsePayloadT,
  ServerMessage,
} from "@fb/ServerMessages";
import { User } from "@fb/User";
import { Session } from "@supabase/supabase-js";
import { Builder, ByteBuffer } from "flatbuffers";
import { API_URL } from "./env";

function normalizeSession(session: Session): Session {
  if (!session) {
    return {
      user: {
        id: window["uid"] || "1",
        email: "anonymous1@gmail.com",
        confirmed_at: "1",
        updated_at: "1",
      },
    } as Session;
  }

  return session;
}

export async function requestNewGame(_session: Session): Promise<string> {
  const session = normalizeSession(_session);
  const builder = new Builder();

  // const username = builder.createString("TODO");
  const id = builder.createString(session.user.id);
  const email = builder.createString(session.user.email);

  User.startUser(builder);
  User.addId(builder, id);
  User.addEmail(builder, email);
  User.addUsername(builder, email);
  const user = User.endUser(builder);
  const payload = NewGamePayload.createNewGamePayload(builder, user);
  const message = ClientMessage.createClientMessage(
    builder,
    BigInt(Date.now()),
    ClientMessagePayload.NewGamePayload,
    payload,
  );

  builder.finish(message);
  const bytes = builder.asUint8Array();

  let res = await fetch(`${API_URL}/new_game`, {
    method: "POST",
    body: bytes,
  });

  let data = await res.arrayBuffer();
  let byteArr = new Uint8Array(data);
  let dataBuffer = new ByteBuffer(byteArr);

  let msg = ServerMessage.getRootAsServerMessage(dataBuffer).unpack();

  let responsePayload = msg.payload as NewGameResponsePayloadT;

  return responsePayload.gameId as string;
}

export async function requestJoinGame(_session: Session, gameId: string): Promise<number> {
  const session = normalizeSession(_session);
  const builder = new Builder();

  // const username = builder.createString("TODO");
  const id = builder.createString(session.user.id);
  const email = builder.createString(session.user.email);
  const gid = builder.createString(gameId);

  User.startUser(builder);
  User.addId(builder, id);
  User.addEmail(builder, email);
  User.addUsername(builder, email);
  const user = User.endUser(builder);
  const payload = JoinGamePayload.createJoinGamePayload(builder, user, gid);
  const message = ClientMessage.createClientMessage(
    builder,
    BigInt(Date.now()),
    ClientMessagePayload.JoinGamePayload,
    payload,
  );

  builder.finish(message);
  const bytes = builder.asUint8Array();

  let res = await fetch(`${API_URL}/join_game`, {
    method: "POST",
    body: bytes,
  });

  let data = await res.arrayBuffer();
  let byteArr = new Uint8Array(data);
  let dataBuffer = new ByteBuffer(byteArr);

  let msg = ServerMessage.getRootAsServerMessage(dataBuffer).unpack();

  let responsePayload = msg.payload as JoinGameResponsePayloadT;

  return responsePayload.wsPort;
}

export async function requestLeaveGame(_session: Session) {
  const session = normalizeSession(_session);
  const builder = new Builder();

  // const username = builder.createString("TODO");
  const id = builder.createString(session.user.id);
  const email = builder.createString(session.user.email);

  User.startUser(builder);
  User.addId(builder, id);
  User.addEmail(builder, email);
  User.addUsername(builder, email);
  const user = User.endUser(builder);
  const payload = LeaveGamePayload.createLeaveGamePayload(builder, user);
  const message = ClientMessage.createClientMessage(
    builder,
    BigInt(Date.now()),
    ClientMessagePayload.LeaveGamePayload,
    payload,
  );

  builder.finish(message);
  const bytes = builder.asUint8Array();

  let res = await fetch(`${API_URL}/leave_game`, {
    method: "POST",
    body: bytes,
  });

  if (res.status !== 200) {
    throw new Error("Failed to leave game");
  }

  return;
}
