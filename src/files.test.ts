import { describe, expect, it, vi } from "vitest";
import { chooseSavePath, genesisPath, protectUnsaved } from "./files";
import { readableError } from "./api";
import type { FileStatus } from "./types";
const files: FileStatus = {
  dirty: true,
  current_path: null,
  warning: null,
  recent: [],
};
describe("native dialog application logic", () => {
  it("adds the extension and preserves case-insensitive GENESIS files", () => {
    expect(genesisPath("world")).toBe("world.genesis");
    expect(genesisPath("world.GENESIS")).toBe("world.GENESIS");
    expect(genesisPath("a.txt")).toBe("a.txt.genesis");
    expect(() => genesisPath(" ")).toThrow();
  });
  it("Save reuses an existing world path without opening a dialog", async () => {
    const dialogs = { save: vi.fn(), open: vi.fn() };
    expect(
      await chooseSavePath(
        { ...files, current_path: "C:/world.genesis" },
        false,
        dialogs,
      ),
    ).toBe("C:/world.genesis");
    expect(dialogs.save).not.toHaveBeenCalled();
  });
  it("Save As opens the dialog with the current path", async () => {
    const dialogs = {
      save: vi.fn().mockResolvedValue("D:/new"),
      open: vi.fn(),
    };
    expect(
      await chooseSavePath(
        { ...files, current_path: "C:/old.genesis" },
        true,
        dialogs,
      ),
    ).toBe("D:/new.genesis");
    expect(dialogs.save).toHaveBeenCalledWith("C:/old.genesis");
  });
  it("cancelled native Save returns null without a default-file fallback", async () => {
    const dialogs = { save: vi.fn().mockResolvedValue(null), open: vi.fn() };
    expect(await chooseSavePath(files, false, dialogs)).toBeNull();
  });
});
function ports(
  choice: "save" | "discard" | "cancel",
  saved = true,
  running = true,
  dirty = true,
) {
  return {
    pause: vi.fn().mockResolvedValue({ running, speed: 0, dirty }),
    confirm: vi.fn().mockResolvedValue(choice),
    save: vi.fn().mockResolvedValue(saved),
    resume: vi.fn().mockResolvedValue(undefined),
  };
}
describe("unsaved guard", () => {
  it("Cancel keeps the world and restores MAX playback", async () => {
    const p = ports("cancel");
    const next = vi.fn();
    expect(await protectUnsaved(p, next)).toBe(false);
    expect(next).not.toHaveBeenCalled();
    expect(p.resume).toHaveBeenCalledWith(0);
  });
  it("Discard proceeds without saving", async () => {
    const p = ports("discard");
    const next = vi.fn().mockResolvedValue(true);
    expect(await protectUnsaved(p, next)).toBe(true);
    expect(p.save).not.toHaveBeenCalled();
    expect(p.resume).not.toHaveBeenCalled();
  });
  it("Save completes before replacing or closing the world", async () => {
    const p = ports("save");
    const next = vi.fn().mockResolvedValue(true);
    expect(await protectUnsaved(p, next)).toBe(true);
    expect(p.save.mock.invocationCallOrder[0]).toBeLessThan(
      next.mock.invocationCallOrder[0],
    );
  });
  it("cancelled or failed Save stops the destructive operation", async () => {
    const p = ports("save", false);
    const next = vi.fn();
    expect(await protectUnsaved(p, next)).toBe(false);
    expect(next).not.toHaveBeenCalled();
    expect(p.resume).toHaveBeenCalled();
  });
  it("failed Load restores playback for the retained world", async () => {
    const p = ports("discard");
    expect(await protectUnsaved(p, async () => false)).toBe(false);
    expect(p.resume).toHaveBeenCalledWith(0);
  });
  it("a paused world stays paused when cancelling", async () => {
    const p = ports("cancel", true, false);
    await protectUnsaved(p, async () => true);
    expect(p.resume).not.toHaveBeenCalled();
  });
  it("clean world skips confirmation after the fresh pause response", async () => {
    const p = ports("cancel", true, true, false);
    expect(await protectUnsaved(p, async () => true)).toBe(true);
    expect(p.confirm).not.toHaveBeenCalled();
  });
  it("exceptions preserve the world and restore playback", async () => {
    const p = ports("discard");
    await expect(
      protectUnsaved(p, async () => {
        throw Error("unavailable");
      }),
    ).rejects.toThrow("unavailable");
    expect(p.resume).toHaveBeenCalledWith(0);
  });
});
it("save error messages explain recovery without presenting raw OS diagnostics", () => {
  for (const message of [
    "Save checksum or size mismatch",
    "Cannot decompress save: invalid",
    "Not a valid GENESIS save file",
  ])
    expect(readableError(message)).toContain("current world has been kept");
  expect(readableError("Incompatible save version at offset 12")).toContain(
    "matching GENESIS version",
  );
  expect(
    readableError("Cannot create temporary save: access denied"),
  ).toContain("writable folder");
  expect(readableError("Cannot open save: missing")).toContain("still exists");
  expect(readableError(Error("Other error"))).toBe("Other error");
});
