import React from "react";
import { Canvas } from "@react-three/fiber";
import styles from "./Scene.module.scss";

type Props = {
  eventSource: React.MutableRefObject<HTMLElement>;
} & Pick<JSX.IntrinsicElements["scene"], "children">;

export default function Scene(props: Props) {
  const { children, eventSource } = props;

  return (
    <div className={styles.canvasElem}>
      <Canvas eventPrefix="client" dpr={[1, 2]} eventSource={eventSource}>
        {children}
      </Canvas>
    </div>
  );
}
