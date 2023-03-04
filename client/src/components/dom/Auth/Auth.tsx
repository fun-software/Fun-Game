import * as React from "react";
import styles from "./Auth.module.scss";
import AuthModal from "./AuthModal";
import { useSessionContext } from "@supabase/auth-helpers-react";

export enum ModalState {
  Login = "Login",
  Register = "Register",
  Hidden = "Hidden",
}

function Auth() {
  const sessionContext = useSessionContext();
  const [modalState, setModalState] = React.useState<ModalState>(ModalState.Hidden);

  const session = sessionContext.session;
  const supabase = sessionContext.supabaseClient;

  return (
    <>
      {session === null && (
        <nav className={styles.nav}>
          <button onClick={() => setModalState(ModalState.Register)}>Register</button>
          <button onClick={() => setModalState(ModalState.Login)}>Sign In</button>
        </nav>
      )}
      {session !== null && (
        <nav className={styles.nav}>
          <p>{session.user.email}</p>
          <button onClick={() => supabase.auth.signOut()}>Sign Out</button>
        </nav>
      )}
      {modalState !== ModalState.Hidden && (
        <AuthModal modalState={modalState} setModalState={setModalState} />
      )}
    </>
  );
}

export default Auth;
