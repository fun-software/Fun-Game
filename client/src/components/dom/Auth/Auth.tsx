import { useState } from "react";
import styles from "./Auth.module.scss";
import AuthModal from "./AuthModal";
import { useSessionContext } from "@supabase/auth-helpers-react";

export enum AuthAction {
  Login = "Login",
  Logout = "Logout",
  Register = "Register",
  Idle = "Idle",
}

function Auth() {
  const sessionContext = useSessionContext();
  const [authAction, setAuthAction] = useState<AuthAction>(AuthAction.Idle);

  const session = sessionContext.session;

  return (
    <>
      {session === null && (
        <nav className={styles.nav}>
          <button onClick={() => setAuthAction(AuthAction.Register)}>Register</button>
          <button onClick={() => setAuthAction(AuthAction.Login)}>Sign In</button>
        </nav>
      )}
      {session && (
        <nav className={styles.nav}>
          <button onClick={() => setAuthAction(AuthAction.Logout)}>{session.user.email}</button>
        </nav>
      )}
      {authAction !== AuthAction.Idle && (
        <AuthModal authAction={authAction} setAuthAction={setAuthAction} />
      )}
    </>
  );
}

export default Auth;
