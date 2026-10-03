import type { Action } from "./types";
export async function request<T>(action: Action): Promise<T> {
  if ("__TAURI_INTERNALS__" in window) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<T>("action", { action });
  }
  const response = await fetch("http://127.0.0.1:1421/api", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(action),
  });
  const value = await response.json();
  if (!response.ok) throw new Error(value.error || "Simulation request failed");
  return value as T;
}
export function readableError(error: unknown): string {
  const detail = error instanceof Error ? error.message : String(error);
  if (/Incompatible save version/i.test(detail))
    return "This world was saved with an unsupported simulation or save version. Open it with the matching GENESIS version.";
  if (
    /checksum|decompress|invalid save|valid GENESIS|Invalid save payload/i.test(
      detail,
    )
  )
    return "This save is damaged or is not a compatible GENESIS world. Your current world has been kept.";
  if (/Cannot open save/i.test(detail))
    return "The world could not be opened. Check that the file still exists and that you have permission to read it. Your current world has been kept.";
  if (/temporary save|Cannot write save|atomically replace/i.test(detail))
    return "The world could not be saved. Choose an existing writable folder and check available disk space. Your current world and previous save have been kept.";
  return detail;
}
