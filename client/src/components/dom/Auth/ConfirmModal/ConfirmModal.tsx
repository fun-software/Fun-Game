import * as React from "react";
import { ModalState } from "../Auth";
import styles from "../Auth.module.scss";

export function ConfirmModal(props: { setModalState: (state: ModalState) => void }) {
  const { setModalState } = props;
  function closeModal(e: React.MouseEvent<HTMLElement>) {
    if (e.target === e.currentTarget) {
      setModalState(ModalState.Hidden);
    }
  }

  return (
    <div className={styles.modalBackdrop} onClick={closeModal}>
      <div className={styles.content}>
        <h2>Confirm your email</h2>
        <p>A confirmation email has been sent to your email address.</p>
        <button className={styles.actionButton} onClick={closeModal}>
          Close
        </button>
      </div>
    </div>
  );
}
