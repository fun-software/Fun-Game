import * as React from "react";
import { useSupabaseClient } from "@supabase/auth-helpers-react";
import styles from "../Auth.module.scss";
import { ModalState } from "../Auth";
import { requestRegister } from "./utils";

export type FormDetails = {
  email: string;
  password: string;
  confirm: string;
};
export type FormErrors = {
  email: string;
  password: string;
  confirm: string;
};

export function RegisterModal(props: { setModalState: (state: ModalState) => void }) {
  const { setModalState } = props;
  const supabase = useSupabaseClient();
  /*COMPONENT STATE*/
  const [formDetails, setFormDetails] = React.useState<FormDetails>({
    email: "",
    password: "",
    confirm: "",
  });
  const [formErrors, setFormErrors] = React.useState<FormErrors>({
    email: "",
    password: "",
    confirm: "",
  });

  React.useEffect(() => {
    setFormDetails({ email: "", password: "", confirm: "" });
    setFormErrors({ email: "", password: "", confirm: "" });
  }, []);

  /*COMPONENT FUNCTIONS*/

  const handleRegister = React.useCallback(async () => {
    let formErrors = await requestRegister(formDetails, supabase);
    if (formErrors) {
      setFormErrors(prev => ({ ...prev, ...formErrors }));
      return;
    }
    setModalState(ModalState.Confirm);
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
        <h1>Register</h1>
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
            onKeyDown={e => {
              if (e.key === "Enter") handleRegister();
            }}
          />
          {formErrors.email && <p className={styles.error}>{formErrors.email}</p>}
        </div>
        <div className={styles.inputSection}>
          <div className={styles.passwordLabels}>
            <label htmlFor="password">Password</label>
          </div>
          <input
            type="password"
            name="password"
            id="password"
            value={formDetails.password}
            onChange={e => {
              setFormDetails(prev => ({ ...prev, password: e.target.value }));
            }}
            onKeyDown={e => {
              if (e.key === "Enter") handleRegister();
            }}
          />
          {formErrors.password && <p className={styles.error}>{formErrors.password}</p>}
        </div>
        <div className={styles.inputSection}>
          <label htmlFor="confirm">Confirm Password</label>
          <input
            type="password"
            name="confirm"
            id="confirm"
            value={formDetails.confirm}
            onChange={e => {
              setFormDetails(prev => ({ ...prev, confirm: e.target.value }));
            }}
            onKeyDown={e => {
              if (e.key === "Enter") handleRegister();
            }}
          />
          {formErrors.confirm && <p className={styles.error}>{formErrors.confirm}</p>}
        </div>
        <div className={styles.buttonWrapper}>
          <button onClick={closeModal}>Cancel</button>
          <button className={styles.actionButton} onClick={handleRegister}>
            Sign Up
          </button>
        </div>
        {/* Option to switch between login and register */}
        <button
          onClick={() => {
            setModalState(ModalState.Login);
          }}
          className={styles.switchActions}
        >
          Already have an account?
        </button>
      </div>
    </div>
  );
}
