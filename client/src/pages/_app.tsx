import { useRef, useState } from "react";
import Header from "@/config";
import "@/styles/global.scss";
import styles from "@/styles/Layout.module.scss";
import dynamic from "next/dynamic";
import { createBrowserSupabaseClient } from "@supabase/auth-helpers-nextjs";
import { SessionContextProvider, Session } from "@supabase/auth-helpers-react";

let supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_PROJECT_URL || "localhost";
let supabaseKey = process.env.NEXT_PUBLIC_SUPABASE_API_KEY || "nokey";

const Scene = dynamic(() => import("@/components/canvas/Scene"), { ssr: true });

type Props = { Component: any; pageProps: { title: string; initialSession: Session } };
export default function App({ Component, pageProps }: Props) {
  const [supabaseClient] = useState(() =>
    createBrowserSupabaseClient({ supabaseKey, supabaseUrl }),
  );

  const ref = useRef();
  return (
    <SessionContextProvider
      supabaseClient={supabaseClient}
      initialSession={pageProps.initialSession}
    >
      <Header title={pageProps.title} />
      <div ref={ref} className={styles.domElem}>
        <Component {...pageProps} />

        {Component?.canvas && <Scene eventSource={ref}>{Component.canvas(pageProps)}</Scene>}
      </div>
    </SessionContextProvider>
  );
}
