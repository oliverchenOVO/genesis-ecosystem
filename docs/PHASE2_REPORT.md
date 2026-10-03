# Phase 2 validation report — INCOMPLETE

Recorded 2026-10-03. Implementation, local regression, final long-run calibration and review packaging are available. **Native production UAT was interrupted by the user's physical Escape key. Sustained multicellular/high-complexity and distinct persistent niche cohorts remain undemonstrated.** Do not merge, tag v0.2.0 or describe this branch as accepted.

## Implemented scope and version policy

Simulation version **5**, RNG version **1**, save envelope version **1**, analytical measurement version **3**. v4 saves are explicitly rejected before state deserialization and must be opened with v0.1.0. No silent reinterpretation or migration. Original v4 fixtures, history, main, v0.1.0 tag and released artifacts are preserved; see PHASE2_BASELINE.md and PHASE2_SAVE_MIGRATION.md.

Authoritative Rust systems: 22 inherited loci including eight structured morphology loci; segments, aspect, appendages, mass, armor, sensory/locomotion/storage investment; integer complexity, maintenance, movement and reproduction costs; Grazer/Crusher/Piercer feeding; prey-size, armor and speed interactions; heterogeneous temperature, productivity, moisture and terrain; resource-limited populations and technical-ceiling diagnostics. Species retain founder morphology, current/last meaningful morphology, feeding observations, niches and persistent innovation thresholds. No LLM, scripted biological transitions or later-phase systems.

Desktop: real compact morphology and habitat layers, phenotype/cost inspector, on-demand genomes, species/niche summaries, founder comparisons and history. Worker batches are bounded by64 ticks and12ms; pacing stays outside simulation truth. Initial habitat is recorded atomically as a tick-zero command with replay. Starter: seed42, size512,50 founders, safety ceiling2000, mutation100,20°C, regeneration4. Showcase: measured seed4 with the same config.

## Tests and private CI

Final local checks passed:

- **63 Rust tests**:41 core,5 golden/version compatibility,10 application worker/file-session,5 calibration,2 desktop. v5 golden passes all six checkpoints. Actual legacy save rejection and atomic new-world failure preservation are tested.
- **24 frontend tests**, **2 Node summary checks**. The summary rejects incomplete/duplicate/non-finite evidence and preserves zero incidence.
- Rust fmt and workspace all-target/all-feature clippy with warnings denied; frontend formatting, lint, typecheck, production build; Windows Tauri EXE/NSIS packaging.
- Observation/snapshot reads preserve hashes. Save/load, full replay and RNG continuation passed for all100 final worlds and all three benchmark scales.

The repository remains private: oliverchenOVO/genesis-ecosystem. Earlier code head11b1908: all three jobs green in [run37109545702](https://github.com/oliverchenOVO/genesis-ecosystem/actions/runs/37109545702). Latest production code head52ec2b2604203adaa282a69727e1ea94f658a14f: [run37110938683](https://github.com/oliverchenOVO/genesis-ecosystem/actions/runs/37110938683). Final result: **SUCCESS in all three jobs**, completed2026-10-03T09:09:48Z. Windows verifies63 Rust tests and production EXE/NSIS packaging; the private installer artifact was uploaded. Final documentation/evidence commit changes no production source; its separate CI run may follow this validated code-head run. The legacy job runs original golden tests under protected v0.1.0 source; v5 does not claim to reproduce v4 hashes.

## Final long-run calibration

Raw evidence: benchmarks/phase2-final-calibration.json, phase2-final-calibration.jsonl, phase2-final-summary.json. Seeds0–99, **100 × 100,000 ticks**, eight independent workers, final v5/analysis3, size512/50 founders/ceiling2000/mutation100/20°C/regeneration4. Validate every1000 ticks; final save/load, full100000-tick replay and1000-tick original/restored continuation. Total2974.0981963s includes verification and some concurrent development workloads; not an isolated TPS timing.

| Measure | Result |
| --- | --- |
| Survival | 100/100 |
| Natural world extinction / Collapse / Explosion | 0 / 0 / 0 |
| Technical failure / invalid state / replay mismatch | 0 / 0 / 0 |
| Save/load/full replay/RNG continuation | 100/100 verified |
| Speciation incidence / new species | 31/100 (31%) / 34 |
| Species-extinction incidence / extinct species | 17/100 (17%) / 19 |
| Safety-ceiling worlds / ticks | 0 / 0 |
| Final population min / median / max / mean | 727 / 849 / 921 / 841.76 |
| Population CV min / median / max / mean | 0.1962987571 / 0.2832553518 / 0.3534112535 / 0.2833988962 |
| Resource CV min / median / max / mean | 0.6637340807 / 1.4249848947 / 2.6876992448 / 1.4415738135 |
| Morphology sampled pair-distance world mean / median / max | 1.5411987382 / 0 / 14.9137749737 |
| Niche sampled pair-distance world mean / median / max | 3.5311356467 / 0 / 29.9442691903 |
| Final maximum morphology distance mean / max | 1.96 / 36 |
| Final maximum niche distance mean / max | 3.76 / 75 |
| Maximum sampled complexity min / median / max | 270 / 316 / 349 |
| Final median species complexity min / median / max | 17 / 48 / 72 |
| Predator/consumer coexistence ticks min / median / max / mean | 3745 / 14586 / 27177 / 14993.68 |
| Post-warmup sampled predator/prey ratio mean | 0.0002333501555 |
| Predator starvation deaths / successful predations | 486 / 12028 |
| Mean birth+death turnover per organism per tick | 0.00175328236575 |
| Mean species formation+extinction per tick | 0.0000053 |

**Persistent multicellular incidence0/100; persistent high-complexity lineages0; all five innovation counts0. Every world has only one persistent niche cluster and one morphological cluster under unchanged thresholds.** Individual multi-unit mutants occur but do not establish a sustained cohort. The legacy classifier labels100 worlds Rich due to lineage persistence; this is not evidence for niche-rich or sustained complex evolution. Predator roles generally fade after early coexistence. No thresholds were reduced to hide these limitations.

Final mean within-organism variances: [11062.27,7116.22,536.88,10.19,6222.28,240.83] for segments×1000,mass,armor,bite,sensory,complexity; full quantiles in summary JSON. Samples100 ticks, warmup5000. Persistent multicellularity requires≥8 multi-unit organisms and≥25% of living population for1000 consecutive sampled ticks. Persistent complexity requires≥8 members of a lineage with mean C≥200 for1000 ticks. Niche persistence tracks representative species and bounded drift; analytical clusters are not reproductive barriers. Definitions in PHASE2_SIMULATION.md.

Rejected/limited candidates remain separate: regeneration12 eight verified100000-tick worlds saturated the ceiling57–84% of duration; remaining scheduled seeds cancelled, not a completed100-seed run. Mutation200/26°C/regen6 eight20000-tick worlds survive with no cap but no sustained complexity. Founders200/ceiling5000/mutation500/26°C/regen6 four50000-tick worlds:4 survive,4 full replay/continuation passes,0 cap ticks,final1320–1387,two new species,0 persistent multicellular/high-complexity/innovation cohorts,max persistent niches1. Higher mutation/density did not justify replacing the stable default. All raw evidence retained.

## Genuine showcase

examples/phase2-seed4-tick50000.genesis: actual seed4, first natural speciation35000, tick50000,population727,two species294/433,31 lineages,11 multi-unit organisms,39 predations. Morphology distance34,niche distance37,max living C231. Actual organism23620: generation105,born48273,parents22862/23005,two segments,Crusher,mass550,C120,maintenance2,movement coefficient155,construction1190; no offspring yet. This proves actual inherited multi-unit morphology, not a persistent successful complex lineage.

Save SHA256 D111C7C70AD308D3D983B304C53E560B0DCB54C607179AA22B5472378D08F7A6. Replay hash8c04e07c25f14f9c01286f26e4d28f7a8e284ecb3e41f79999dcb5c6f4512b79;1000-tick continuation0974b24968c9ac5b5b953f90203a3bdfb69f6c9b0b107f8c02a48455fdd33e46. Actual records in benchmarks/phase2-showcase-seed4-50000-verification.json. Native load/two-species inspection remain pending.

## Performance

Sequential final headless scales: no competing compiler/calibration; desktop paused. Memory is sampled process working set including verification, not WebView memory. Each passed full replay and1000-tick continuation.

| Scale | Configuration / duration | TPS | Organism updates/s | Peak working set |
| --- | --- | ---: | ---: | ---: |
| Reference | seed11/512/founders50/cap200/regen12;50000 ticks/14.1096123s | 3543.683478815 | 705695.50660155 | 9068544 bytes |
| Medium | seed42/1024/founders1000/cap1000/regen100;10000 ticks/19.9510751s | 501.226121895 | 500154.049342 | 10727424 bytes |
| Density | seed42/1024/founders5000/cap5000/regen100;2000 ticks/34.0384361s | 58.757106059 | 291801.802257 | 18874368 bytes |

Post-warmup mean population200/999.5055/4999; density minimum4981/final5000. Density cases deliberately hold a technical cap to test performance, not ecological carrying capacity. Files: benchmarks/phase2-final-reference-200*, phase2-final-medium-1000*, phase2-final-density-5000*.

Apparent>30% historical slowdown was profiled: perception ~85% of instrumented time. Mate checks skip candidates unable to win and calculate bounded distance once; full50000-tick before/after hashes match. Contemporary sequential pair: unmodified v4 21.8963453s/2283.486TPS versus optimized v5 13.5118253s/3700.462TPS. Host variation and changed rules prevent a universal speedup claim. Contended/early extinct runs retain provenance and do not replace final scales.

Paused worker snapshot construction/JSON encoding,100 samples, eight calibration workers active:200 organisms114411bytes/median2.8321ms/p9512.347ms;1000 organisms218157bytes/median8.8869ms/p9532.4155ms;5000 organisms738501bytes/median30.4464ms/p9541.9294ms. Hashes unchanged. Excludes WebView transport/rendering and does not prove MAX responsiveness.

## Native UAT and artifacts

**UAT INCOMPLETE.** Development production launch, real morphology/inspector/species/tree/layers, native Save and Close observed. Restart failed with WebView2 resource-in-use HRESULT0x800700AA, exit101. Versioned WebView profile isolation preserves worlds/preferences and produced actual page-loaded production startup without panic. User then physically stopped Computer Use; no further app input. Repeated close/reopen, Load/continue/replay, legacy rejection UX, MAX, resize/dirty guards and final calibrated-build UAT remain unverified. Headless checks are not substitutes. Installer built, not interactively installed. See PHASE2_UAT.md.

Latest review build after calibrated default and corrected mouth legend:

- EXE: target/phase2-review/release/genesis-desktop.exe; SHA256 **1B9B9DBA6635FD52E5A3D1E6B5253DB88E0FD9908F6FA6E4B6FFABB3E3912A7F**.
- Installer: target/phase2-review/release/bundle/nsis/GENESIS_0.2.0_x64-setup.exe; SHA256 **BBF075FE7CEBACD4088A06329D1C7DB5DADBDBA7B9157D02EC96289A30B57B21**.

These are review artifacts, not a v0.2.0 release. Protected v0.1.0 binaries remain unchanged in target/release.

## Commits and remaining work

Branch phase2/multicellular-ecology. Main216a92ffca8e66e31789fc0a0e0eaf5085f13d76 unchanged; protected annotated v0.1.0 still targets ed6c77d4b876b859ed7a4b1d665f7bf9054b9096.

| Commit | Scope |
| --- | --- |
| 9e9fab9 | Baseline protection and v5 save policy |
| 3ce5cc1 | Inherited morphology, resource ecology, headless observations |
| 734ace9 | Morphology UI and bounded worker batches |
| 3560609 | Persistent cohorts/complexity analysis and benchmarks |
| 3751cbc | 0.2.0 review packaging and isolated v4 golden CI |
| 200f926 | Candidate rejection and provenance/pending UAT |
| d03561f | Versioned WebView profile, interrupted UAT |
| 651cfae | Atomic recorded initial habitat |
| 11b1908 | Verified showcase, snapshot timing, summary tooling |
| 741672f | Resource-limited default and seed4 showcase |
| 52ec2b2 | Complete100×100000 calibration evidence |

Final documentation/evidence commit follows this list and is identifiable from git log. No merge, v0.2.0 tag or release while gates remain incomplete.

Next required work: resume native UAT after explicit user authorization; retest repeated restart/profile recovery and all final-build file/replay/legacy/dirty/MAX flows; investigate genuine sustained multicellular/ecological differentiation against unchanged acceptance definitions. Parameter probes did not demonstrate it. Further authoritative-rule changes require an explicitly validated candidate/golden/save policy and fresh calibration, not relabeling these results.

Known limits: population-normalized niche exposure is not individual lifetime hazard; greedy clusters are not species; founder glyph is representative while means show actual current/last meaningful data; no safe v4 migration; no demonstrated sustained trophic balance or persistent complex niche expansion. Optional growth/development, full species comparison, extra terrain and richer overlays stay Phase2.1. LLM, diseases, parasites, neural networks,3D,cloud and later phases remain excluded.
