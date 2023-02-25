class RTCClient {
  public channel: RTCDataChannel;

  constructor() {
    let peer = new RTCPeerConnection();

    let channel = peer.createDataChannel("data", {
      ordered: false,
      maxRetransmits: 0,
    });
    channel.binaryType = "arraybuffer";

    channel.onerror = e => {
      console.warn("data channel error:", e);
    };

    channel.onmessage = e => {
      let bytes = new Uint8Array(e.data);
      console.log("data channel message: ", bytes.toString());
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
        req.open("POST", "/offer");
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
        req.send(peer.localDescription.sdp);
      })
      .catch(err => {
        console.warn("create offer error:", err);
      });

    this.channel = channel;
  }
}

export default RTCClient;
