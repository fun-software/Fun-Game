import { API_URL } from "@/utils/env";
import { GameStateT, GameState } from "@fb/GameState";
import { ByteBuffer } from "flatbuffers";
import { useCallback, useEffect, useRef, useState } from "react";

export function useGameState(game_id: string) {
  const state = useRef<GameStateT>(new GameStateT());
  const [peer, setPeer] = useState<RTCPeerConnection>(undefined);
  const [channel, setChannel] = useState<RTCDataChannel>(undefined);

  const handleMessage = useCallback((e: MessageEvent<unknown>) => {
    console.log(e);
    if (e.data instanceof Blob) {
      let blob: Blob = e.data;

      blob.arrayBuffer().then((arr) => {
        let buf = new ByteBuffer(new Uint8Array(arr));
        let msg = GameState.getRootAsGameState(buf);

        msg.unpackTo(state.current);
      });
    }
  }, []);

  // once channel is established, listen for messages
  useEffect(() => {
    if (!channel) return;

    channel.onmessage = handleMessage;
  }, [channel, handleMessage]);

  // initial connection and state sync
  useEffect(() => {
    const _peer = new RTCPeerConnection();
    const _channel = _peer.createDataChannel("data", {
      ordered: false,
      maxRetransmits: 0,
    });

    _channel.binaryType = "arraybuffer";
    _channel.onerror = (e) => {
      console.warn("data channel error:", e);
    };

    _peer
      .createOffer()
      .then((offer) => _peer.setLocalDescription(offer))
      .then(() => {
        let req = new XMLHttpRequest();
        req.open("POST", `${API_URL}/offer`);
        req.onload = () => {
          if (req.status == 200) {
            let res = JSON.parse(req.responseText);
            _peer
              .setRemoteDescription(new RTCSessionDescription(res.answer))
              .then(() => _peer.addIceCandidate(new RTCIceCandidate(res.candidate)));
          }
        };

        const body = JSON.stringify({ sdp: _peer.localDescription.sdp, game_id: this.game_id });
        req.send(body);
      })
      .catch((err) => {
        console.warn("create offer error:", err);
      });

    setPeer(_peer);
    setChannel(_channel);

    return () => {
      _channel.close();
      _peer.close();
    };
  }, []);

  return { state: state.current, peer, channel };
}
