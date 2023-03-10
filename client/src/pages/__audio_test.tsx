import * as React from "react";
async function addModule(ctx) {
  await ctx.audioWorklet.addModule("audio/lib/add-noise-processor.js");
  const node = new AudioWorkletNode(ctx, "add-noise-processor");
  return node;
}

export default function Page() {
  const audioElementRef = React.useRef<HTMLAudioElement>(null);
  const playButtonRef = React.useRef<HTMLButtonElement>(null);

  const [context, setContext] = React.useState(null);
  const [addNoiseNode, setAddNoiseNode] = React.useState(null);
  const [track, setTrack] = React.useState(null);

  // causes all callbacks with context in their dependency array to React
  React.useEffect(() => {
    setContext(new AudioContext());
  }, []);

  // "..." with addNoiseNode to React
  React.useEffect(() => {
    if (context !== null) {
      addModule(context).then(res => {
        setAddNoiseNode(res);
      });
    }
  }, [context]);

  const gainNode = React.useMemo(() => {
    if (context) {
      // console.log(context);
      return context.createGain();
    } else return null;
  }, [context]);

  React.useEffect(() => {
    if (context && !track) {
      setTrack(context.createMediaElementSource(audioElementRef.current));
    }
  }, [context, track]);

  React.useEffect(() => {
    if (gainNode === null || addNoiseNode === null || context === null || track === null) return;
    // wire up the nodes
    track.connect(addNoiseNode);
    addNoiseNode.connect(gainNode);
    gainNode.connect(context.destination);
  }, [context, gainNode, addNoiseNode, track]);

  const playButtonClickHandler = React.useCallback(() => {
    // Check if context is in suspended state (autoplay policy)
    if (context.state === "suspended") {
      context.resume();
    }
    // Play or pause track depending on state
    if (playButtonRef.current.dataset.playing === "false") {
      audioElementRef.current.play();
      playButtonRef.current.dataset.playing = "true";
    } else if (playButtonRef.current.dataset.playing === "true") {
      audioElementRef.current.pause();
      playButtonRef.current.dataset.playing = "false";
    }
  }, [context]);

  const noiseAmountHandler = React.useCallback(
    e => {
      if (addNoiseNode === null) return;

      const noiseAmountValue = e.target.value;
      // get the noiseAmount parameter from the addNoiseNode
      const paramNoiseAmount = addNoiseNode.parameters.get("noiseAmount");
      paramNoiseAmount.setValueAtTime(noiseAmountValue, 0);
    },
    [addNoiseNode],
  );

  return (
    <div>
      <audio
        ref={audioElementRef}
        onEnded={() => {
          playButtonRef.current.dataset.playing = "false";
        }}
        src="audio/camelot_acid.wav"
      ></audio>

      <button
        ref={playButtonRef}
        onClick={playButtonClickHandler}
        data-playing="false"
        role="switch"
        aria-checked="false"
      >
        <span>Play/Pause</span>
      </button>

      <div>
        <input
          onChange={e => {
            gainNode.gain.value = parseFloat(e.target.value);
          }}
          type="range"
          min="0"
          max="1"
          defaultValue={0.2}
          step="0.01"
        />
        <label htmlFor="mainGain">mainGain</label>

        <input
          onChange={noiseAmountHandler}
          type="range"
          min="0"
          max="1"
          defaultValue={0}
          step="0.01"
        />
        <label htmlFor="noiseAmount">noiseAmount</label>
      </div>
    </div>
  );
}

export async function getStaticProps() {
  return { props: { title: "Fun.Game" } };
}
