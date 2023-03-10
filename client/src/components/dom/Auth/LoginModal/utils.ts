import type { FormDetails, FormErrors } from "./LoginModal";
import { SupabaseClient } from "@supabase/supabase-js";

function verifyLoginInputs(formDetails: FormDetails) {
  let hasErrors = false;
  let errors: FormErrors = {email: "", password: ""}

  // Check if email is valid
  if (!formDetails.email.includes("@")) {
    hasErrors = true;
    errors.email = "Invalid email";
  }
  if (formDetails.email === "") {
    hasErrors = true;
    errors.email = "Email cannot be empty";
  }
  if (formDetails.password === "") {
    hasErrors = true;
    errors.password = "Password cannot be empty";
  }

  if (hasErrors) return errors;
  return undefined;
}

export async function requestLogin(formDetails: FormDetails, supabase:SupabaseClient): Promise<FormErrors> {
  // Check if inputs are valid
  let formErrors = verifyLoginInputs(formDetails);
  if (formErrors) return formErrors;

  // Send request to supabase
  const { error } = await supabase.auth.signInWithPassword({
    email: formDetails.email,
    password: formDetails.password,
  });
  // Check for api errors
  if (error) {
    // console.log(error);

    formErrors = {email: "Invalid email or password", password: "Invalid email or password"}
    return formErrors;
  }
  return undefined;
}