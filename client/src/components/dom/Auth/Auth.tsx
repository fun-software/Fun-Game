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

export function Auth() {
  const [modalState, setModalState] = React.useState<ModalState>(ModalState.Hidden);

  return (
    <>
      <Nav modalState={modalState} setModalState={setModalState} />

      {modalState === ModalState.Login && <LoginModal setModalState={setModalState} />}
      {modalState === ModalState.Register && <RegisterModal setModalState={setModalState} />}
      {modalState === ModalState.Confirm && <ConfirmModal setModalState={setModalState} />}
      {modalState === ModalState.Hidden && <></>}
    </>
  );
}
