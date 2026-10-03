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
main/v0.1.0 and binaries remain untouched. Long-run selection and final100×100k
remain pending; native UAT is conditional on biological gates passing.
