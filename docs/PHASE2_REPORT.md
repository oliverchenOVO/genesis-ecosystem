# Phase2 finalization report — INCOMPLETE

Current unreleased candidate: simulation5 / rules revision2 / RNG1 / save envelope1 / analysis4 / diagnostic4. Authoritative rules froze at57704c3d62cc67d8259ac1cef6d7f1f531742c29; subsequent0ca7a3b04dd2c2d6f6841c1e97af4b3b5d6a4cbe adds evidence/inspection tests only. The final report/data follow-up is identified by git log on phase2/multicellular-ecology. No authoritative changes occurred during the final100-seed run. No LLM is used in the simulation loop.

**Completed:** measured root-cause diagnostics before tuning; bounded paired candidate matrix; revised inherited morphology viability; explicit incompatible prerelease save/replay policy; intentional real v5 golden replacement; formal100*100000 validation; genuine complex-lineage showcase; production EXE/NSIS builds; isolated three-scale benchmarks; complete local regression; actual green private code-head CI.

**Still incomplete:** multiple persistent morphology/niche clusters and differentiated persistent trophic roles. Native UAT follows biological acceptance under the user's finalization order, so final repeated restart, save/load/continue/replay, legacy-v4 UX, MAX, resize and dirty guards remain unverified. Installer is build-only. Phase2 remains INCOMPLETE; no merge, v0.2.0 tag or private release.

## Final100-seed evidence

Seeds0-99, each100000 ticks;512 world,50 founders,technical ceiling2000,mutation100,20C/regeneration4. Default app seed42 unchanged. Wall time 3703.905768 seconds (61.731763 minutes), eight workers, includes diagnostics/full replay/save-load/1000-tick continuation and concurrent validation/build work; it is not isolated kernel TPS. Raw JSON and unordered flushed JSONL agree on all100 sorted rows; all900 overlapping cohort energy/population closures are zero.

| Measurement | Actual result |
| --- | --- |
| Survival / natural world extinctions | 100/100 / 0 |
| Collapse / Explosion / technical failures | 0 / 0 / 0 |
| Save/load/full replay +1000-tick RNG continuation | 100/100;0 mismatches |
| Safety ceiling worlds / fraction | 0/100 /0 in every world |
| Natural speciation | 18/100 worlds;20 new species |
| Species extinction | 5/100 worlds;5 extinct species |
| Persistent multi-unit population | 17/100; longest 55300 sampled ticks |
| Persistent high-C lineage | 13/100;38 unique lineages across worlds; per-world max 12 |
| Innovation species/world counts [multi-unit,armor,piercer,mass,sensory] | [0,1,0,0,0]; [0,1,0,0,0] |
| Maximum simultaneous persistent morphology clusters |1 in100/100; >1 in0/100 |
| Maximum simultaneous persistent niche clusters |1 in100/100; >1 in0/100 |
| Maximum simultaneous actual persistent feeding roles |1 in100/100; differentiated0/100 |
| Persistent prey-income role after warmup |0/100; longest qualifying sampled duration0 |
| Resource-dependent role duration |95000 sampled ticks in every world |
| Phenotype-label coexistence ticks | min 7498.000000 / median 20267.000000 / p90 27194.000000 / max 31299.000000 / mean 19997.060000 |
| Predator-like / consumer ratio (post-warmup means) | min 0.000089 / median 0.000248 / p90 0.000358 / max 0.000437 / mean 0.000254 |
| Predator-like starvation deaths / actual predations | 738 / 19922 |
| Population CV | min 0.157232 / median 0.245165 / p90 0.297272 / max 0.344251 / mean 0.244895 |
| Resource CV | min 0.637220 / median 1.162213 / p90 1.747528 / max 2.228408 / mean 1.229515 |
| Final population | min 909.000000 / median 1002.000000 / p90 1041.000000 / max 1093.000000 / mean 998.280000 |
| Across-world sampled morphology mean distance | min 0.000000 / median 0.000000 / p90 2.675079 / max 21.907466 / mean 1.100705 |
| Across-world sampled niche mean distance | min 0.000000 / median 0.000000 / p90 7.910620 / max 24.486856 / mean 1.918297 |

Persistence thresholds remain unchanged: multi-unit >=8 and>=25% living population for1000 consecutive sampled ticks; high-C lineage >=8 members with mean C>=200 for1000 ticks; same morphology/niche representative with drift<150 for1000 ticks. Sampling100 ticks, warmup min(ticks/10,5000). Actual trophic role additionally requires each100-tick window >=90% resource income for consumers or>=50% prey income for predator-like organisms, with>=8 living members and1000 sampled ticks. Phenotype-label coexistence allows rare/transient mutants and is not proof of sustained prey dependence. Historical kills alone also do not pass this gate. Rich classification100/100 describes genetic lineages/diversity and does not mean niche-rich ecology.

## Viability, bounded decisions and limitations

PHASE2_VIABILITY.md records prior measurements. Candidates A(structural mating cliff),B(mechanical hard-resource processing),C(A+B) all had0 persistent complex cohorts in eight50000-tick worlds. D additionally separates extra morphology storage from proportional mating-readiness penalty while retaining construction/maintenance/movement/maturity costs: two persistent multi-unit worlds and one high-C world in its eight100000-tick extension. D was frozen for final validation. E removed the dietary barrier and caused5/8 natural world extinctions; rejected. D with200 founders lost1/8 worlds and offered no high-C improvement; rejected as default. Full raw failures/source patches/source and executable hashes are preserved in phase2-candidate-* and phase2-finalization-provenance. No generic complexity bonus, global energy increase, outcome hardcoding, persistence relaxation or test removal.

Formal100-world pooled realized cohorts (overlapping inherited phenotype cohorts; complete lifetimes exclude censored survivors):

| C bucket | Intake/organism-tick | Actual expenditure/organism-tick | Completed lifespan | Completed offspring/parent | Successful mating/attempt |
| --- | ---: | ---: | ---: | ---: | ---: |
| C<80 | 4.442835 | 5.097856 | 1113.914410 | 2.032573 | 0.027165 |
| 80<=C<140 | 5.375965 | 6.091805 | 1049.576505 | 1.860962 | 0.030581 |
| 140<=C<200 | 6.443814 | 7.303701 | 982.656578 | 1.828125 | 0.029022 |
| C>=200 | 8.279355 | 9.376892 | 867.407478 | 1.879436 | 0.026775 |

High-C parent offspring:75154; high-C child retention70.149293%; same-lineage retention77.829523%. Exact initial/birth/food/prey/death/final energy closure is retained separately; intake minus expenditure alone excludes birth energy and cannot be used as a conservation residual. Cohort comparisons are exposure-weighted associations, not controlled causal effects. Baseline high-C completed offspring0.291139 versus formal revision2 1.879436. Complexity is viable in a subset of worlds but does not permanently dominate; many cohorts later recede. Natural speciation decreased from preserved R1's34 species/31 worlds to20/18, and extinction19/17 to5/5. Improved morphology viability does not solve ecological differentiation.

Most likely next architectural hypothesis: habitat/resource separation and local gene flow are insufficient to stabilize distinct niches. This is an inference, not proven by the current counters. Stay in Phase2: first measure feeding hardness/locations and inter-lineage mating, then test explicit renewable soft/hard resource channels and local mating/dispersal in a bounded paired matrix. Do not restore arbitrary structural sterility or lower clustering thresholds to manufacture diversity. No Phase2.1/3, juvenile stages, LLM narration or new product scope was implemented.

## Genuine showcase

examples/phase2-r2-seed7-tick40000.genesis: actual seed7/default ecology, tick40000,703 organisms,262 multi-unit(37.27%),18 living lineages. High-C lineage17 originated20425 from lineage4,27 members,mean C227,23 multi-unit members,real parents for all27,generations93-99; those living members already produced36 offspring. In a copied-world continuation, lineage17 stays>=8/mean C>=200 at every100-tick sample through41000,1000 sampled ticks. Original save/world unchanged. Full100k seed7 independently records33000 multi-unit sampled ticks and5 high-C lineages over the whole run; not5 concurrent saved cohorts.

Save SHA256 F5C5A91AF0B3735F94A6E45C569BD963E20D11FB7F517FAE70F0E9DB94B2713C; saved hash d4c8ccdcef711de6d8ff81e1bbbf59d2c48672323377f8d926e20616f5b6ad94; continuation07c88770d5e87e9fcc98f9897bde1cbf4d0ae2ec58d77d68bc7fcd0d3db2ac12. Independent locked inspector and authoritative CLI agree. Evidence phase2-r2-showcase*. One morphology/niche cluster and0 predator-like organisms; no trophic differentiation claim. Original R1 seed4 save remains unchanged and is intentionally rejected by R2 before decoding.

## Performance and validation

Sequential normal CLI, observer/profile features disabled; no concurrent compiler/calibration/native desktop process. Same configurations as historical R1 scales. Reference seed11/512/50 founders/cap200/regen12/50000 ticks; medium seed42/1024/1000 founders/cap1000/regen100/10000; density seed42/1024/5000 founders/cap5000/regen100/2000. These technical caps sustain legal performance populations and are not default ecological evidence.

| Scale | Kernel seconds | TPS | Organism updates/s | Peak working-set bytes incl.verification | Post-warmup mean population |
| --- | ---: | ---: | ---: | ---: | ---: |
| reference-200 | 6.063178 | 8246.500569 | 1642345.839795 | 9244672 | 200.000000 |
| medium-1000 | 7.114274 | 1405.624765 | 1402126.164887 | 10915840 | 998.945055 |
| density-5000 | 13.018060 | 153.632723 | 762931.189440 | 19288064 | 4999.263158 |

All scales pass full replay and1000-tick continuation. Historical R1 TPS3543.683479/501.226122/58.757106. No measured>30% regression; profiling is therefore not required. The current measurements are faster, but changed trajectories and host timing prevent a universal speedup claim. Raw phase2-r2-final-* plus host/compiler provenance retained. Frozen normal CLI file hash is in phase2-r2-freeze-provenance.

Real paused-worker snapshot+JSON encoding,100 samples/scale with eight calibration workers active:200 organisms114411bytes/median2.7948ms/p955.5294ms;1000 organisms217092bytes/median8.733ms/p9520.7642ms;5000 organisms738501bytes/median28.4645ms/p9540.0759ms. World hashes unchanged. Excludes WebView transport/rendering; no native FPS/MAX claim. Optional diagnostics stay headless and are not serialized per frame.

After final calibration:69 normal Rust workspace tests and73 all-features tests passed;24 frontend tests;5 Node evidence checks. Rust fmt/clippy all-targets/all-features, frontend formatting/lint/typecheck/build, v5 six-checkpoint golden, explicit old-R1/v4 rejection, full replay, save/load/RNG continuation, Tauri production and NSIS builds passed. Tests/thresholds were not removed or weakened. Protected v4 golden executes on actual v0.1.0 in CI and is green.

Actual private source-head CI: https://github.com/oliverchenOVO/genesis-ecosystem/actions/runs/37137134444, head 0ca7a3b04dd2c2d6f6841c1e97af4b3b5d6a4cbe, all jobs success. Previous57704c3 CI37134881580 and diagnosticf31837a CI37131492061 also success. Final report/evidence-only follow-up may run separate CI; inspect its head in git/GitHub before interpreting it as code changes.

## Native status and final packages

Final source0ca7a3b production package built after formal validation in independent target/phase2-r2-final. Build-only: no interactive NSIS install/uninstall or final native UAT. Repeated restart5 cycles, native save/load/continue/replay, legacy-v4 error UX, MAX, resize, invalid-file/recent-world and dirty guards remain pending because biological acceptance failed first. Historical R1 WebView2 HRESULT0x800700AA and profile recovery/physical Escape evidence remain in PHASE2_UAT.md; no R2 restart success or repeated failure is inferred.

| Artifact | Absolute path | SHA256 |
| --- | --- | --- |
| EXE | F:/CodeX開發小東東/AI 生態箱：從單細胞一路演化/target/phase2-r2-final/release/genesis-desktop.exe | CA7788C9BAA9506D6AB6AFF03B0A610E57EEBFBB3E9D805943DE30C4DCB565F1 |
| NSIS installer | F:/CodeX開發小東東/AI 生態箱：從單細胞一路演化/target/phase2-r2-final/release/bundle/nsis/GENESIS_0.2.0_x64-setup.exe | 2916F9660D9C9B7C888BE1454E46CDC477E3FD66284839128047D5AEBE8522F6 |

These are review artifacts, not a v0.2.0 release. Previous R1 and pre-calibration R2 review binaries remain at their separate paths. Main216a92ffca8e66e31789fc0a0e0eaf5085f13d76 unchanged/ancestor intact; annotated v0.1.0 still targets ed6c77d4b876b859ed7a4b1d665f7bf9054b9096. Protected EXE16FB27A061D6F817BDC68FE26897A6C69E1814FDB2110AD23C0A623ABE3C56C9 and installerEF09F5B0CEDACD1C5B62DF12776A4FF747E398A574E808054F51765821C115F4 unchanged. No merge commit, new main CI, v0.2.0 tag or release; existing repository remains private.

## Preserved history and new commits

[Unmodified R1 report at a3d8cc0](PHASE2_REPORT_R1_ARCHIVE.md), original100*100000 phase2-final-calibration.json/jsonl and phase2-final-summary.json retained. Pre-final source branch phase2/pre-finalization-a3d8cc0 preserved. Original logical commits and all failed probes remain. New commits:

| Commit | Scope |
| --- | --- |
| f31837a | Optional realized energy/fitness/mating diagnostics; R1 authority unchanged |
| 57704c3 | Causally measured selected D rules revision2, explicit save/replay policy, preserved old golden and real replacement, analysis4 persistence |
| 0ca7a3b | Genuine saved complex lineage and independent1000-tick proof; production review hashes;5 Node checks |
| Report/data follow-up (git log) | Complete100-seed R2 evidence, isolated performance, final package hashes and explicit INCOMPLETE decision |

The requested biological gates and native acceptance remain unfulfilled. Preserve the viable morphology changes, investigate ecological stabilization within Phase2, and rerun the same acceptance gates after a bounded evidence-supported change. No automatic transition to the next major Phase.

## Ecological finalization: unchanged-R2 diagnostic milestone

Source commit `7703ef1` adds optional habitat occupancy, realized feeding, mating
distance, symmetric gene-flow, offspring ancestry, dispersal and temporal-window
diagnostics. Simulation remains v5 / rules2 / RNG1 / envelope1 / analysis4;
diagnostic schema3 is separate from authority. The archived R2 golden is preserved.

[Measurement definitions and diagnosis](PHASE2_ECOLOGICAL_DIAGNOSTICS.md) records
three instrument iterations and their limitations. All eight seeds ×50,000 ticks
match the frozen normal R2 executable exactly, including refined schema3; save/load,
full replay and independent1,000-tick continuations pass with zero failures.
Cross-lineage mating world mean0.47899162408452234; refined cross-habitat mean
0.5741865561478211; mating-distance p90=23 against existing radius24. Initial coarse
habitat/productivity definitions are retained and must not be interpreted as rule
changes. Refined feeding-productivity overlap world means0.823071521149354–
0.929405977708568; environmental food uses one shared pool. The complex showcase's
maximum living-lineage nine-trait distance136 remains below unchanged threshold150.

Measured predator-like prey visibility131546/142789 organism ticks contrasts with
only14 compatible candidates among1097 nearby eligible mates. Piercer actual prey
income fraction0.7707490178271771 is pooled transient lifetime data, not a persistent
trophic gate. Resource homogenization and sexual bottlenecks therefore both matter.
No authoritative ecology, mating or dispersal adjustment is selected by this milestone.

Validation:67 normal release headless tests and2 native debug routing tests pass;
73 all-feature release headless tests (including2 added diagnostic tests) pass;
24 frontend and8 Node checks pass. fmt, all-target/all-feature clippy, frontend
format/lint/typecheck/build pass. A concurrent debug autosave test timed out at its
unchanged120-second limit; the existing release-profile test passes. Protected main,
v0.1.0 and Phase1 EXE/installer hashes remain unchanged. These are diagnostics,
not completed Phase2 biological/native acceptance. Next: paired resource separation
and local mating candidates, with no dispersal change until measured need.
