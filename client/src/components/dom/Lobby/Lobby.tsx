import { useChatConnection } from "@/hooks/useChatConnection";
import { serializeChatMessage } from "@/utils/messaging";

import { ChatSource } from "@fb/Chat";
import Link from "next/link";
import { useRouter } from "next/router";
import * as React from "react";

export function Lobby() {
  const router = useRouter();
  const game_id = React.useMemo(() => router.query.id as string, [router]);
  const [chatMessages, sendMessage] = useChatConnection(game_id);
  const [message, setMessage] = React.useState<string>("");

  const handleSend = () => {
    sendMessage(message);
    setMessage("");
  };

  return (
    <div>
      <Link href={"/"}>Home</Link>
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

      <div className="chat">
        {chatMessages.map((msg, i) => (
          <p key={i} className={msg.source === ChatSource.System ? "system" : ""}>
            {msg.message}
          </p>
        ))}
      </div>
    </div>
  );
}
