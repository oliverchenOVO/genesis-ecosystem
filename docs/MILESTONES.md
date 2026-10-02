# Milestone validation

## 1 — Authoritative headless foundation (simulation v1)

Implemented seeded integer simulation, separate genetic/phenotype models, spatial indexing, utility behavior, metabolism, two-parent inheritance/mutation, local ecological interactions, lineages/species persistence, extinction, history, real telemetry, canonical hash, versioned compressed atomic save/load, replay and benchmark/stress CLI.

Validation: 22 core unit tests; Rust release build; clippy with warnings denied; rustfmt. Golden v1 has six checkpoint hashes for seed 42 with two recorded interventions. Browser frontend is being developed separately.

Initial performance measurements (`benchmarks/small.json`, `benchmarks/medium.json`) uncovered excessive predation: seed 42 at 2000 founders becomes extinct, so the large ticks/sec figure mostly measures an empty world. These values are diagnostic, not a claim of sustained-population throughput. Next milestone must fix the ecological attack model, bump simulation version, preserve v1 golden history, add regression coverage, and rerun benchmarks. Phase 1 is not complete at this milestone.

Local toolchain issue: Windows MSVC installer stalled downloading one CRT archive. Rust 1.99.0 and verified official Microsoft MSVC/SDK package contents were used from ignored `.tools/` to allow independent validation. `scripts/local-toolchain.ps1` activates that local fallback only when present. Normal installed toolchains remain supported. Cargo HTTP multiplexing was disabled because the local network proxy stalled downloads; HTTPS/checksums remain enabled.
