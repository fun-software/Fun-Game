import * as React from "react";

const context = new AudioContext();
const gainNode = context.createGain();

export default async function Page() {
  const audioElementRef = React.useRef<HTMLAudioElement>(null);
  const playButtonRef = React.useRef<HTMLButtonElement>(null);

  await context.audioWorklet.addModule("audio/lib/add-noise-processor.js");
  const addNoiseNode = React.useMemo(
    () => new AudioWorkletNode(context, "add-noise-processor"),
    [],
  );

  // wire up the nodes
  const track = context.createMediaElementSource(audioElementRef.current);
  track.connect(addNoiseNode);
  addNoiseNode.connect(gainNode);
  gainNode.connect(context.destination);

  const playButtonClickHandler = () => {
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
  };

  const noiseAmountHandler = e => {
    const noiseAmount = e.target;
    const noiseAmountValue = noiseAmount.value;
    // get the noiseAmount parameter from the addNoiseNode
    const paramNoiseAmount = addNoiseNode.parameters.get("noiseAmount");
    paramNoiseAmount.setValueAtTime(noiseAmountValue, 0);
  };

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
          id="mainGain"
          min="0"
          max="1"
          value="0.2"
          step="0.01"
        />
        <label htmlFor="mainGain">mainGain</label>

        <input
          onChange={noiseAmountHandler}
          type="range"
          id="noiseAmount"
          min="0"
          max="1"
          value="0.5"
          step="0.01"
        />
        <label htmlFor="noiseAmount">noiseAmount</label>
      </div>
    </div>
  );
}
