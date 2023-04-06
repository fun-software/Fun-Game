import { deserializeChatMessage, serializeChatMessage } from "@/utils/messaging";
import { requestJoinGame } from "@/utils/requests";
import { ChatMessageT, ChatSource } from "@fb/Chat";
import { useSession } from "@supabase/auth-helpers-react";
import * as React from "react";

const API_HOST = process.env.NEXT_PUBLIC_API_HOST || "127.0.0.1";

export function useChatConnection(game_id: string) {
  const session = useSession();
  let [port, setPort] = React.useState<number>(undefined);
  let [socket, setSocket] = React.useState<WebSocket>(undefined);
  const [chatMessages, setChatMessages] = React.useState<ChatMessageT[]>([]);

  React.useEffect(() => {
    if (!game_id || !session) return;
    requestJoinGame(session, game_id).then(address => {
      setPort(address);
    });
  }, [game_id, session]);

  React.useEffect(() => {
    if (!port) return;

    let protocol = window.location.protocol === "https:" ? "wss" : "ws";
    let ws = new WebSocket(`${protocol}://${API_HOST}:${port}`);
    ws.onmessage = e => {
      let blob: Blob = e.data;
      blob.arrayBuffer().then(buf => {
        let msg = deserializeChatMessage(new Uint8Array(buf));
        setChatMessages(prev => prev.concat(msg));
      });
    };

    setSocket(ws);

    return () => {
      ws.close();
    };
  }, [port]);

  let sendChatMessage = React.useCallback(
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
