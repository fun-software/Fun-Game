class RTCClient {
  private game_id: string;
  public channel: RTCDataChannel;

  constructor(game_id: string) {
    this.game_id = game_id;
    let peer = new RTCPeerConnection();

    let channel = peer.createDataChannel("data", {
      ordered: false,
      maxRetransmits: 0,
    });
    channel.binaryType = "arraybuffer";

    channel.onerror = e => {
      console.warn("data channel error:", e);
    };

    peer.onicecandidate = e => {
      if (e.candidate) {
        console.log("ice candidate:", e.candidate);
      } else {
        console.log("ice candidate complete");
      }
    };

    peer
      .createOffer()
      .then(offer => {
        return peer.setLocalDescription(offer);
      })
      .then(() => {
        let req = new XMLHttpRequest();
        req.open("POST", "http://localhost:8080/offer");
        req.onload = () => {
          if (req.status == 200) {
            let res = JSON.parse(req.responseText);
            peer
              .setRemoteDescription(new RTCSessionDescription(res.answer))
              .then(() => {
                let candidate = new RTCIceCandidate(res.candidate);
                peer
                  .addIceCandidate(candidate)
                  .then(() => {
                    console.log("add ice candidate success");
                  })
                  .catch(function (err) {
                    console.log("error during 'addIceCandidate':", err);
                  });
              })
              .catch(function (e) {
                console.log("error during 'setRemoteDescription':", e);
              });
          }
        };

        const body = JSON.stringify({ sdp: peer.localDescription.sdp, game_id: this.game_id });
        req.send(body);
      })
      .catch(err => {
        console.warn("create offer error:", err);
      });

    this.channel = channel;
  }
}

export default RTCClient;
