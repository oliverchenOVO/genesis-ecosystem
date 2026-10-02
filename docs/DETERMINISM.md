# Determinism contract

Same simulation version, seed, configuration, and ordered commands produce identical checkpoint hashes. Authoritative values use integers: temperature in hundredths of Celsius, energy in integer units, bounded genes 0–1000, integer positions. Display interpolation never feeds back into the core.

RNG version 1 uses SplitMix64 with explicit wrapping arithmetic and rejection-sampled bounded draws. World, behavior, reproduction, and mutation have independent saved streams. No system time or UI randomness is allowed in simulation.

Entities are processed in monotonically increasing IDs. Intent evaluation reads an immutable tick snapshot, then resolution uses explicit stable tie breaks. Registries use ordered maps. Hashes include canonical serialized world state, versions, RNG streams, history, telemetry and commands, excluding timing/UI data.

Algorithm changes require a simulation version bump, a reviewed new golden fixture, and a report entry explaining the change. Never refresh a fixture simply to hide a failure.
