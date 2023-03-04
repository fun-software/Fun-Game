import * as React from "react";
import { useSupabaseClient } from "@supabase/auth-helpers-react";
import styles from "../AuthModal.module.scss";
import { ModalState } from "../../Auth";

type FormDetails = {
  email: string;
  password: string;
  confirm: string;
};
type FormErrors = {
  email: string;
  password: string;
  confirm: string;
};

export default function RegisterModal(props: {
  modalState: ModalState;
  setModalState: (action: ModalState) => void;
}) {
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
  /*COMPONENT FUNCTIONS*/

  function handleInputChange(e: any) {
    setFormDetails({ ...formDetails, [e.target.name]: e.target.value });
  }

  function resetForm() {
    setFormDetails({ email: "", password: "", confirm: "" });
  }

  function resetErrors() {
    setFormErrors({ email: "", password: "", confirm: "" });
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
    props.setModalState(ModalState.Login);
  }

  function verifyRegisterInputs() {
    let isValid = true;
    resetErrors();

    // Check password length
    if (formDetails.password.length < 6) {
      isValid = false;
      setFormErrors(prev => ({ ...prev, password: "Password must be at least 6 characters" }));
    }
    // Check if passwords match
    if (formDetails.password !== formDetails.confirm) {
      isValid = false;
      setFormErrors(prev => ({ ...prev, confirm: "Passwords do not match" }));
    }
    // Check if email is valid
    if (!formDetails.email.includes("@")) {
      isValid = false;
      setFormErrors(prev => ({ ...prev, email: "Invalid email" }));
    }
    // Check if any fields are empty
    for (const key in formDetails) {
      if (formDetails[key] === "") {
        isValid = false;
        setFormErrors(prev => ({ ...prev, [key]: "Required" }));
      }
    }
    return isValid;
  }

  async function handleRegister() {
    // Check if inputs are valid
    if (!verifyRegisterInputs()) return;

    // Send request to supabase
    const { data, error } = await supabase.auth.signUp({
      email: formDetails.email,
      password: formDetails.password,
    });
    // Check for api errors
    if (error) {
      // console.log(error);
      return;
    }
    // Check if account already exists
    if (!data.user.identities.length) {
      setFormErrors(prev => ({ ...prev, email: "Account already exists" }));
      return;
    }
    // Account created, needs email verification
    resetForm();
    resetErrors();
    props.setModalState(ModalState.Hidden);
  }
  /*COMPONENT JSX*/
  return (
    <div className={styles.authModal} onClick={closeModal}>
      <div className={styles.content}>
        <h1>Register</h1>
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
        <div className={styles.inputSection}>
          <label htmlFor="confirm">Confirm Password</label>
          <input
            type="password"
            name="confirm"
            id="confirm"
            value={formDetails.confirm}
            onChange={handleInputChange}
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
        <button onClick={switchActions} className={styles.switchActions}>
          Already have an account?
        </button>
      </div>
    </div>
  );
}
