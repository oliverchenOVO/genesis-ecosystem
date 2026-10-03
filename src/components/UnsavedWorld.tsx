import { useEffect, useRef } from "react";
import type { UnsavedChoice } from "../files";
export function UnsavedWorld({
  choose,
  saving,
}: {
  choose: (choice: UnsavedChoice) => void;
  saving: boolean;
}) {
  const dialog = useRef<HTMLDialogElement>(null);
  useEffect(() => {
    dialog.current?.showModal();
  }, []);
  return (
    <dialog
      ref={dialog}
      aria-labelledby="unsaved-title"
      onCancel={(e) => {
        e.preventDefault();
        if (!saving) choose("cancel");
      }}
    >
      <h2 id="unsaved-title">Save this world before leaving?</h2>
      <p className="muted">
        This world has unsaved progress. The simulation is paused while you
        decide. Discard continues without saving; autosaves remain available.
      </p>
      <div className="unsaved-actions">
        <button disabled={saving} onClick={() => choose("cancel")}>
          Cancel
        </button>
        <button disabled={saving} onClick={() => choose("discard")}>
          Discard
        </button>
        <button
          className="primary"
          disabled={saving}
          onClick={() => choose("save")}
        >
          Save
        </button>
      </div>
    </dialog>
  );
}
