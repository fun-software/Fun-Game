import * as React from "react";
import LoginModal from "./LoginModal";
import RegisterModal from "./RegisterModal";
import ConfirmModal from "./ConfirmModal";
import Nav from "./Nav";

export enum ModalState {
  Login = "Login",
  Register = "Register",
  Confirm = "Confirm",
  Hidden = "Hidden",
}

function Auth() {
  const [modalState, setModalState] = React.useState<ModalState>(ModalState.Hidden);

  return (
    <>
      <Nav modalState={modalState} setModalState={setModalState} />

      {modalState === ModalState.Login && (
        <LoginModal modalState={modalState} setModalState={setModalState} />
      )}
      {modalState === ModalState.Register && (
        <RegisterModal modalState={modalState} setModalState={setModalState} />
      )}
      {modalState === ModalState.Confirm && (
        <ConfirmModal modalState={modalState} setModalState={setModalState} />
      )}
      {modalState === ModalState.Hidden && <></>}
    </>
  );
}

export default Auth;
