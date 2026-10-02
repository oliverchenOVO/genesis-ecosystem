# Milestone validation

## 1 — Authoritative headless foundation (simulation v1)

Implemented seeded integer simulation, separate genetic/phenotype models, spatial indexing, utility behavior, metabolism, two-parent inheritance/mutation, local ecological interactions, lineages/species persistence, extinction, history, real telemetry, canonical hash, versioned compressed atomic save/load, replay and benchmark/stress CLI.

Validation: 22 core unit tests; Rust release build; clippy with warnings denied; rustfmt. Golden v1 has six checkpoint hashes for seed 42 with two recorded interventions. Browser frontend is being developed separately.

Initial performance measurements (`benchmarks/small.json`, `benchmarks/medium.json`) uncovered excessive predation: seed 42 at 2000 founders becomes extinct, so the large ticks/sec figure mostly measures an empty world. These values are diagnostic, not a claim of sustained-population throughput. Next milestone must fix the ecological attack model, bump simulation version, preserve v1 golden history, add regression coverage, and rerun benchmarks. Phase 1 is not complete at this milestone.

Local toolchain issue: Windows MSVC installer stalled downloading one CRT archive. Rust 1.99.0 and verified official Microsoft MSVC/SDK package contents were used from ignored `.tools/` to allow independent validation. `scripts/local-toolchain.ps1` activates that local fallback only when present. Normal installed toolchains remain supported. Cargo HTTP multiplexing was disabled because the local network proxy stalled downloads; HTTPS/checksums remain enabled.

## 2 — Ecological attack correction (simulation v2)

The real v1 benchmark revealed prey collapse from cost-free, instant predation. V2 replaces that rule with hunger-gated hunting, aggression/body-size damage, an energy cost and cooldown, while preserving real mortality and inherited diet. Added healthy-prey survival/cooldown regression coverage; the lethal-predation test explicitly tests an already wounded prey. No simulation outcome is forced, and no failed assertion was removed. The old six-checkpoint golden fixture remains in the repository as an incompatible historical version; v2 establishes a new baseline because the documented biological algorithm changed.

Validation: release build, 23 unit + 2 golden/version tests, clippy (warnings denied), rustfmt pass. Small and medium runs still eventually collapse: predatory alleles spread through unrestricted inter-diet mating. V2 therefore fixes the attack semantics, but does not yet meet the intended sustained-ecosystem product experience. Its benchmark JSON is retained as `v2-*` diagnostic evidence. A further genetic-compatibility correction is required before final stress/scalability claims.

## 3 — Genetic compatibility and dietary tradeoff (simulation v3)

Mating now requires both mean genetic distance <=220 and every locus distance <=400. Plant conversion scales continuously with inherited carnivory. Added a generic per-locus compatibility regression; preserved v1/v2 fixtures and explicitly reject their versions. Release build, 24 unit + 3 golden tests, clippy and formatting pass. Both v3 seed42 benchmarks still become extinct; results are diagnostic and cannot establish sustained-population performance. Ecological balancing remains a Phase1 acceptance limitation, rather than forcing survival or weakening tests. Normal MSVC installation has now completed.


## 4 — Permanent species origin generation and honest workload accounting (simulation v4)

Species now permanently retain the minimum actual parental generation among their founding members. This schema/canonical-state change explicitly bumps simulation version; v1/v2/v3 fixtures remain incompatible historical records. Classifier fixture checks the stored cohort minimum and save roundtrip. Release build, 26 core unit tests, 1 worker integration test and 4 golden/version tests pass. Benchmark records exact occupied ticks and organism-ticks rather than implying every tick carried a large population. Seed42 small: occupied2515/10000; medium: occupied2264/50000. V4 biology is unchanged from v3. The first v4 timings were measured concurrently with the ongoing stress job; final isolated performance will be recorded separately.

