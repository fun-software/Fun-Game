import { ModalState } from "../Auth";
import LoginModal from "./LoginModal";
import RegisterModal from "./RegisterModal";

export default function AuthModal(props: {
  modalState: ModalState;
  setModalState: (state: ModalState) => void;
}) {
  /*COMPONENT FUNCTIONS*/

  /*COMPONENT JSX*/
  switch (props.modalState) {
    case ModalState.Login:
      return <LoginModal modalState={props.modalState} setModalState={props.setModalState} />;
    case ModalState.Register:
      return <RegisterModal modalState={props.modalState} setModalState={props.setModalState} />;
    default:
      return <></>;
  }
}
