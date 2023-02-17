import Link from "next/link";
import styles from "./MainMenu.module.scss";

export default function MainMenu() {
  return (
    <main className={styles.menu}>
      <div className={styles.background} />

      <h1>Fun.Game</h1>

      <nav>
        <Link href={"#"}>New Game</Link>
        <Link href={"#"}>Join Game</Link>
        <Link href={"#"}>Settings</Link>
      </nav>
    </main>
  );
}
