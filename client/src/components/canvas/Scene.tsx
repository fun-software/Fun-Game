import React from "react";
import { Canvas } from "@react-three/fiber";

type Props = {
  eventSource: React.MutableRefObject<HTMLElement>;
  children: React.ReactElement[];
};

export default function Scene(props: Props) {
  const { children, eventSource } = props;

  return (
    <div id="canvasElem">
      <Canvas eventPrefix="client" dpr={[1, 2]} eventSource={eventSource}>
        {children}
      </Canvas>
    </div>
  );
}
