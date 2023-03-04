import * as React from "react";
import Link from "next/link";
import styles from "./MainMenu.module.scss";
import { requestNewGame } from "@/utils/requests";
import { useSession } from "@supabase/auth-helpers-react";
import { useRouter } from "next/router";

export default function MainMenu() {
  const session = useSession();
  const router = useRouter();

  const [gid, setGid] = React.useState<string>("");

  const handleNewGame = React.useCallback(
    e => {
      e.preventDefault();

      requestNewGame(session).then(game_id => {
        console.log("got game id: ", game_id);
        router.push(`/lobby?id=${game_id}`);
      });
    },
    [session, router],
  );

  return (
    <main className={styles.menu}>
      <div className={styles.background} />

      <h1>Fun.Game</h1>

      <nav>
        <a onClick={handleNewGame}>New Game</a>
        <div>
          <input
            value={gid}
            onChange={e => {
              setGid(e.target.value);
            }}
          />
          <Link href={`/lobby?id=${gid}`}>Join Game</Link>
        </div>
        <button>Settings</button>
      </nav>
    </main>
  );
}
