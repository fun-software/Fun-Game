import { useChatConnection } from "@/hooks/useChatConnection";
import styles from "./Lobby.module.scss";
import { ChatSource } from "@fb/Chat";
import Link from "next/link";
import { useRouter } from "next/router";
import { requestLeaveGame } from "@/utils/requests";
import { useSession } from "@supabase/auth-helpers-react";
import { MouseEvent, useCallback, useMemo, useState } from "react";

export function Lobby() {
  const router = useRouter();
  const session = useSession();
  const game_id = useMemo(() => router.query.id as string, [router]);
  const [chatMessages, sendMessage] = useChatConnection(game_id);
  const [message, setMessage] = useState<string>("");

  const handleSend = () => {
    sendMessage(message);
    setMessage("");
  };

  const handleStartGame = () => {
    router.push(`/game?game-id=${game_id}`);
  };

  const handleLeave = useCallback(
    (e: MouseEvent) => {
      e.preventDefault();
      requestLeaveGame(session);
      router.push("/");
    },
    [session, router],
  );

  return (
    <div className={styles.lobby}>
      <button onClick={handleLeave}>Home</button>
      <h1>Lobby</h1>

      <input
        value={message}
        onKeyDown={e => {
          e.key === "Enter" && handleSend();
        }}
        onChange={e => {
          setMessage(e.target.value);
        }}
      />
      <button onClick={handleSend}>Send</button>
      <button onClick={handleStartGame}>Start Game</button>

      <div className={styles.chat}>
        {chatMessages.map((msg, i) => (
          <div key={i} className={msg.source === ChatSource.System ? styles.system : styles.player}>
            <strong>{msg.author}</strong>
            <p>: {msg.message}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
