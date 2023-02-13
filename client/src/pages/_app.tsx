import { useRef } from "react";
import Header from "@/config";
import "@/styles/global.scss";
import styles from "@/styles/Layout.module.scss";
import dynamic from "next/dynamic";

const Scene = dynamic(() => import("@/components/canvas/Scene"), { ssr: true });

export default function App({ Component, pageProps = { title: "index" } }) {
  const ref = useRef();
  return (
    <>
      <Header title={pageProps.title} />
      <div ref={ref} className={styles.domElem}>
        <Component {...pageProps} />
        {/* The canvas can either be in front of the dom or behind. If it is in front it can overlay contents.
         * Setting the event source to a shared parent allows both the dom and the canvas to receive events.
         * Since the event source is now shared, the canvas would block events, we prevent that with pointerEvents: none. */}
        {Component?.canvas && <Scene eventSource={ref}>{Component.canvas(pageProps)}</Scene>}
      </div>
    </>
  );
}
