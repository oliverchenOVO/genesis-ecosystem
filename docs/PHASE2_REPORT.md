# R4 stabilization — diagnostic stage A in progress

**PHASE 2 INCOMPLETE.** R3 authority remains simulation5 / rules3 / RNG1 / envelope1 / analysis5. No authoritative ecology change, rules4 selection, new biological pass, native UAT, merge, tag or release has occurred.

R3 cfef69e has been preserved as `phase2/r3-frozen-cfef69e`; `PHASE2_REPORT_R3_ARCHIVE.md` and `golden-v5-r3-archive.json` exactly match its Git blobs. The protection manifest is `benchmarks/phase2-r4-r3-preservation.json`.

The optional headless observer now records100-tick per-lineage credited diets and resident traits,1000-tick lifecycle/energy budgets, same-world/same-lineage individual diet/fitness groups, density-controlled actual mating searches and successes, predator budgets and spatial opportunities, plus1000-tick stock/productivity snapshots for deterministic coherence/persistence analysis. Definitions and limits are in `PHASE2_R4_DIAGNOSTICS.md`. The additive nested diagnostic schema is `ecology.r4_temporal.version=1`; the original ecological acceptance is unchanged.

Local validation: normal Rust75, all-features83, frontend24, Node23 PASS; fmt/clippy/frontend lint/typecheck/build PASS. A fixed1000-tick seed0 smoke has identical R3-frozen/current-normal/observed hash `b21177ecbc60aacfb3425b32ec123e478ebd5233d09cd19b0c593ff13e9c5aef`, with save/load/full replay and1000-tick continuation verified. A lossless measured fixture supports independent corruption rejection tests. Code-validation provenance: `benchmarks/phase2-r4-diagnostics-code-validation.json`.

The prescribed unchanged-R3 seeds0..7 ×50000 diagnostic run is in progress. No candidate changes are justified by incomplete measurements; reference-200 candidate performance and frozen R4 100×100k are not reached. Native UAT remains biologically gated. Private branch CI for the new diagnostic code will be recorded after the commit. The complete sealed R3 formal findings follow; they are not R4 results.

---

# Phase 2 ecological differentiation finalization — PHASE 2 INCOMPLETE

Revision3 passed deterministic stability and retained viable complex forms, but **all four ecological acceptance gates remain zero in the formal100-seed study**. Biological acceptance FAIL. Native UAT remains gated. No merge to main, v0.2.0 tag, release, Phase2.1 or Phase3.

## Frozen authority and provenance

- Branch: `phase2/multicellular-ecology`.
- Frozen simulation source: `831f8617d19714d856d653278e244e1f9becaa28`; evidence commit: `aa0dce26b7de8bc9a1373e7c65912dd08774dc5b`. Subsequent report changes do not alter simulation rules.
- Simulation5 / rules3 / RNG1 / save envelope1 / analysis5. Optional viability diagnostic4 and nested ecology4 remain outside State, RNG, save payloads and frontend frame telemetry.
- Preserved R2 baseline: `b193c9b13ea000284bcc0daffb8120e5d07a9655`, including original100×100k evidence, showcase, benchmarks and golden. The exact original report is [PHASE2_REPORT_R2_ARCHIVE.md](PHASE2_REPORT_R2_ARCHIVE.md).
- Protected main: `216a92ffca8e66e31789fc0a0e0eaf5085f13d76`; v0.1.0 target: `ed6c77d4b876b859ed7a4b1d665f7bf9054b9096`. No LLM determines world state.

Diagnostics preceded physical changes. Bounded paired seeds0–7 ×50k evaluated unchanged R2, resource-only B/B2, local-mating C and combined D; B2 was one bounded correction to the newly introduced hard-processing range. Unchanged A and selected D then completed paired8×100k. D retained3 persistent multicellular worlds and2 high-C worlds versus A2/1, with zero completed-world technical/replay/cap failures. Every A long-run hash matches sealed R2; all8 final R3 hashes match frozen D. Rejected candidates and compiled-source patches remain preserved. Details: [diagnostics](PHASE2_ECOLOGICAL_DIAGNOSTICS.md), [candidate matrix](PHASE2_ECOLOGY_CANDIDATES.md).

Selected D adds renewable soft/hard fields, existing morphology-dependent processing and local mating radius16. Costs, readiness, compatibility, dispersal and cluster150/persistence1000 thresholds remain unchanged. No fertility ban or region wall was introduced. The selected-channel efficiency is reused without changing integer operation order or world hashes; completed calibration ledgers are moved rather than cloned into the final report.

## Formal100×100000 evidence

Seeds0–99,512 world,50 founders,limit2000,mutation100,temperature2000,regeneration4,8 workers. Reported wall time10891.9436458 seconds (181.53239409666666 minutes) includes diagnostics, full replay, save/load and independent1000-tick continuation; this is not kernel benchmark TPS.

| Measurement | Actual R3 result |
| --- | --- |
| Completed / save-load-full-replay-continuation verified |100/100 /100/100 |
| Technical failures / replay mismatches |0 /0 |
| Survival / natural world extinction |99/100 /1 (seed43) |
| Collapse / Explosion / Sterile classifications |1 /0 /0 |
| Safety-ceiling worlds / maximum fraction |0/100 /0 |
| Speciation |7/100 worlds;9 new species |
| Species extinction |5/100 worlds;6 extinct species |
| Persistent multicellular |12/100; longest11700 sampled ticks |
| Persistent high-C lineage |21/100;35 world–lineage pairs; maximum4 per world |
| >1 simultaneous persistent morphology cluster |0/100 |
| >1 simultaneous persistent niche cluster |0/100 |
| >1 actual simultaneous persistent feeding role |0/100 |
| Persistent actual prey-income cohort |0/100; longest qualifying streak0 |
| Innovation species/world counts [multi-unit,armor,Piercer,mass,sensory] |[0,0,1,0,0] /[0,0,1,0,0] |
| Final population [min,median,max,mean] |[0,1234,1336,1220.97] |
| Population CV [median,mean] |[0.21464853972476822,0.2170815635491241] |
| Resource CV [median,mean] |[0.8875655825903283,0.9258168471726713] |

Compared with sealed R2, multicellular incidence changes17→12, high-C worlds13→21, and high-C world–lineage pairs38→35. These are mixed viability tradeoffs, not universal improvement. Both nonzero complexity gates pass; ecological gates do not. Natural extinction is preserved as an ecological outcome rather than omitted or counted as a technical failure.

Acceptance uses unchanged thresholds: multicellular >=8 and >=25% living multi-unit population for1000 sampled ticks; high-C lineage >=8 members and mean C>=200 for1000 ticks; cluster distance150 with1000-tick persistence. Realized resource consumers require >=8 and >=90% actual resource income per sampled window; prey consumers require >=8 and >=50% actual prey income, with1000 consecutive sampled ticks after warmup. Mouth names do not establish feeding-role persistence.

All100 complete JSONL rows equal their final JSON rows. All900 overlapping cohort energy/population closures are independently recomputed, including world-counter closure; temporal mating, symmetric gene flow and credited-energy ledgers close. The two full raw outputs are retained locally and losslessly compressed into Git, with decompressed SHA256 round-trip verification. No ledgers were cropped.

- [Formal summary](../benchmarks/phase2-r3-final-calibration-summary.json)
- [Full ecological summary and per-world gene-flow matrices](../benchmarks/phase2-r3-final-calibration-ecology-summary.json)
- [Viability budgets](../benchmarks/phase2-r3-final-calibration-viability-summary.json)
- [Biological gate and ecological review](../benchmarks/phase2-r3-final-ecological-review.json)
- [Provenance, raw/compressed sizes and hashes](../benchmarks/phase2-r3-final-calibration-provenance.json)
- Full lossless evidence: [JSON.gz](../benchmarks/phase2-r3-final-calibration.json.gz), [JSONL.gz](../benchmarks/phase2-r3-final-calibration.jsonl.gz).

## Realized ecology and gene flow

There are4413350 successful mating pairs:2680774 cross-lineage and2190979 cross-habitat. Pair-weighted fractions are0.6074238390338405 /0.49644351796254543. Unweighted per-world means are0.6017934660410414 /0.4947262877516618. Pooled integer-floor mating distance mean12.908427838263451, median14,p90=15. The original paired8-world A→D cross-lineage means0.48577273560644685→0.5841650703643353 do **not** support reduced cross-lineage mixing; cross-habitat means decrease0.5880522490506069→0.49738592855325925. This paired comparison is separate from the100-world R3 statistic.

Among99 worlds with eligible persistent-lineage overlap samples, unweighted means of each world's top16 lineage-pair histogram intersections are spatial0.3720213039967763, habitat0.794625946593701, feeding hardness0.8912081221982705, productivity0.8919233504301333 and realized energy sources0.9047529960701577. Source-overlap world means range0.822494532865523–0.9636227649364436. Empty overlap data for the extinct world are excluded, not replaced with zero. Lower spatial overlap can coexist with substantial habitat and dietary overlap.

Actual credited whole-run energy, pooled by inherited mouth morphology:

| Mouth cohort | Soft | Hard | Prey | Hard / total | Prey / total |
| --- | ---: | ---: | ---: | ---: | ---: |
| Grazer |12528576670 |1628599116 |0 |0.11503700601150393 |0 |
| Crusher |12532998244 |19174166255 |5065752 |0.6046300150837033 |0.0001597412720551327 |
| Piercer |2817149 |2884667 |6129799 |0.2438100800271138 |0.5180864150836552 |

All1881 active post-warmup5000-tick windows in99 worlds consume both environmental channels. No active window or surviving world permanently leaves either channel unused. These totals demonstrate functional resource processing differences but do not establish continuous lineage-specialist persistence.

The separate predator-like cohort (carnivory>650 and non-Grazer) realizes11195551 prey energy versus1300474 resource energy, prey fraction0.8959289854173628. It perceives prey for2215913/2429770 organism-ticks, hunts1312691 ticks, attempts66186 attacks and kills15855;836 of1889 deaths are starvation. Actual intake per tick5.142883894360372 is below metabolic+attack+reproductive expenditure5.831810418270042; initial/offspring energy is included separately in the closed ledger. It initiates352628 mate searches; candidate-observation counts are1460 nearby eligible and330 compatible (not distinct individuals). It records657 successful matings and1118 offspring. These pooled associations support investigating energetic and reproductive limitations, not imposing absolute carnivory. Requested hunting movement by mouth [Grazer,Crusher,Piercer] is[0,51202,336984]; requested costs are distinct from clamped actual metabolic expenditure.

### Analytical limitation

Legacy species niche vectors combine resource units/prey mass, prey size, received attacks, occupied temperature, elevation and speed; they have no direct soft/hard energy axis. The unchanged realized trophic gate distinguishes resource consumers from prey-income consumers rather than separate soft and hard specialists. Ecology4 stores temporal mouth-channel income and lineage occupancy, but lineage feeding-channel proportions are lifetime aggregates. Therefore zero niche-cluster incidence cannot alone prove complete absence of soft/hard specialization. Nevertheless the sustained actual prey-income gate independently fails. No threshold, persistence interval or acceptance definition was relaxed.

Independent experimental D seed1×100k has15 living lineages>=8, maximum normalized nine-trait distance42 and zero pairs>=150. Its hash matches the final formal seed1. This is one snapshot, not a100-world persistence claim or a qualified ecological showcase. No scripted R3 showcase or validated release asset was created.

## Final performance

Sequential same-host R2 then R3 scenarios after calibration/audit/compiler/native-app task loads stopped. External host load is uncontrolled; small report reads occurred during the sequence. Original R2 benchmark files remain unchanged. Different rules produce different trajectories, so this comparison does not isolate resource mechanics from evolved phenotype distribution.

| Scale | Preserved original R2 TPS | Fresh paired R2 TPS | R3 TPS | Paired slowdown | R3 peak working set bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| reference-200 |8246.500568620953 |5894.054859009905 |3567.448191109341 |39.47378712201861% |9342976 |
| medium-1000 |1405.6247649268284 |832.3077013606392 |586.5427319079399 |29.52813833765179% |11472896 |
| density-5000 |153.6327225408394 |89.3660164337223 |82.65501872305617 |7.509563454295054% |19308544 |

Reference exceeds30% and was profiled on the frozen optimized source: perception11.6716762 of13.1044635 marked seconds (89.0664177133234%). The endpoint hash matches the normal benchmark. Two-channel opportunity scoring adds processing work within perception; the earlier redundant third efficiency calculation has been removed with identical golden and benchmark hashes. There is residual reference slowdown, not a claim of performance parity. New mating queries reuse the spatial index; resource opportunities traverse bounded nearby cells rather than scanning the entire population. Spatial/full-scan oracle tests cover query boundaries. Profile phases locate the bottleneck but do not separately attribute every cost to channel arithmetic versus changed trajectories.

[Performance comparison](../benchmarks/phase2-r3-final-performance-comparison.json) includes exact normal TPS and profile provenance; individual benchmark/memory JSONs preserve hashes and updates/second. Profile: [result](../benchmarks/phase2-r3-final-profile-reference-200.json), [phases](../benchmarks/phase2-r3-final-profile-reference-200-phases.json).

## Regression, builds and compatibility

Rust75 normal /81 all-features; frontend24; Node13: all PASS. The targeted debug autosave recheck also passes (188.44s total including3 replay verifications) with its120-second advance deadline unchanged; it is not counted twice. Rust fmt, workspace clippy all targets/features, frontend format/lint/typecheck and production frontend build PASS. Current golden suite7 PASS, all6 R3 checkpoints match frozen D; protected v4 CI golden4 PASS. R2 golden is archived exactly at blob `da2e3b0427b8efa077bb2a7674a9ce567cac91f4`. Details: [code validation receipt](../benchmarks/phase2-r3-final-code-validation.json).

Frozen code-head private CI [37157748106](https://github.com/oliverchenOVO/genesis-ecosystem/actions/runs/37157748106) is GREEN for core/windows/legacy-v4. Windows CI builds the production frontend, Tauri EXE and NSIS installer. Final evidence/report commits are pushed to the same private branch and their actual CI status is verified at delivery; the code-head receipt refers specifically to831f861.

Rules3 payload tag is `GENESIS5RULES003`. Real R2, R1, v4 and experimental probe saves reject explicitly before incompatible State decoding; no silent migration. See [save/replay policy](PHASE2_SAVE_MIGRATION.md). Backend/golden rejection and corruption tests pass; native rejection UX remains untested.

| Artifact | SHA256 / status |
| --- | --- |
| Frozen headless formal calibration EXE |`3B01B8B2BA0478D1F7CBCBA838E2E05C5F27EB238864573F725422E6649A53BF` |
| Frozen normal R3 benchmark EXE |`C245D1C559915499317BABC7D4A5F68BE12925B2BE4305271B124D316880DF64` |
| Frozen optimized R3 profile EXE |`83BC8B93F3B241DA2D5198BB7766CC0BAFDD1B110FD382D39A69C9235A1AAE01` |
| Downloaded R3 code-head CI NSIS |`883875DD6FA40A6E2FA346FE1D18309EE50CBF21114174E9092D26382668A1FA`; BUILD ONLY |
| R3 raw desktop EXE |Compiled by CI; raw EXE is not in the uploaded artifact. No raw desktop EXE SHA or validated release EXE is claimed. |
| Protected Phase1 desktop EXE |`16FB27A061D6F817BDC68FE26897A6C69E1814FDB2110AD23C0A623ABE3C56C9` |
| Protected Phase1 NSIS |`EF09F5B0CEDACD1C5B62DF12776A4FF747E398A574E808054F51765821C115F4` |
| Preserved R2 showcase |`F5C5A91AF0B3735F94A6E45C569BD963E20D11FB7F517FAE70F0E9DB94B2713C` |

Native5-cycle restart, clean/Save/Discard close, New World, speed controls/MAX return, save/load/continue/replay, dirty dialogs, legacy/invalid/checksum UX, resize and ecological inspection: **NOT PERFORMED — biological gate FAIL**. Installer install/launch/restart/uninstall: **NOT PERFORMED; BUILD ONLY**. CI build success does not establish native or installation PASS. Main CI after merge, v0.2.0 tag/release and release hashes: NOT APPLICABLE; no merge/release occurred.

## Recovered technical issues and next hypothesis

Git milestones: `7703ef1` deterministic ecological diagnostics; `cc639813` paired R2 measurements; `89d622cc` bounded candidates; `00b1786e` selected rules3/resources/local mating and compatibility; `831f8617` bit-identical efficiency/report-memory improvement; `aa0dce26` complete formal ecology and paired performance evidence. Final report and validation receipt updates follow on the same private branch.

The earlier unchanged-R2 A long-run four-worker invocation aborted with allocation failure /exit0xC0000409 before any completed seed. Raw failure evidence is retained; the exact unchanged one-worker recovery completes all8 and matches sealed R2 hashes. Formal R3 runs once, completes all100 and exits0. Its post-processing liveness watcher raced the parent-shell exit-receipt write; only post-processing was restarted after adding a bounded30-second receipt grace period. No world data, tests, standards or authoritative rules were altered to recover. The earlier debug autosave timeout under concurrent calibration/build load is resolved by the unchanged targeted recheck after task load subsides; the prior overlapping executable-link lock was resolved by waiting for the owning build to finish.

Next Phase2 hypothesis: measure per-lineage temporal soft/hard realized diets and reproduction alongside spatial resource correlation/coherence before further physical tuning. Fine independent cells may not provide durable contiguous resource patches; this is an inference requiring measurement. Use existing actual hunting/mating budgets to distinguish energetic insufficiency from scarcity of compatible predator mates. Descendant-localization evidence does not justify global dispersal penalties. Do not shorten persistence, relax cluster thresholds, introduce reproductive walls or begin a later Phase.

**PHASE 2 INCOMPLETE.** Deterministic finalization and evidence are complete; stable ecological differentiation and the gated native/release acceptance remain unfinished.
