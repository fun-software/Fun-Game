import * as React from "react";
import { useSessionContext, useSupabaseClient } from "@supabase/auth-helpers-react";
import styles from "./Nav.module.scss";
import { ModalState } from "../Auth";

export function Nav(props: { modalState: ModalState; setModalState: (state: ModalState) => void }) {
  const sessionContext = useSessionContext();
  const session = sessionContext.session;
  const supabase = sessionContext.supabaseClient;

  return (
    <>
      {session === null && (
        <nav className={styles.nav}>
          <button onClick={() => props.setModalState(ModalState.Register)}>Register</button>
          <button onClick={() => props.setModalState(ModalState.Login)}>Sign In</button>
        </nav>
      )}
      {session !== null && (
        <nav className={styles.nav}>
          <p>{session.user.email}</p>
          <button onClick={() => supabase.auth.signOut()}>Sign Out</button>
        </nav>
      )}
    </>
  );
}
