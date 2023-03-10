import { Lobby } from "@/components/dom/Lobby";

// Dom components go here
export default function Page() {
  return (
    <>
      <Lobby />
    </>
  );
}

// Canvas components go here
// It will receive same props as the Page component (from getStaticProps, etc.)
Page.canvas = () => <></>;

export async function getStaticProps() {
  return { props: { title: "Lobby - Fun.Game" } };
}
