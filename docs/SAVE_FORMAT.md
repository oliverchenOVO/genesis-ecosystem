# GENESIS binary save v1

Single `.genesis` file: 8-byte magic `GENESIS1`, three little-endian u32 versions (save/simulation/RNG), u64 uncompressed length, BLAKE3 checksum (32 bytes), then zlib-compressed fixed-width bincode state. Header is 60 bytes. Full world, ancestry, registries, RNG streams, configuration, command history, telemetry and event history are retained.

Decode rejects incompatible versions, truncated or corrupt data, checksum mismatch, excessive compressed/uncompressed size (>128 MiB), malformed bincode, invalid configuration and world invariants. A failed load does not replace the running world. No implicit migration is attempted in Phase 1.

Atomic save writes to a unique temporary file in the destination directory, synchronizes it, then uses tempfile's atomic persistence/replacement. Failed writes preserve the previous destination. Manual save/load are mandatory; rotating autosave is implemented in the desktop application layer.
