# Replay v1

Replay JSON contains simulation/RNG version, full initial configuration (including seed), ordered environmental commands, final tick and ordered checkpoint hashes. The core reconstructs the initial world, applies commands at their recorded tick in recorded same-tick order, then compares checkpoints before advancing the next tick.

The desktop can verify its current world by reconstructing from seed and command log. The headless validator verifies golden fixtures, multi-seed save continuation and replay equality. Replay is not a frame recording; no browser timing enters the result. External replay files are versioned and validated, with a 10-million-tick execution bound.

No changes to golden hashes are allowed without a simulation version change and explicit documented algorithm reason.

V4 changes permanent species origin-generation metadata and canonical serialization, retaining its own golden baseline. A final checkpoint at final_tick is mandatory, including tick zero. Earlier version fixtures are preserved and rejected. No fixture is refreshed to mask an assertion failure.

Phase 1.1 file UX and calibration do not change replay versions or golden fixtures. Showcase seed 11 uses the existing v4 configuration: size 512, founders 50, capacity 200, mutation multiplier 100, initial temperature 20°C and regeneration 12. Its world begins at tick zero and contains no injected species, mutations or events.

Calibration's temperature sweep uses `1800 + (seed % 5) * 100` at tick zero. The runner omits a redundant command at 20°C/regen 12; the original Phase 1 stress runner logged it. This changes command-log hashes for that subset when comparing different command sequences, but not ecology rules. Preserve original stress artifacts as the exact Phase 1 baseline; compare hashes only for identical command sequences.


Phase 2 v5 has a separate golden-v5 fixture. The original v4 fixture remains untouched and executes in CI by checking out protected v0.1.0; the current v5 decoder explicitly rejects it. Morphology/ecology rules and extra loci intentionally change v5 hashes. Analytical calibration reads never change world state or RNG.
