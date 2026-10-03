# Phase 2 save and replay policy

The first authoritative morphology implementation advances simulation version from 4 to 5, once. RNG algorithm/version stays 1: extra genetic loci change stream consumption and world evolution, not the RNG algorithm. The GENESIS1 envelope remains save format 1; simulation version distinguishes the changed bincode state representation.

v4 saves/replays are intentionally unsupported in v5. Their version is checked before payload decoding and rejected with instructions to use the validated v0.1.0 application. There is no migration, no reinterpretation of old loci and no invented morphology/history. Keep v0.1.0 and historical v4 fixtures intact. Frontend file-error handling preserves the current world.

v5 stores complete regulatory alleles, derived phenotype, all four RNG streams, registries, commands, telemetry and history. Loading validates genome/phenotype equality and bounds. Replay starts from seed/config and consumes ordered commands under v5 rules. New golden fixtures must be generated from actual execution and independently verified, never substituted into v4 evidence.
