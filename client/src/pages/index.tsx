import Auth from "@/components/dom/Auth";
import MainMenu from "@/components/dom/MainMenu";
import dynamic from "next/dynamic";

// Dynamic import is used to prevent a payload when the website starts, that includes threejs, r3f etc..
// WARNING ! errors might get obfuscated by using dynamic import.
// If something goes wrong go back to a static import to show the error.
// https://github.com/pmndrs/react-three-next/issues/49
const Temp = dynamic(() => import("@/components/canvas/Temp"), { ssr: false });

// Dom components go here
export default function Page() {
  return (
    <>
      <Auth />
      <MainMenu />
    </>
  );
}

// Canvas components go here
// It will receive same props as the Page component (from getStaticProps, etc.)
Page.canvas = () => <Temp />;

export async function getStaticProps() {
  return { props: { title: "Fun.Game" } };
}
