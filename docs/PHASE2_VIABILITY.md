# Phase 2 finalization: complexity viability diagnostics

The a3d8cc05af16e01ba38096db11bfac293e59df4f state is retained by local branch phase2/pre-finalization-a3d8cc0. Existing100×100000 results, showcase, v5 golden and protected v4 history remain unchanged. No merge/tag/release or Phase2.1 work.

The first milestone adds feature-gated deterministic viability accounting to the unchanged authoritative kernel. Enable with cargo build --release -p sim-benchmark --features viability; calibration adds phase2.viability. The collector is thread-local, reset per independent world, stopped before replay/continuation, and absent from State, RNG, save bytes and normal compact snapshots. No rule change has yet occurred.

Nine overlapping inherited cohorts: C<80,80≤C<140,140≤C<200,C≥200,single-unit,multi-unit,high-complexity,Piercer,predator-like. Counters record actual capped food/prey income, metabolic/attack/reproductive expenditure, offspring birth energy and energy removed at death. Per-cohort input minus output must close exactly, as must initial population+births−deaths−final population. Requested basal/morphology/movement/thermal costs are separate from actual clamped expenditure; do not pretend scarce energy pays every requested component in full. Completed lifespan/offspring means exclude living right-censored organisms, whose age/offspring sums are recorded separately.

Mating diagnosis records readiness, nearby unpaired individuals, nearby eligible mates, full-genome compatible eligible mates and mean-distance-compatible eligible mates. Successful matings/offspring are recorded per participating parent, not just the initiating parent. Nearby mean-distance counts are a diagnostic counterfactual, not permission to change compatibility. Predator measurements include prey visibility, hunting exposure, attempted attacks, actual kills, prey income, starvation and offspring.

Whole-run pooled phenotype cohorts are observational and can differ in other genes or habitats. They identify hypotheses, not controlled causal effects. Candidate rules require paired fixed-seed experiments; analytical persistence acceptance is unchanged (≥8 /25% /1000 ticks for multicellular,≥8 /mean C≥200 /1000 for complex lineage).

Initial8×50000 diagnostic-v1 evidence is preserved as phase2-prefinal-viability-v1.json/jsonl. It has energy/mating counters but predates detailed nearby-mate gate decomposition. Diagnostic-v2 repeats the same8 seeds/config with gate decomposition; original seed4 full50000 hash provides an independent baseline parity check. New100×100000 acceptance is required only after any authoritative candidate is selected and frozen.

## Measured pre-final economics (before any rule changes)

Baseline8 seeds×50000,50 founders/size512/cap2000/mutation100/20°C/regen4. All8 pass validation, full replay and1000-tick save/load continuation. Diagnostic-v1 and v2 authoritative hashes are identical for every seed; seed4 equals the independently verified original showcase hash8c04e07c25f14f9c01286f26e4d28f7a8e284ecb3e41f79999dcb5c6f4512b79. The original100×100000 dataset remains valid for this unchanged kernel. All nine overlapping cohort energy and population closures are zero.

| Cohort | Food/prey intake/tick | Total actual expenditure/tick | Requested morphology/tick | Requested movement/tick | Construction mean | Completed lifespan | Completed offspring | Starvation death fraction |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| C<80 | 5.0827053364 | 5.4341586379 | 0.9999971774 | 0.6650658153 | 1153.95010957 | 1135.02632754 | 2.0539123744 | 0.4645519440 |
| Multi-unit | 6.2808285699 | 6.8016882647 | 2.0381036379 | 1.1300999130 | 1462.28455515 | 1079.28658537 | 1.3889735772 | 0.4979674797 |
| C≥200 | 6.9770547850 | 7.1900797732 | 3.3134085666 | 1.3077660728 | 1836.98986853 | 895.25316456 | 0.2911392405 | 0.6329113924 |
| Piercer | 2.6321629323 | 4.7870589039 | 1.0227476774 | 0.6400821688 | 1195.88723749 | 654.75080906 | 0.0388349515 | 0.8576051780 |
| Predator-like | 5.1449717139 | 5.8241153319 | 1.0287791714 | 0.7331068057 | 1245.10320157 | 1379.37974684 | 0.1518987342 | 0.2531645570 |

Food acquisition is not simply a25-unit throughput ceiling: average resource units consumed per attempt4.19(simple),5.44(multi-unit),5.56(C≥200); available resource8.43/9.65/9.38 respectively. Increasing a global quota alone is unsupported. Multicellular forms pay roughly double morphology maintenance without doubled realized income. More capacity also raises the capacity-relative reproduction threshold; measured average capacity4709.80(multi-unit)/5245.40(high C), stored energy1733.26/1614.11.

High-C reproduction-ready exposure8.3929% exceeds simple4.3894%, yet mating success per participating-parent attempt0.21498% versus2.47024%. Of nearby eligible mates, high-C full compatibility0.87642%, simple18.5815%; mean genome distance alone accepts100% in both. This supports a sharp single-locus incompatibility bottleneck, separately from energy economics.

A targeted two-seed×50000 diagnostic-v3 decomposes that barrier without changing rules. For high-C: original14-locus condition83.7524% pass; structural8-locus condition2.90135%; segmentation alone13.5397%; full1.35397%. For predator-like: original condition2.47934%, structural51.2397%, carnivory alone2.47934%. Founder dietary alleles800/200 already span600 against the400 single-locus limit; structural mouth variants add a second barrier. Predicate order/RNG/tick authority stay unchanged. These percentages are conditional on actual nearby eligible mates, not all possible population pairs.

Candidate hypotheses: first soften structural per-locus cliffs while retaining overall genome divergence; separately test a bite-dependent hard-resource processing payoff for Crusher mouths. If predator barriers remain dominant, test continuous dietary compatibility separately. No global energy abundance increase or generic complexity buff. Preserve25%/1000-tick persistence definitions. Small paired-seed matrix precedes any golden replacement or formal100-seed rerun.

Validation at this diagnostic milestone:65 Rust tests with all features (63 normal+2 optional observer),24 frontend,3 Node evidence checks,fmt/clippy(all features)/frontend lint/typecheck/build passed. A separate non-observer Tauri EXE and NSIS build passed under src-tauri/target/phase2-review; no native UAT claimed. Original target/release v0.1.0 hashes remain unchanged. No authoritative changes or golden replacement in this milestone.

## Bounded candidate decision

Measured tuning is now complete for the initial bounded matrix. The detailed raw runs, source patches, source/executable SHA256 provenance and failures are preserved under benchmarks/phase2-candidate-* and benchmarks/phase2-finalization-provenance. PHASE2_FINALIZATION.md lists all paired results, rejected collapse-prone dietary/density changes and the selected D readiness/feeding/compatibility combination. Diagnostic4 additionally measures actual child retention; high-complexity offspring retention77.287% and multi-unit offspring retention83.037% in D's eight100000-tick worlds support genuinely inherited cohorts, rather than one-off mutants. These are whole-run parent-cohort associations, not global population fractions or a substitute for100-seed acceptance.


## Final100-world revision2 cohort evidence

Formal100*100000 completed with100 full replay/save/load/1000-tick continuation passes and900 zero energy/population closure rows.

| C bucket | Intake/tick | Actual expenditure/tick | Completed lifespan | Completed offspring/parent | Mating success/attempt |
| --- | ---: | ---: | ---: | ---: | ---: |
| C<80 | 4.442835 | 5.097856 | 1113.914410 | 2.032573 | 0.027165 |
| 80<=C<140 | 5.375965 | 6.091805 | 1049.576505 | 1.860962 | 0.030581 |
| 140<=C<200 | 6.443814 | 7.303701 | 982.656578 | 1.828125 | 0.029022 |
| C>=200 | 8.279355 | 9.376892 | 867.407478 | 1.879436 | 0.026775 |

High-C retention70.149293%,same-lineage77.829523%;75154 actual parent offspring. Whole-run pooled completed lifespan/offspring exclude censored survivors; deficits do not include credited birth energy and are not closure errors. All exact balances/censored totals remain in phase2-final-viability-summary-v2.json.

Persistent multi-unit17/100; high-C13/100 (38 lineages,max12/world); one armor innovation. Multiple persistent morphology/niche clusters and differentiated trophic roles0/100. Speciation20/18 worlds versus R1's34/31 is a measured tradeoff, not a solved diversity gate. See PHASE2_REPORT.md; Phase2 remains INCOMPLETE.
