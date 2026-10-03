# GENESIS binary save v1

Single `.genesis` file: 8-byte magic `GENESIS1`, three little-endian u32 versions (save/simulation/RNG), u64 uncompressed length, BLAKE3 checksum (32 bytes), then zlib-compressed fixed-width bincode state. Header is 60 bytes. Full world, ancestry, registries, RNG streams, configuration, command history, telemetry and event history are retained.

Decode rejects incompatible versions, truncated or corrupt data, checksum mismatch, excessive compressed/uncompressed size (>128 MiB), malformed bincode, invalid configuration and world invariants. A failed load does not replace the running world. No implicit migration is attempted in Phase 1.

Atomic save writes to a unique temporary file in the destination directory, synchronizes it, then uses tempfile's atomic persistence/replacement. Failed writes preserve the previous destination. Manual save/load are mandatory; rotating autosave is implemented in the desktop application layer.

Phase 1.1 retains save v1 / simulation v4 / RNG v1 compatibility. Native Save asks for a filename when the session has no current path; later Save reuses it. Save As always opens the supported Tauri dialog. Save appends `.genesis` when missing; case-insensitive existing `.GENESIS` is preserved. Load dialogs filter GENESIS files, but the decoder remains authoritative even when a renamed/corrupt file passes the picker filter.

Successful manual Save or Load stores a separate application marker `(tick, command count)`. Tick advancement or a same-tick environmental command makes the session dirty. Autosave does not clear that marker. Before New/Load/Close, Save/Discard/Cancel protects unsaved progress. Failed load preserves both current world and current path. Failed save preserves the world, earlier file and marker. Technical errors are logged; the frontend explains incompatible versions, damaged data, missing files and writable-folder recovery.

Recent paths and last-opened timestamps are held in atomic `worlds/preferences.json`, capped at five. They are not part of the `.genesis` file. A failed preference write is a separate warning after an otherwise successful world save. Autosave remains three atomic slots every crossed 5,000 ticks. Encoding/writing is synchronous on the simulation worker; UI remains responsive, but very large saves can briefly pause ticks. Asynchronous autosave redesign is deferred unless measurements show unacceptable delay.

Phase 2 uses simulation v5 with the existing save envelope v1 and RNG v1. The changed 22-locus genome and morphology/species state are not v4-compatible. Header version checks reject actual v4 saves with a clear instruction to use v0.1.0; no silent migration or compatibility emulation. See PHASE2_SAVE_MIGRATION.md.
