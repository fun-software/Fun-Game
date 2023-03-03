import { useState } from "react";
import { useSupabaseClient } from "@supabase/auth-helpers-react";
import styles from "./AuthModal.module.scss";
import { AuthAction } from "../Auth";

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
export default function AuthModal(props: {
  authAction: AuthAction;
  setAuthAction: (action: AuthAction) => void;
}) {
  /*COMPONENT STATE*/
  const [formDetails, setFormDetails] = useState<FormDetails>({
    email: "",
    password: "",
    confirm: "",
  });

  const [formErrors, setFormErrors] = useState<FormErrors>({
    email: "",
    password: "",
    confirm: "",
  });
  const supabase = useSupabaseClient();

  if (!supabase) return;

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

  function switchActions() {
    resetForm();
    resetErrors();
    props.setAuthAction(
      props.authAction === AuthAction.Login ? AuthAction.Register : AuthAction.Login,
    );
  }

  function closeModal(e: any) {
    if (e.target === e.currentTarget) {
      resetForm();
      props.setAuthAction(AuthAction.Idle);
    }
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

  async function handleLogout() {
    const error = await supabase.auth.signOut();
    props.setAuthAction(AuthAction.Idle);
  }

  async function handleLogin() {
    // Check if inputs are valid
    if (!verifyLoginInputs()) return;

    // Send request to supabase
    const { data, error } = await supabase.auth.signInWithPassword({
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
    props.setAuthAction(AuthAction.Idle);
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
    props.setAuthAction(AuthAction.Idle);
  }

  /*COMPONENT JSX*/
  if (props.authAction === "Logout") {
    // Logout Modal
    return (
      <div className={styles.authModal} onClick={closeModal}>
        <div className={styles.content}>
          <h1>Log out?</h1>
          <div className={styles.buttonWrapper}>
            <button onClick={handleLogout} className={styles.dangerButton}>
              Log Out
            </button>
            <button onClick={closeModal}>Cancel</button>
          </div>
        </div>
      </div>
    );
  } else {
    // Login/Register Modal
    return (
      <div className={styles.authModal} onClick={closeModal}>
        <div className={styles.content}>
          <h1>{props.authAction}</h1>
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
              {/* Optional forgot password link on login */}
              {props.authAction === AuthAction.Login && (
                <a href="#" className="forgotPassword">
                  Forgot Password
                </a>
              )}
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
          {/* Password Confirmation on register */}
          {props.authAction === AuthAction.Register && (
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
          )}
          <div className={styles.buttonWrapper}>
            <button onClick={closeModal}>Cancel</button>
            <button
              className={styles.actionButton}
              onClick={props.authAction === AuthAction.Login ? handleLogin : handleRegister}
            >
              {props.authAction}
            </button>
          </div>
          {/* Option to switch between login and register */}
          <button onClick={switchActions} className={styles.switchActions}>
            {props.authAction === AuthAction.Login ? "Don't" : "Already"} have an account?
          </button>
        </div>
      </div>
    );
  }
}
