import { useEffect, useRef } from "react";
import type { FileStatus } from "../types";
export function RecentWorlds({
  files,
  load,
  remove,
  close,
  busy,
}: {
  files: FileStatus;
  load: (path: string) => void;
  remove: (path: string) => void;
  close: () => void;
  busy: boolean;
}) {
  const dialog = useRef<HTMLDialogElement>(null);
  useEffect(() => {
    dialog.current?.showModal();
  }, []);
  return (
    <dialog ref={dialog} aria-labelledby="recent-title" onCancel={close}>
      <div className="view-heading">
        <h2 id="recent-title">Recent worlds</h2>
        <button disabled={busy} onClick={close}>
          Close
        </button>
      </div>
      {files.recent.length === 0 ? (
        <p className="muted">Saved and loaded worlds will appear here.</p>
      ) : (
        <ul className="recent-worlds">
          {files.recent.map((file) => (
            <li key={file.path}>
              <button
                disabled={busy || !file.available}
                onClick={() => load(file.path)}
              >
                <strong>{file.path.split(/[\\/]/).pop()}</strong>
                <span>{file.path}</span>
                <small>
                  {file.available
                    ? new Date(file.last_opened_ms).toLocaleString()
                    : "Unavailable — file moved or missing"}
                </small>
              </button>
              <button
                disabled={busy}
                aria-label={`Remove ${file.path} from recent worlds`}
                onClick={() => remove(file.path)}
              >
                Remove
              </button>
            </li>
          ))}
        </ul>
      )}
      <p className="muted">Removing an entry keeps the saved world on disk.</p>
    </dialog>
  );
}
