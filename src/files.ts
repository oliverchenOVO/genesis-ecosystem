import type { FileStatus } from "./types";
export interface DialogService {
  save(defaultPath: string): Promise<string | null>;
  open(): Promise<string | null>;
}
export function genesisPath(path: string): string {
  if (!path.trim()) throw new Error("Choose a file name for your world.");
  return /\.genesis$/i.test(path) ? path : `${path}.genesis`;
}
export const nativeDialogs: DialogService = {
  async save(defaultPath) {
    if (!("__TAURI_INTERNALS__" in window))
      throw new Error(
        "Open the Windows desktop application to choose a save file.",
      );
    const { save } = await import("@tauri-apps/plugin-dialog");
    return save({
      title: "Save GENESIS world",
      defaultPath,
      filters: [{ name: "GENESIS world", extensions: ["genesis"] }],
    });
  },
  async open() {
    if (!("__TAURI_INTERNALS__" in window))
      throw new Error("Open the Windows desktop application to load a world.");
    const { open } = await import("@tauri-apps/plugin-dialog");
    const path = await open({
      title: "Load GENESIS world",
      multiple: false,
      directory: false,
      filters: [{ name: "GENESIS world", extensions: ["genesis"] }],
    });
    return typeof path === "string" ? path : null;
  },
};
export async function chooseSavePath(
  files: FileStatus,
  saveAs: boolean,
  dialogs: DialogService,
): Promise<string | null> {
  if (!saveAs && files.current_path) return files.current_path;
  const path = await dialogs.save(files.current_path ?? "world.genesis");
  return path === null ? null : genesisPath(path);
}
export type UnsavedChoice = "save" | "discard" | "cancel";
interface GuardPorts {
  pause(): Promise<{ running: boolean; speed: number; dirty: boolean }>;
  confirm(): Promise<UnsavedChoice>;
  save(): Promise<boolean>;
  resume(speed: number): Promise<void>;
}
/** Pause first, then read the worker's fresh marker, so MAX cannot race a save decision. */
export async function protectUnsaved(
  ports: GuardPorts,
  proceed: () => Promise<boolean>,
): Promise<boolean> {
  const original = await ports.pause();
  let completed = false;
  try {
    if (original.dirty) {
      const choice = await ports.confirm();
      if (choice === "cancel") return false;
      if (choice === "save" && !(await ports.save())) return false;
    }
    completed = await proceed();
    return completed;
  } finally {
    if (!completed && original.running) await ports.resume(original.speed);
  }
}
