import Link from "next/link";
import styles from "./MainMenu.module.scss";
import { User } from "@fb/User";
import { Game } from "@fb/Game";
import { ClientMessage, ClientMessagePayload, NewGamePayload } from "@fb/ClientMessages";
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

    let game = Game.getRootAsGame(new ByteBuffer(byteArr)).unpack();
    // console.log(game);
  };

  return (
    <main className={styles.menu}>
      <div className={styles.background} />

      <h1>Fun.Game</h1>

      <nav>
        <button onClick={handleNewGame}>New Game</button>
        <button>Join Game</button>
        <button>Settings</button>
      </nav>
    </main>
  );
}
