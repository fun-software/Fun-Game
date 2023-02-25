import * as React from "react";
import { Builder, ByteBuffer } from "flatbuffers";
import { Player } from "@fb/Player";
import { Vec3 } from "@fb/Math";
import { JoinPayload } from "@fb/client-messages";
import RTCClient from "@/utils/RTCClient";

export default function Page() {
  const [msg, setMsg] = React.useState<string>("");
  const responses: string[] = React.useMemo(() => [], []);
  const [client, setClient] = React.useState<RTCClient>();

  React.useEffect(() => {
    const _client = new RTCClient();
    setClient(_client);

    return () => {
      _client.channel.close();
      setClient(undefined);
    };
  }, []);

  const builder = React.useMemo(() => new Builder(1024), []);

  const player = React.useMemo(() => {
    Player.startPlayer(builder);
    Player.addHp(builder, 150);
    Player.addPos(builder, Vec3.createVec3(builder, 1, 2, 3));
    let bufOffset = Player.endPlayer(builder);

    builder.finish(bufOffset);

    let buf = new ByteBuffer(builder.asUint8Array());
    let readable = Player.getRootAsPlayer(buf);

    return builder.asUint8Array();
  }, [builder]);

  const sendJoin = React.useCallback(() => {
    if (!client) return;

    const name = builder.createString("Player1");

    JoinPayload.startJoinPayload(builder);
    JoinPayload.addName(builder, name);
    JoinPayload.endJoinPayload(builder);

    client.channel.send(builder.asUint8Array());
  }, [client, builder]);

  return (
    <main>
      <div className="inputs">
        <input value={msg} onChange={e => setMsg(e.target.value)} />
        <button onClick={sendJoin}>Send Join</button>
        <button>Send Leave</button>
        <button>Send Chat</button>
        <button>Send Invite</button>
        <button>Send Raw Message</button>
      </div>

      <div className="responses">
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
