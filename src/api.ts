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
  return error instanceof Error ? error.message : String(error);
}
