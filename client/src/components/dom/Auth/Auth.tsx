import * as React from "react";
import { useSupabaseClient } from "@supabase/auth-helpers-react";
import styles from "./Auth.module.scss";
import { SignInWithPasswordCredentials } from "@supabase/supabase-js";

function Auth() {
  const supabase = useSupabaseClient();

  const handleRegister = React.useCallback(() => {
    if (!supabase) return;

    let creds: SignInWithPasswordCredentials = {
      email: "myass@gmail.com",
      password: "myass24",
    };

    supabase.auth
      .signUp(creds)
      .then(res => {
        //console.log(res);
      })
      .catch(err => {
        //console.warn(err)
      });
  }, [supabase]);

  return (
    <nav className={styles.nav}>
      <button>Login</button>
      <button onClick={handleRegister}>Register</button>
    </nav>
  );
}

export default Auth;
