export function Floor() {
  return (
    <mesh rotation-x={-Math.PI / 2}>
      <planeGeometry args={[500, 500]} />
      <meshStandardMaterial color={0x6655ff} />
    </mesh>
  );
}
