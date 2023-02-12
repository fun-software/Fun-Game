import { OrbitControls } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import React from "react";

export default function Temp() {
  const ref = React.useRef(null);

  useFrame(({ clock }) => {
    ref.current.rotation.y = clock.elapsedTime;
  });

  return (
    <>
      <OrbitControls enableZoom={false} />
      <mesh ref={ref}>
        <boxBufferGeometry args={[1, 1, 1]} />
        <meshNormalMaterial />
      </mesh>
    </>
  );
}
