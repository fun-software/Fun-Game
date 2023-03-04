import * as React from "react";
import { useSupabaseClient } from "@supabase/auth-helpers-react";
import styles from "../AuthModal.module.scss";
import { ModalState } from "../../Auth";

type FormDetails = {
  email: string;
  password: string;
};
type FormErrors = {
  email: string;
  password: string;
};

export default function LoginModal(props: {
  modalState: ModalState;
  setModalState: (action: ModalState) => void;
}) {
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

  /*COMPONENT FUNCTIONS*/

  function handleInputChange(e: any) {
    setFormDetails({ ...formDetails, [e.target.name]: e.target.value });
  }

  function resetForm() {
    setFormDetails({ email: "", password: "" });
  }
  function resetErrors() {
    setFormErrors({ email: "", password: "" });
  }

  function closeModal(e: any) {
    if (e.target === e.currentTarget) {
      resetForm();
      resetErrors();
      props.setModalState(ModalState.Hidden);
    }
  }

  function switchActions() {
    resetForm();
    resetErrors();
    props.setModalState(ModalState.Register);
  }

  function verifyLoginInputs() {
    let isValid = true;
    resetErrors();

    // Check if email is valid
    if (!formDetails.email.includes("@")) {
      isValid = false;
      setFormErrors(prev => ({ ...prev, email: "Invalid email" }));
    }
    if (formDetails.password === "") {
      isValid = false;
      setFormErrors(prev => ({ ...prev, password: "Required" }));
    }
    return isValid;
  }

  async function handleLogin() {
    // Check if inputs are valid
    if (!verifyLoginInputs()) return;

    // Send request to supabase
    const { error } = await supabase.auth.signInWithPassword({
      email: formDetails.email,
      password: formDetails.password,
    });
    // Check for api errors
    if (error) {
      // console.log(error);

      setFormErrors(prev => ({
        ...prev,
        email: "Invalid email or password",
        password: "Invalid email or password",
      }));
      return;
    }
    resetForm();
    resetErrors();
    props.setModalState(ModalState.Hidden);
  }
  /*COMPONENT JSX*/
  return (
    <div className={styles.authModal} onClick={closeModal}>
      <div className={styles.content}>
        <h1>Login</h1>
        <div className={styles.inputSection}>
          <label htmlFor="email">Email</label>
          <input
            type="email"
            name="email"
            id="email"
            value={formDetails.email}
            onChange={handleInputChange}
          />
          {formErrors.email && <p className={styles.error}>{formErrors.email}</p>}
        </div>
        <div className={styles.inputSection}>
          <div className={styles.passwordLabels}>
            <label htmlFor="password">Password</label>
            <a href="#" className="forgotPassword">
              Forgot Password
            </a>
          </div>
          <input
            type="password"
            name="password"
            id="password"
            value={formDetails.password}
            onChange={handleInputChange}
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
        <button onClick={switchActions} className={styles.switchActions}>
          Don't have an account?
        </button>
      </div>
    </div>
  );
}
