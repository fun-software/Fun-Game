import { API_HOST } from "@/utils/env";
import { deserializeChatMessage, serializeChatMessage } from "@/utils/messaging";
import { requestJoinGame } from "@/utils/requests";
import { ChatMessageT, ChatSource } from "@fb/Chat";
import { useSession } from "@supabase/auth-helpers-react";
import { useCallback, useEffect, useState } from "react";

export function useChatConnection(game_id: string) {
  const session = useSession();
  let [port, setPort] = useState<number>(undefined);
  let [socket, setSocket] = useState<WebSocket>(undefined);
  const [chatMessages, setChatMessages] = useState<ChatMessageT[]>([]);

  useEffect(() => {
    if (!game_id || !session) return;
    requestJoinGame(session, game_id).then((address) => {
      setPort(address);
    });
  }, [game_id, session]);

  useEffect(() => {
    if (!port) return;

    let protocol = window.location.protocol === "https:" ? "wss" : "ws";
    let ws = new WebSocket(`${protocol}://${API_HOST}:${port}`);
    ws.onmessage = (e) => {
      let blob: Blob = e.data;
      blob.arrayBuffer().then((buf) => {
        let msg = deserializeChatMessage(new Uint8Array(buf));
        setChatMessages((prev) => prev.concat(msg));
      });
    };

    setSocket(ws);

    return () => {
      ws.close();
    };
  }, [port]);

  useEffect(() => {
    if (!socket || !socket.OPEN) return;
    let heartbeat = setInterval(() => {
      socket.send("ping");
    }, 1000);

    return () => {
      clearInterval(heartbeat);
    };
  }, [socket]);

  let sendChatMessage = useCallback(
    (message: string) => {
      if (!socket || !message) return;

      let author = session?.user?.email || "anonymous";
      let msg = serializeChatMessage(message, author, ChatSource.Player);
      socket.send(msg);
    },
    [socket, session],
  );

  return [chatMessages, sendChatMessage] as const;
}
