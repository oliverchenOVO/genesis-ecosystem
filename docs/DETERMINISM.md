# Determinism contract

Same simulation version, seed, configuration, and ordered commands produce identical checkpoint hashes. Authoritative values use integers: temperature in hundredths of Celsius, energy in integer units, bounded genes 0–1000, integer positions. Display interpolation never feeds back into the core.

RNG version 1 uses SplitMix64 with explicit wrapping arithmetic and rejection-sampled bounded draws. World, behavior, reproduction, and mutation have independent saved streams. No system time or UI randomness is allowed in simulation.

Entities are processed in monotonically increasing IDs. Intent evaluation reads an immutable tick snapshot, then resolution uses explicit stable tie breaks. Registries use ordered maps. Hashes include canonical serialized world state, versions, RNG streams, history, telemetry and commands, excluding timing/UI data.

Algorithm changes require a simulation version bump, a reviewed new golden fixture, and a report entry explaining the change. Never refresh a fixture simply to hide a failure.

Phase 1.1 keeps simulation v4, RNG v1 and every v4 golden checkpoint unchanged. Calibration observes worlds without consuming RNG. Worker concurrency distributes whole independent worlds, never steps within a world. Floating-point summary statistics and elapsed times are analytical output only; they do not decide survival, inheritance or speciation.

Dirty markers, native dialog state, file paths and recent-open wall-clock timestamps are application preferences. They are not serialized into State or included in hashes. Pausing for an unsaved decision changes wall-clock pacing only. Autosave uses the same read-only encoder and cannot change the continuation hash.
