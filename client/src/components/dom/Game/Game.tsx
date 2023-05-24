import Link from "next/link";
import styles from "./Game.module.scss";

export function Game() {
  function handleLeaveGame() {
    // TODO: Implement
  }

  return (
    <div className={styles.HUD}>
      <h1>In-Game</h1>
      <Link href="/" onClick={handleLeaveGame}>
        Back Home
      </Link>
    </div>
  );
}
