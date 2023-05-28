import { InGameScene } from "@/components/canvas/InGameScene";
import { Game } from "@/components/dom/Game";

// Dom components go here
export default function Page() {
  return (
    <>
      <Game />
    </>
  );
}

// Canvas components go here
// It will receive same props as the Page component (from getStaticProps, etc.)
Page.scene = () => <InGameScene />;

export async function getStaticProps() {
  return { props: { title: "In Game - Fun.Game" } };
}
