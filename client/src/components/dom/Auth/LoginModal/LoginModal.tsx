import * as React from "react";
import { useSupabaseClient } from "@supabase/auth-helpers-react";
import styles from "../Auth.module.scss";
import { ModalState } from "../Auth";
import { requestLogin } from "./utils";

export type FormDetails = {
  email: string;
  password: string;
};
export type FormErrors = {
  email: string;
  password: string;
};

export function LoginModal(props: { setModalState: (state: ModalState) => void }) {
  const { setModalState } = props;
  const supabase = useSupabaseClient();

  /*COMPONENT STATE*/
  const [formDetails, setFormDetails] = React.useState<FormDetails>({
    email: "",
    password: "",
  });
  const [formErrors, setFormErrors] = React.useState<FormErrors>({
    email: "",
    password: "",
  });

  React.useEffect(() => {
    setFormDetails({ email: "", password: "" });
    setFormErrors({ email: "", password: "" });
  }, []);

  /*COMPONENT FUNCTIONS*/

  const handleLogin = React.useCallback(async () => {
    let errors = await requestLogin(formDetails, supabase);
    if (errors) {
      setFormErrors(prev => ({ ...prev, ...errors }));
      return;
    }
    setModalState(ModalState.Hidden);
  }, [formDetails, supabase, setModalState]);

  const closeModal = React.useCallback(
    (e: React.MouseEvent<HTMLElement>) => {
      if (e.target === e.currentTarget) {
        setModalState(ModalState.Hidden);
      }
    },
    [setModalState],
  );

  /*COMPONENT JSX*/
  return (
    <div className={styles.modalBackdrop} onClick={closeModal}>
      <div className={styles.content}>
        <h1>Login</h1>
        <div className={styles.inputSection}>
          <label htmlFor="email">Email</label>
          <input
            type="email"
            name="email"
            id="email"
            value={formDetails.email}
            onChange={e => {
              setFormDetails(prev => ({ ...prev, email: e.target.value }));
            }}
          />
          {formErrors.email && <p className={styles.error}>{formErrors.email}</p>}
        </div>
        <div className={styles.inputSection}>
          <div className={styles.passwordLabels}>
            <label htmlFor="password">Password</label>
            <a href="#">Forgot Password</a>
          </div>
          <input
            type="password"
            name="password"
            id="password"
            value={formDetails.password}
            onChange={e => {
              setFormDetails(prev => ({ ...prev, password: e.target.value }));
            }}
          />
          {formErrors.password && <p className={styles.error}>{formErrors.password}</p>}
        </div>
        <div className={styles.buttonWrapper}>
          <button onClick={closeModal}>Cancel</button>
          <button className={styles.actionButton} onClick={handleLogin}>
            Sign in
          </button>
        </div>
        {/* Option to switch between login and register */}
        <button
          onClick={() => {
            setModalState(ModalState.Register);
          }}
          className={styles.switchActions}
        >
          {"Don't have an account?"}
        </button>
      </div>
    </div>
  );
}
