import Header from "@/config";
import "@/styles/global.scss";
import styles from "@/styles/Layout.module.scss";
import dynamic from "next/dynamic";
import { createBrowserSupabaseClient } from "@supabase/auth-helpers-nextjs";
import { SessionContextProvider, Session, SupabaseClient } from "@supabase/auth-helpers-react";
import { useEffect, useRef, useState } from "react";

let supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_PROJECT_URL;
let supabaseKey = process.env.NEXT_PUBLIC_SUPABASE_API_KEY;

const SceneWrapper = dynamic(() => import("@/components/canvas/SceneWrapper"), { ssr: true });

type Props = { Component: any; pageProps: { title: string; initialSession: Session } };
export default function App({ Component, pageProps }: Props) {
  const ref = useRef();
  const [supabaseClient, setSupabaseClient] = useState<SupabaseClient>();

  useEffect(() => {
    if (!supabaseUrl || !supabaseKey) throw new Error("Missing supabase env vars");
    let client = createBrowserSupabaseClient({ supabaseKey, supabaseUrl });
    setSupabaseClient(client);
  }, []);

  // short circuit if supabaseClient is not ready
  if (!supabaseClient)
    return (
      <>
        <Header title={pageProps.title} />
        <div ref={ref} className={styles.domElem}>
          <Component {...pageProps} />

          {Component?.scene && (
            <SceneWrapper eventSource={ref}>{Component.scene(pageProps)}</SceneWrapper>
          )}
        </div>
      </>
    );

  return (
    <SessionContextProvider
      supabaseClient={supabaseClient}
      initialSession={pageProps.initialSession}
    >
      <Header title={pageProps.title} />
      <div ref={ref} className={styles.domElem}>
        <Component {...pageProps} />

        {Component?.scene && (
          <SceneWrapper eventSource={ref}>{Component.scene(pageProps)}</SceneWrapper>
        )}
      </div>
    </SessionContextProvider>
  );
}
