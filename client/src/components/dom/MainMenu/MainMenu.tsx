import Link from "next/link";
import styles from "./MainMenu.module.scss";
import { User } from "@fb/User";
import {
  JoinGameResponsePayloadT,
  NewGameResponsePayloadT,
  ServerMessage,
} from "@fb/ServerMessages";
import {
  ClientMessage,
  ClientMessagePayload,
  NewGamePayload,
  JoinGamePayload,
} from "@fb/ClientMessages";
import { Builder, ByteBuffer } from "flatbuffers";

export default function MainMenu() {
  const handleNewGame = async () => {
    const builder = new Builder();

    const username = builder.createString("test");
    const email = builder.createString("qwe@gmail.com");

    const user = User.createUser(builder, BigInt(1), username, email, BigInt(4), BigInt(5));
    const payload = NewGamePayload.createNewGamePayload(builder, user);
    const message = ClientMessage.createClientMessage(
      builder,
      BigInt(Date.now()),
      ClientMessagePayload.NewGamePayload,
      payload,
    );

    builder.finish(message);
    const bytes = builder.asUint8Array();

    let res = await fetch("http://127.0.0.1:8080/new_game", {
      method: "POST",
      body: bytes,
    });

    let data = await res.arrayBuffer();
    let byteArr = new Uint8Array(data);
    let dataBuffer = new ByteBuffer(byteArr);

    let msg = ServerMessage.getRootAsServerMessage(dataBuffer).unpack();

    let responsePayload = msg.payload as NewGameResponsePayloadT;

    let ws = new WebSocket(responsePayload.socketAddress as string);
    ws.onopen = () => {
      window["ws"] = ws;
      ws.send("Hello from client1");
    };
    ws.onmessage = e => {
      console.log(e.data);
    };
  };

  const handleJoinGame = async () => {
    const builder = new Builder();

    const username = builder.createString("test2");
    const email = builder.createString("qwe2@gmail.com");

    const user = User.createUser(builder, BigInt(2), username, email, BigInt(42), BigInt(52));
    const payload = JoinGamePayload.createJoinGamePayload(builder, user, BigInt(1));
    const message = ClientMessage.createClientMessage(
      builder,
      BigInt(Date.now()),
      ClientMessagePayload.JoinGamePayload,
      payload,
    );

    builder.finish(message);
    const bytes = builder.asUint8Array();

    let res = await fetch("http://127.0.0.1:8080/join_game", {
      method: "POST",
      body: bytes,
    });

    let data = await res.arrayBuffer();
    let byteArr = new Uint8Array(data);
    let dataBuffer = new ByteBuffer(byteArr);

    let msg = ServerMessage.getRootAsServerMessage(dataBuffer).unpack();

    let responsePayload = msg.payload as JoinGameResponsePayloadT;

    let ws = new WebSocket(responsePayload.socketAddress as string);
    ws.onopen = () => {
      window["ws"] = ws;
      ws.send("Hello from client2");
    };
    ws.onmessage = e => {
      console.log(e.data);
    };
  };

  return (
    <main className={styles.menu}>
      <div className={styles.background} />

      <h1>Fun.Game</h1>

      <nav>
        <button onClick={handleNewGame}>New Game</button>
        <button onClick={handleJoinGame}>Join Game</button>
        <button>Settings</button>
      </nav>
    </main>
  );
}
