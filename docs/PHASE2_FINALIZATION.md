# Phase2 finalization — rules frozen, acceptance pending

The original a3d8cc0 evidence and private branch are preserved. The selected candidate D is authoritative simulation5/rules_revision2, RNG1/save envelope1, analysis4/diagnostic4. No merge, tag or release is permitted while Phase2 is INCOMPLETE.

## Root cause and bounded experiments

PHASE2_VIABILITY.md records actual capped energy income/cost, lifespan, offspring and mating-gate analysis before tuning. High-complexity organisms had higher costs, low actual offspring and a sharp structural single-locus mating cliff. The feeding quota25 was not binding. Extra storage also raised the old fractional-capacity readiness requirement, while costly construction was independently paid. These observations motivated separate paired changes; they are not proof by themselves.

All rows below use the same seeds0-7,512 world,50 founders,ceiling2000,mutation100,20C/regen4, except the explicitly labeled density probe. Every completed world passed invariants, full save/load/replay and1000-tick RNG continuation, with exact diagnostic energy/population closure and0 safety-cap ticks. No seed or outcome was hardcoded.

| Candidate | Changes | Ticks/world | Survive | Persistent multi-unit | High-C worlds | Innovation species counts [multi,armor,piercer,mass,sensory] | Decision |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- |
| A | Structural400 cliff removed; global220 and original14 barrier retained |50000|8/8|0/8|0/8|0,0,0,0,0|Insufficient alone|
| B | Bite-dependent Crusher hard-resource processing |50000|8/8|0/8|0/8|0,0,0,0,0|Insufficient alone|
| C | A+B |50000|8/8|0/8|0/8|0,0,0,0,0|Insufficient alone|
| D | C+readiness reserve decoupled from extra morphology capacity |50000|8/8|2/8|1/8|0,0,0,0,0|Selected for extension|
| D long | Frozen D |100000|8/8|2/8|1/8|0,1,0,0,0|Selected for formal100-seed run|
| E | D+removing dietary original-locus barrier |50000|3/8|1/8|2/8|1,0,0,0,0|Rejected:5 natural world extinctions;6 Collapse classifications|
| D density | D with200 founders |50000|7/8|1/8|0/8|0,0,2,0,0|Rejected as default:1 world extinction; no high-C improvement|

Persistent multicellular remains>=8 and>=25% of the population for1000 consecutive sampled ticks; high-C lineage remains>=8 members,mean C>=200 for1000 ticks. No thresholds are weakened. D long final populations948-1013,0 Collapse/Explosion. Seeds5/7 have longest multi-unit periods6300/33000 ticks; seed7 has5 persistent high-C lineages. These can later recede and do not imply sustained dominance to the end. High-C completed offspring average1.9481949 versus baseline0.291139; child high-C retention77.287%, multi-unit retention83.037%, same-lineage retention81.894%. This documents actual fitness/inheritance rather than snapshot mutation counts.

All eight D long worlds still had one persistent niche cluster and one instantaneous morphology cluster. No differentiated persistent trophic role is claimed from traits alone. The two Piercer innovations in the density probe require observed kills historically; they do not prove ongoing predator energy dependence. Removing the diet barrier caused collapse and is not adopted. Formal analysis4 measures actual100-tick energy roles and persistent morphology representatives to resolve these outstanding biological gates.

Raw JSON/JSONL and pooled diagnostic summaries: benchmarks/phase2-candidate-{a,b,c,d,d-long,e,d-density}*. Source patches/executable and source hashes: benchmarks/phase2-finalization-provenance. Old baseline diagnostic-v2 eight50k worlds retain exact hash parity with diagnostic-v1. The new golden was generated from revised execution and independently matches all six D-executable checkpoints. Expected prior-golden failure is retained. Old v5 saves/replays are explicitly rejected rather than reinterpreted; see PHASE2_SAVE_MIGRATION.md.

## Current acceptance checkpoint

Rules are frozen. The final100*100000 run is executing separately as phase2-final-calibration-v2, with the unchanged ecological default and full verification. The preserved revision1 phase2-final-calibration remains valid historical evidence for its own rules only. Final production packaging, updated showcase, isolated three-scale timings and native UAT have not yet been accepted for revision2. Native UAT follows successful biological acceptance, per the finalization instruction. Do not claim Phase2 COMPLETE from the bounded matrix.

Freeze validation:73 all-features workspace Rust tests,24 frontend tests,4 Node checks; fmt/clippy all-targets/all-features, frontend formatting/lint/typecheck/build, release benchmark build and real-worker snapshot benchmark all passed. Optional observer tests prove exact closure/hash parity, per-window draining and absence-of-income rejection; morphology persistence tests cover duration and species loss. The selected D long benchmark is causal ecology evidence; concurrent snapshot latency is not isolated simulation TPS. No authoritative rule edits after this freeze are planned during the100-seed run.
