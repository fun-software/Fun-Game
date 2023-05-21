import { Environment } from "@react-three/drei";
import { Camera } from "./Camera";
import { Floor } from "./Floor";
import { Controls } from "./Controls";

export function InGameScene() {
  return (
    <group>
      <fog attach="fog" args={[0xffffff, 0, 100]} />

      <Camera />
      <Controls />

      <mesh position={[-2, 0.5, -5]}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color={0x00ffff} />
      </mesh>
      <Floor />

      <Environment preset="warehouse" background blur={1} />
    </group>
  );
}
