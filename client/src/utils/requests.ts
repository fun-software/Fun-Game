import {
  ClientMessage,
  ClientMessagePayload,
  JoinGamePayload,
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

const API_HOST = process.env.NEXT_PUBLIC_API_HOST || "http://127.0.0.1:8080";

export async function requestNewGame(session: Session): Promise<string> {
  if (!session) {
    session = {
      user: {
        id: window["uid"] || "1",
        email: "anonymous1@gmail.com",
        confirmed_at: "1",
        updated_at: "1",
      },
    } as Session;
  }
  const builder = new Builder();

  // const username = builder.createString("TODO");
  const id = builder.createString(session.user.id);
  const email = builder.createString(session.user.email);

  const user = User.createUser(
    builder,
    id,
    email,
    email,
    BigInt(session.user.confirmed_at),
    BigInt(session.user.updated_at),
  );
  const payload = NewGamePayload.createNewGamePayload(builder, user);
  const message = ClientMessage.createClientMessage(
    builder,
    BigInt(Date.now()),
    ClientMessagePayload.NewGamePayload,
    payload,
  );

  builder.finish(message);
  const bytes = builder.asUint8Array();

  let res = await fetch(`${API_HOST}/new_game`, {
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

export async function requestJoinGame(session: Session, gameId: string): Promise<string> {
  if (!session) {
    session = {
      user: {
        id: window["uid"] || "2",
        email: "anonymous2@gmail.com",
        confirmed_at: "2",
        updated_at: "2",
      },
    } as Session;
  }
  const builder = new Builder();

  // const username = builder.createString("TODO");
  const id = builder.createString(session.user.id);
  const email = builder.createString(session.user.email);
  const gid = builder.createString(gameId);

  const user = User.createUser(
    builder,
    id,
    email,
    email,
    BigInt(session.user.confirmed_at),
    BigInt(session.user.updated_at),
  );
  const payload = JoinGamePayload.createJoinGamePayload(builder, user, gid);
  const message = ClientMessage.createClientMessage(
    builder,
    BigInt(Date.now()),
    ClientMessagePayload.JoinGamePayload,
    payload,
  );

  builder.finish(message);
  const bytes = builder.asUint8Array();

  let res = await fetch(`${API_HOST}/join_game`, {
    method: "POST",
    body: bytes,
  });

  let data = await res.arrayBuffer();
  let byteArr = new Uint8Array(data);
  let dataBuffer = new ByteBuffer(byteArr);

  let msg = ServerMessage.getRootAsServerMessage(dataBuffer).unpack();

  let responsePayload = msg.payload as JoinGameResponsePayloadT;

  return responsePayload.socketAddress as string;
}
