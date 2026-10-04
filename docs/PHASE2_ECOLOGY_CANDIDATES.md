# Bounded ecological architecture experiment

Status: **PHASE 2 INCOMPLETE**. This records experimental evidence, not a
selected production rules revision or biological acceptance.

All initial scenarios use seeds 0–7, 50,000 ticks, size512, 50 founders,
limit2000, mutation100, temperature2000 and regeneration4. A is unchanged R2
with schema-3 read-only diagnostics. B/B2/C/D retain the original morphology
costs, reproductive readiness/capacity and genetic compatibility rules. No
dispersal change, habitat wall, niche sterility or clustering relaxation is used.
Clustering distance150 and sampled persistence1000 ticks remain unchanged.

| Candidate | Change | Survival | Persistent multi-unit worlds | Persistent high-C worlds | Mean cross-lineage mating | Mean cross-habitat mating |
|---|---|---:|---:|---:|---:|---:|
| A | Unchanged R2 | 8/8 | 2 | 1 | 0.47899162408452234 | 0.5741865561478211 |
| B | Two renewable resource fields, hard450–950 | 8/8 | 0 | 0 | 0.5145646512893827 | 0.5794170332956098 |
| C | Mate radius24→16 only | 8/8 | 1 | 0 | 0.47307335474146284 | 0.44920286119399805 |
| B2 | B with hard300–633 | 7/8 | 0 | 2 | 0.4396307878469665 | 0.5616331021846477 |
| D | B2 plus mate radius16 | 8/8 | 3 | 2 | 0.5134257979053622 | 0.47032630475988385 |

All five settings have zero completed-run technical/replay failures and zero
safety-ceiling contact. All retain just one persistent morphology cluster,
one persistent niche cluster and one actual persistent feeding role; none has
a persistent prey-income cohort. B2 seed1 is a natural extinction, separately
classified from a technical failure. Each candidate's eight JSONL rows exactly
match its final JSON results after sorting by seed.

B loses both complex cohorts and is rejected. C shortens mating p90 from23 to15,
but hardly reduces cross-lineage gene flow and loses persistent high-C; it is
rejected as a standalone change. B's high-C intake falls from8.785879903153099
to6.210368594545999 energy per organism tick. One bounded correction adjusts
only the newly introduced hard-channel processing range to300–633, retaining
R2 body costs/readiness. B2 restores high-C but no persistent multi-unit world,
so it is not selected alone. D preserves both complex gains and is promising
enough for a paired 8×100,000 extension. It does **not** yet reduce cross-lineage
mixing or demonstrate biological acceptance.

The fields divide the original starting chemical budget: soft+3×hard never
exceeds the old food pool. Soft regeneration depends on moisture; hard depends
on elevation and dryness. Integer carried fractions make slow hard regeneration
nonzero without extra RNG draws. Processing uses existing food efficiency,
carnivory, mouth resource efficiency and bite capacity; larger Crusher forms
retain their existing maintenance, movement and reproductive costs. Details are
in each immutable `.patch` and source/executable/golden SHA256 provenance file.

Resources-only B/B2 have 53 passing core viability library tests; C has52;
D has54. Candidate core/benchmark all-target/all-feature clippy passes. Each
initial world verifies save/load, full replay and independent1000-tick RNG
continuation. Probe goldens are separate measured artifacts; the current R2
golden remains unchanged. Probe save tags deliberately distinguish settings.

The B executable's English diagnostic annotation retains R2 wording due to an
edit/build overlap; its structured channel names and actual channel counters
are correct. This limitation is explicitly recorded in its provenance. B2/D
annotations were corrected before their builds. Original data are preserved.

The initial A 100k extension exited with Windows allocation failure
(`memory allocation of 4099390 bytes failed`, exit0xC0000409), before any complete
seed record. Empty outputs and stderr are retained in ignored artifacts. A is
retried with one worker; rules/configuration/seeds/ticks remain identical.
This failed invocation is an execution issue, not an extinct simulated world.
No partial run is eligible for candidate evaluation.

Machine-readable comparisons: `benchmarks/phase2-r3-initial-matrix.json`.
Per-scenario original JSON/JSONL, summaries, patches, probe goldens and provenance
use separate `phase2-r3-candidate-*` names. R2 formal100×100k, showcase, protected
main/v0.1.0 and binaries remain untouched. The stage below records the subsequent
long-run selection; native UAT remains conditional on biological gates passing.

## Completed paired100k extension and selection

The A retry completes all8 seeds; each authoritative hash matches sealed R2
100k evidence, JSON/JSONL rows agree, all replay/save/continuations close.
The allocation-failed invocation remains separately documented above.
Both A and D have8 survivors, zero completed-run technical/replay/cap failures.
A has2 persistent multi-unit and1 high-C world; D has3 and2 (3 high-C lineages).
Both have zero worlds meeting persistent morphology/niche/trophic differentiation
or prey-income gates. Mean cross-lineage A0.48577273560644685 versus
D0.5841650703643353; cross-habitat A0.5880522490506069 versus
D0.49738592855325925. Mating p90 A23 versus D15. Shortening radius reduces
cross-habitat mating but does **not** establish lower cross-lineage mixing.

D is selected for formal expansion as the only resource-separated candidate
retaining both complex-form gains. It is a validation candidate, not biological
acceptance. Mechanical short-run hard-income fractions are Grazer0.11858573616522688
and Crusher0.6114486602626756. Piercer pooled prey fraction0.8746944895735231
remains transient, not a sustained prey role. No further physical parameter
tuning is added. The final100-seed study remains required.

Independent standalone D seed1×100k matches the diagnostic world's full hash;
15 living lineages have maximum normalized morphology distance42 and zero
pairs>=150. This snapshot does not show qualified hidden lineage clusters or
prove persistence. It is kept separately from any future official showcase.

Selected rules3 use final `GENESIS5RULES003`, simulation5/RNG1/envelope1/analysis5.
R2 and experimental probe saves reject explicitly. The actual generated current
golden matches all six frozen D checkpoints; R2/v4 archives remain unchanged.
Normal full workspace regression has75 passing tests; all-feature validation
has81 passing tests. fmt and all-target/all-feature workspace clippy pass.
Additional evidence checks now total13; frontend24/build/checks
remain green. Candidate-evidence CI89d622c is green in all3 jobs (normal69,
Node11, frontend24, Tauri/NSIS build), on its unchanged-R2 source tree.

Exploratory selected-R3 preflight TPS: reference1641.51526580142,
medium331.091851431895, density41.6537814165003. Fresh same-host R2 rechecks:
3055.64142756756 /484.212603944649 /50.0308833138064. These runs overlap R2
diagnostic calibration and R3 regression compilation, with uncontrolled external
host load; they are not final isolated acceptance measurements. Original R2
isolated numbers remain unchanged. The rough reference/medium deficits motivate
profiling: selected-R3 density profile matches the normal hash exactly and
spends46.8968605 seconds in perception out of49.5349629 seconds of marked phases.
The food-cell loop calculates both channel efficiencies and then recalculates
the selected one; a minimal bit-identical correction is the next technical task.

## Bit-identical technical correction before formal expansion

Selected channel/stock/efficiency are returned together, reducing each resource
opportunity from three efficiency evaluations to two without changing operation
order or soft-channel ties. The existing processing-cost test additionally checks
zero-yield ties and observed supply. All six golden checkpoints and all three
complete benchmark endpoint hashes match the unoptimized selected D exactly.
Full optimized regression passes75 normal /81 all-feature tests, fmt and
all-target/all-feature workspace clippy. The final viability executable's two-seed
1000-tick preflight verifies complete JSON/JSONL equality, all diagnostic closure,
save/load, replay and independent1000-tick continuation.

Calibration report assembly moves its completed `Value` array instead of
serializing/cloning every diagnostic ledger a second time. No data are omitted;
the preflight rows retain every field. This reduces final report memory pressure
after the earlier recovered R2 allocation failure.

Optimized preliminary TPS:2092.20057627322 /354.169944509503 /50.7030763632721.
They run sequentially with task compilers/calibrations stopped and no native app;
external host load remains uncontrolled. Compared with earlier unoptimized R3
preflight these are about27%,7%,22% faster, but contexts differ, so this is not a
controlled effect-size claim. Final same-host R2/R3 checks still follow the formal
100-seed study. No authoritative rules, RNG, thresholds or current golden change.

Lineage source-overlap means across the D extension are0.8720404690885815–
0.9500105612499526, with some pairs as low as0.5970046387404786. This is partial
realized resource separation in pooled lifetime data, not proof of continuous
1000-tick feeding-niche persistence. Existing species-based niche vectors retain
combined resource/prey mass and do not directly encode a soft/hard axis. Formal
acceptance definitions remain unchanged; this analytical limitation must be kept
separate from the genuine failure of the unchanged sustained prey-income gate.

## Completed formal expansion of the frozen D candidate

Source831f861 / simulation5 / rules3 / RNG1 / envelope1 / analysis5 / ecology4
completes100 seeds×100000 ticks in10891.9436458 reported seconds. All100
verify save/load/full replay/independent1000-tick continuation; all900
overlapping cohort closures and temporal mating/gene-flow/energy closures
pass. Complete JSONL rows equal final JSON, and the first8 endpoint hashes
match frozen D exactly. Full raw ledgers are preserved as lossless gzip
with byte-for-byte decompressed SHA256 verification; no data are cropped.

Survival99, natural extinction1(seed43), technical/replay/cap failures0.
Persistent multicellular12/100, high-C21/100 (35 world–lineage pairs).
Speciation7 worlds/9 new species; extinction5 worlds/6 extinct species.
Persistent morphology/niche differentiation, actual differentiated feeding
roles and sustained prey income all remain0/100. **PHASE2 INCOMPLETE**;
no additional tuning, later Phase, merge, tag or release follows this result.

Successful mating pairs4413350; pair-weighted cross-lineage0.6074238390338405
and cross-habitat0.49644351796254543; pooled distance mean12.908427838263451,
median14,p90=15. No reduced cross-lineage mixing is demonstrated by the
paired A/D extension. Lifetime source-overlap world means average
0.9047529960701577 across99 eligible worlds; this is partial resource-use
separation, not temporal lineage-specialist acceptance.

Both environmental channels supply credited energy in all1881 active
post-warmup windows of99 worlds. Pooled Grazer hard fraction0.11503700601150393,
Crusher0.6046300150837033, Piercer actual prey fraction0.5180864150836552.
The strict predator-like cohort's actual prey fraction0.8959289854173628
still does not yield a qualifying sustained prey cohort. Species niche
vectors combine resource units/prey mass without a direct soft/hard axis;
ecology4 has temporal mouth income and lineage occupancy, but lifetime-only
lineage diet data. Preserve this analytical limitation and the unchanged
acceptance failure separately.

Final R3 TPS3567.448191109341 /586.5427319079399 /82.65501872305617.
Same-host fresh R2 TPS5894.054859009905 /832.3077013606392 /89.3660164337223.
Reference slowdown39.47378712201861% triggers profiling: perception
11.6716762/13.1044635 marked seconds, with identical normal endpoint hash.
No new whole-population resource/mate scan is introduced. External host
load is uncontrolled; original R2 benchmarks remain preserved.

Final regression75 normal /81 all-feature Rust, frontend24, Node13 passes;
targeted debug autosave recovery also passes without changing its120-second
advance deadline. Tauri/NSIS CI builds pass and remain BUILD ONLY. Native
UAT is not performed before biological acceptance. PHASE2_REPORT.md records
full provenance, hashes, CI and the next measured Phase2 hypothesis.
