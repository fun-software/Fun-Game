import * as React from "react";
import { Builder, ByteBuffer } from "flatbuffers";
import {
  JoinPayload,
  ClientMessagePayload,
  ClientMessage,
  LeavePayload,
  ChatPayload,
} from "@fb/client-messages";
import RTCClient from "@/utils/RTCClient";
import {
  ServerMessage,
  ServerMessagePayload,
  JoinPayload as JoinPayloadResponse,
  LeavePayload as LeavePayloadResponse,
  ChatPayload as ChatPayloadResponse,
} from "@fb/server-messages";

export default function Page() {
  const [msg, setMsg] = React.useState<string>("");
  const [responses, setResponses] = React.useState([]);
  const [client, setClient] = React.useState<RTCClient>();
  const [username, setUsername] = React.useState<string>(undefined);
  const [isAuth, setIsAuth] = React.useState<boolean>(false);

  React.useEffect(() => {
    const _client = new RTCClient();

    setClient(_client);

    return () => {
      _client.channel.close();
      setClient(undefined);
    };
  }, []);

  React.useEffect(() => {
    if (!client) return;
    client.channel.onmessage = e => {
      let bytes = new Uint8Array(e.data);

      let buf = new ByteBuffer(bytes);

      let msg = ServerMessage.getRootAsServerMessage(buf);

      let res: string = "";

      switch (msg.payloadType()) {
        case ServerMessagePayload.JoinPayload: {
          let payload: JoinPayloadResponse = msg.payload(new JoinPayloadResponse());
          if (payload.name() === username) {
            setIsAuth(true);
            res = `Join: ${username}`;
          }
          break;
        }
        case ServerMessagePayload.LeavePayload: {
          let payload: LeavePayloadResponse = msg.payload(new LeavePayloadResponse());
          if (payload.name() === username) {
            setIsAuth(false);
            res = `Leave: ${username}`;
          }
          break;
        }
        case ServerMessagePayload.ChatPayload: {
          let payload: ChatPayloadResponse = msg.payload(new ChatPayloadResponse());
          let name = payload.name();
          let message = payload.message();

          if (name !== "") {
            res = `Chat: ${name} - ${message}`;
          }
          break;
        }
        default: {
          break;
        }
      }

      if (res !== "") {
        setResponses(prev => [...prev, res]);
      }
    };
  }, [client, username]);

  const builder = React.useMemo(() => new Builder(1024), []);

  const sendMessage = React.useCallback(
    (type: ClientMessagePayload, payloadOffset: number) => {
      if (!client) return;

      const timestamp = BigInt(Date.now());

      ClientMessage.startClientMessage(builder);
      ClientMessage.addPayloadType(builder, type);
      ClientMessage.addPayload(builder, payloadOffset);
      ClientMessage.addTimestamp(builder, timestamp);
      let bufOffset = ClientMessage.endClientMessage(builder);
      builder.finish(bufOffset);

      client.channel.send(builder.asUint8Array());
    },
    [client, builder],
  );

  const sendJoin = React.useCallback(() => {
    if (!client) return;

    builder.clear();
    const name = builder.createString(username);
    JoinPayload.startJoinPayload(builder);
    JoinPayload.addName(builder, name);
    let bufOffset = JoinPayload.endJoinPayload(builder);

    sendMessage(ClientMessagePayload.JoinPayload, bufOffset);
  }, [client, builder, sendMessage, username]);

  const sendLeave = React.useCallback(() => {
    if (!client) return;

    builder.clear();
    const name = builder.createString(username);
    LeavePayload.startLeavePayload(builder);
    LeavePayload.addName(builder, name);
    let bufOffset = LeavePayload.endLeavePayload(builder);

    sendMessage(ClientMessagePayload.LeavePayload, bufOffset);
  }, [client, builder, sendMessage, username]);

  const sendChat = React.useCallback(() => {
    if (!client) return;

    builder.clear();
    const name = builder.createString(username);
    const message = builder.createString(msg);
    ChatPayload.startChatPayload(builder);
    ChatPayload.addName(builder, name);
    ChatPayload.addMessage(builder, message);
    let bufOffset = ChatPayload.endChatPayload(builder);

    sendMessage(ClientMessagePayload.ChatPayload, bufOffset);
  }, [client, builder, sendMessage, username, msg]);

  return (
    <main style={{ padding: "1rem" }}>
      <div className="inputs" style={{ display: "flex", gap: "0.25rem" }}>
        <label>
          Username:
          <input
            disabled={isAuth}
            value={username || ""}
            onChange={e => setUsername(e.target.value)}
          />
        </label>
        <label>
          Message:
          <input value={msg} onChange={e => setMsg(e.target.value)} />
        </label>
        <button disabled={isAuth} onClick={sendJoin}>
          Send Join
        </button>
        <button disabled={!isAuth} onClick={sendLeave}>
          Send Leave
        </button>
        <button onClick={sendChat}>Send Chat</button>
        <button>Send Raw Message</button>
      </div>

      <div className="responses">
        <h3 style={{ marginTop: "1rem" }}>Server Responses</h3>
        {responses.map((r, i) => (
          <p key={i}>{r}</p>
        ))}
      </div>
    </main>
  );
}

export async function getStaticProps() {
  return { props: { title: "Communications Test - Fun.Game" } };
}
