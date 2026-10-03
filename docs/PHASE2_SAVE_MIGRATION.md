# Phase 2 save and replay policy

The first authoritative morphology implementation advances simulation version from 4 to 5, once. RNG algorithm/version stays 1: extra genetic loci change stream consumption and world evolution, not the RNG algorithm. The GENESIS1 envelope remains save format 1; simulation version distinguishes the changed bincode state representation.

v4 saves/replays are intentionally unsupported in v5. Their version is checked before payload decoding and rejected with instructions to use the validated v0.1.0 application. There is no migration, no reinterpretation of old loci and no invented morphology/history. Keep v0.1.0 and historical v4 fixtures intact. Frontend file-error handling preserves the current world.

v5 stores complete regulatory alleles, derived phenotype, all four RNG streams, registries, commands, telemetry and history. Loading validates genome/phenotype equality and bounds. Replay starts from seed/config and consumes ordered commands under v5 rules. New golden fixtures must be generated from actual execution and independently verified, never substituted into v4 evidence.

## Unreleased rules revision2 (selected candidate D)

Simulation version5, RNG1 and the GENESIS1 save envelope1 remain unchanged because Phase2 has never been released. Authoritative rules revision2 explicitly distinguishes the revised physiology/mating rules. New saves prepend the16-byte `GENESIS5RULES002` marker inside the checksummed, compressed payload; the60-byte envelope remains unchanged. A valid old v5 payload cannot alias this marker: its encoded world-size bytes would be invalid. The original revision1 seed4 save remains intact as rejection evidence.

Old prerelease v5 saves are rejected before State decoding, with instructions to use the preserved pre-final build/branch phase2/pre-finalization-a3d8cc0. No migration, invented traits or silent continuation occurs. Replays now carry rules_revision2; a missing revision defaults to1 and is explicitly rejected. Legacy v4 still receives the v0.1.0 message before payload decoding.

The former v5 golden is retained byte-for-byte at fixtures/golden-v5-prefinal-r1.json. Its expected failure under selected D is retained in benchmarks/phase2-finalization-provenance/phase2-selected-d-old-golden.txt. The new v5 fixture was generated from real revised execution; all six checkpoint hashes independently match the frozen D executable. Protected v4 fixtures and v0.1.0 binaries are unchanged.
