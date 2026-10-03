# Phase 2 ecological diagnostics — R2 control

Status: **PHASE 2 INCOMPLETE**. This milestone instruments the frozen R2 rules;
it does not change simulation authority, acceptance thresholds or save policy.
Protected main, v0.1.0, production artifacts, the R2 100×100k run and showcase
remain provenance. Local branch `phase2/pre-ecological-finalization-b193c9b`
preserves the starting head. `fixtures/golden-v5-r2-archive.json` is an exact
copy of the current R2 golden, not a replacement hash.

## Observation method

The optional `viability` feature owns thread-local collectors outside `State`.
Normal desktop builds omit these collectors. Start at a world boundary and finish
before replay or restored continuation. No event consumes RNG or changes state.

- Occupancy: sample tick zero and every 100 ticks; aggregate lineage and species
  8×8 spatial histograms, habitat histograms, position moments, occupied temperature,
  productivity, terrain and moisture. Birth origins exist only for living organisms.
  Dispersal and centroid distances use floor integer Euclidean distance, world units.
  Habitat switches are sampled lower bounds, not every crossing.
- Habitat bits in diagnostic schema 3: elevation ≥500, moisture ≥500, fertility ≥100.
  R2 fertility is 60–140. Schema 1/2 used fertility ≥500 (an inactive bit) and wider
  productivity bins; their overlap values must not be compared to schema 3 as an
  ecological effect. They are retained as instrument development provenance.
- Feeding: positive **credited** energy events, aggregated by lineage, species and
  mouth. Hardness bins width 100, productivity bins width 20, prey mass bins width 100;
  ten bins per feature, last bin clipped. R2 has one environmental channel (0),
  reserved channel 1 is unused by construction, channel 2 is actual prey income.
  This is a measured source distinction, not a mouth-based diet label.
- Successful mating: count one parent pair that produces at least one child.
  Record distance, genetic distance, cross-lineage/species/habitat/mouth counts.
  Symmetric parent edges sum to twice pair count; parent→child edges sum to twice
  births. Gene-flow row fractions use all partners in the denominator; omitted
  minor partners can make displayed top-16 row fractions sum below one.
- Major lineages: at least eight sampled residents for 1,000 ticks; select top 16
  by whole-run occupancy exposure, break ties by ID. This analytical selection
  is separate from unchanged biological acceptance and high-C definitions.
- Overlap: histogram intersection `sum(min(a[i]/sum(a), b[i]/sum(b)))`, in fixed
  bin/ID order. Empty distributions produce null, not fabricated overlap zero.
  Report spatial, habitat, feeding hardness/productivity/prey-size and actual
  energy-source overlap separately. A pooled whole-run score does not establish
  simultaneous coexistence. Schema 2/3 also retain 5,000-tick windows of occupancy,
  mating edges/distance and credited energy by mouth/channel.
- Mouth and lineage energy/fitness ledgers retain exposure, prey perception,
  hunting, attacks/kills, costs, starvation, compatibility, mating and offspring.
  Hunting movement is **requested** movement energy conditional on Hunt behavior;
  depleted organisms can pay less than the request. Whole-run actual energy closure
  remains in the existing cohort ledger.

## Initial paired measurements

`benchmarks/phase2-r3-diagnostics-r2.json` and
`benchmarks/phase2-r3-paired-frozen-r2.json`: seeds 0–7, 50,000 ticks each,
size 512, founders 50, cap 2,000, mutation 100, 20°C, regeneration 4.
Every final hash agrees between observer and preserved normal R2 executable.
Both runs report zero failures; all worlds pass save/load, full replay and
1,000-tick independent continuation. These concurrent calibration timings are
not isolated production TPS benchmarks.

Schema-1 instrument results:

- Cross-lineage mating fraction: world mean 0.47899162408452234;
  range 0.3695157415602478–0.529880880274581.
- Cross-habitat mating fraction using the initial two active habitat bits:
  world mean 0.2497437446237773; refine this with schema 3.
- Actual mating distance p90: 23 in each world, against an existing radius 24.
- Grazer/Crusher/Piercer mean food hardness:
  435.496942347321 / 450.5030632062775 / 464.634557712147.
- Their mean occupied feeding productivity:
  80.96953899492105 / 79.19759248598925 / 80.80422157295331.
- All environmental credited income uses the same pool. Major persistent
  lineages' realized energy-source overlap is nearly one, including pairs with
  low spatial overlap. Spatial separation alone is not functional resource
  specialization.
- Piercer pooled prey-energy fraction is 0.7707490178271771. Its parent offspring
  rate is 0.00018796499479817804 per organism tick, compared with Crusher
  0.0018978095269240795. This includes transient founders; it is **not** sustained
  prey-income acceptance. Environmental-food leakage alone is not an adequate
  explanation for failed predator persistence.

The read-only `lineage-profiles` example compares the genuine R2 seed-7 tick-40k
showcase using the existing nine-trait normalization and integer L1/9 distance.
Ten living lineages meet population ≥8, maximum pair distance is 136, zero pairs
reach 150, species cluster count is one. Original world hash remains
`d4c8ccdcef711de6d8ff81e1bbbf59d2c48672323377f8d926e20616f5b6ad94`.
This snapshot does not show a qualifying second morphology cluster hidden solely
by species averaging, and does not make a temporal persistence claim.

## Interpretation and next bounded experiment

Resource homogenization has a real architectural component: actual consumers
share one renewable pool even across spatially separated branches. Spatial and
hardness differences also exist, so species averages can conceal subthreshold
variation. Do not change cluster cutoff 150 or persistence 1,000 to address this.

Measured cross-lineage mixing and mating distance concentrated near the existing
radius justify a **paired local-distance experiment**, not a direct niche-based
fertility barrier. Revised habitat diagnostics and temporal windows must be read
before interpreting its effects. No dispersal change is selected from these
whole-run distances alone: later lineages can already retain small ranges.

Next: finish refined unchanged-R2 controls; test minimal independent renewable
soft/hard resource fields and local mating separately on the same seeds. Preserve
complex-form costs/readiness, ancestry, mutation distribution and RNG algorithm.
Reject failed candidates before choosing a revision-3 rule set. Biological gates,
100×100k, production/native UAT, benchmarks and actual private CI remain required
before completion, merge, tag or release.

The refined schema-3 control is complete: the same eight final hashes agree with
the frozen R2 executable, zero failures, eight verified replay/continuation runs.
Cross-habitat mating with all three active habitat bits has world mean
0.5741865561478211 (range 0.5355509148766905–0.5978631938298141). The higher
value is an instrument definition change, **not** changed mating behavior.
Mean pairwise feeding-productivity overlap by world is 0.823071521149354–
0.929405977708568 using width-20 bins; actual energy-source homogenization
remains. Temporal mating, symmetric gene-flow and feeding-energy windows close
against their whole-run event ledgers. R2 authority remains revision 2.

For predator-like organisms, prey is perceptible for 131,546 of 142,789 organism
ticks; 891 kills credit 748,593 energy, compared with 93,709 environmental energy.
There are 25,162 initiating mate searches, 1,097 nearby eligible candidates,
but only 14 compatible candidates and 42 parent offspring. This is evidence of
a sexual bottleneck despite prey availability. A smaller mate radius must be
evaluated for that risk, and cannot be assumed to solve predation persistence.

Local validation: 67 normal release tests plus two native routing/close-listener
debug tests passed (69 combined). 73 release all-feature tests passed (including
two new ecology tests); native tests add two for 75 combined. Frontend 24 and
existing Node 5 pass; ecological Node checks additionally validate partial evidence,
event-ledger consistency and diet units. Formatting, lint, typecheck, frontend
production build and all-target/all-feature clippy pass. One concurrent debug
workspace run hit the existing 120-second autosave timeout; the same unchanged
test passes in the project's existing release validation profile. Its timeout and
expectations were not relaxed. This is not native UAT or installer acceptance.
