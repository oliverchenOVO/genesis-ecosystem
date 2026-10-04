# Phase 2 simulation v5 — development

Current ecological validation candidate is rules3 / analysis5, selected from
the completed bounded D8×100k probe. RNG1 and save envelope1 remain unchanged.
Two renewable fields use existing morphology processing and retained physical
costs; local mate radius is16 with the same genetic compatibility rules.
No dispersal change, region barrier or threshold relaxation is introduced.
Analysis5 reports combined environmental resource quantity; clustering and
actual prey-income persistence definitions remain unchanged. Headless ecology
diagnostic schema4 reports real soft/hard/prey income; it stays outside State
and frontend frame payloads. The compact food layer reports soft+hard stock.

The frozen source831f861 has completed formal100×100k with99 survivors,
one natural extinction, zero technical/replay/cap failures,12 persistent
multicellular worlds and21 high-C worlds (35 world–lineage pairs).
Morphology/niche/trophic differentiation and persistent prey income remain zero.
Rust75/81, frontend24, Node13, builds and code-head CI pass; paired final
benchmarks and the required reference slowdown profile are complete.
**PHASE2 INCOMPLETE**; native UAT, merge, tag and release remain gated by
failed biological acceptance. See PHASE2_REPORT.md for exact evidence and
the analytical soft/hard niche-vector limitation. The revision2
descriptions below preserve the earlier sealed baseline; see candidate evidence
and save policy for current revision3 provenance.

Sealed revision2 baseline result:100*100000,100 survivors/0 technical failures/0 caps; persistent multi-unit17/100 and high-C13/100. Multiple persistent morphology/niche clusters and differentiated trophic roles0/100. Phase2 INCOMPLETE; native UAT remains pending under the biological-first execution order. See PHASE2_REPORT.md for exact data, final package paths/hashes and green code-head CI. Earlier revision1/freeze checkpoints below are historical.

Authoritative rules changed once from simulation4 to5. RNG algorithm stays1; save envelope stays1. Additional loci/RNG draws, morphology-dependent energy and interactions, habitat fields and new species/history data change hashes intentionally. Old v4 fixtures are unchanged historical evidence and rejected by v5; they passed on the protected baseline before implementation. See PHASE2_SAVE_MIGRATION.md.

Headless features: inherited eight-locus structural expression, fixed-point phenotype, complexity/storage/construction costs, three mouth morphologies, prey-size constraints, defense and terrain interactions, coarse resource/temperature habitats, technical safety-ceiling diagnostics, species observations, representative actual founder genome/morphology, persistent innovations and read-only ecology measurements. No LLM. Frontend is never authoritative.

Species morphology summary indices: mean segments×1000, mass, armor, bite capacity, locomotion expression, sensory investment, complexity, speed, piercer fraction×1000. Records update every100 ticks and retain the last non-empty sample when extinct. A representative is the lowest-ID actual founder in the formation cohort, with its exact genome retained; it never changes with later population averages. Root and descendant origin environmental summaries store ambient temperature, regeneration and the actual representative founder habitat elevation.

Niche indices (0–1000): cumulative primary resource share of consumed resource+prey mass; mean killed prey mass/3; attacks received per current population×world duration, scaled1000; current occupied temperature shifted+26°C and divided10 centidegrees; current terrain elevation; current speed×50. The exposure estimate uses current population and is a rough diagnostic, not an individual lifetime hazard. Feeding observations record actual consumed resources, killed prey mass, kills and attacks received. Empty denominators give0 and raw observation counts remain available.

Innovation indices0–4: multi-unit morphology (mean segments≥2), armor≥300, predatory morphology (piercer fraction≥500 and actual species kills>0), mass≥1400, sensory≥700. Require at least8 living members and10 consecutive100-tick samples; reset streak below threshold. Emit once per species/innovation, never per mutation. Thresholds are explicit artificial-life observation rules. Founder snapshots and telemetry provide the underlying evidence; no event implies human biological equivalence.

Species niche/morphology pairwise distances use normalized fixed-point traits and mean absolute distance. Only living species with≥8 organisms participate. Distance is0 if fewer than2 species; organism variance separately describes within-species morphology. Niche clusters use deterministic greedy representatives ordered by SpeciesId with distance threshold150. Empty species set yields0 clusters. This is analytical grouping, not an additional species/reproductive rule. No O(N²) organism distance calculation is used.

Analysis version3 additionally records actual occupied per-species temperature min/max (centidegrees), morphology cluster counts, median species complexity distribution and cohort persistence. A high-complexity lineage requires at least8 living members with mean complexity≥200 for1000 consecutive sampled ticks. Niche persistence requires the same greedy representative SpeciesId and distance<150 from its streak-start profile for1000 ticks; reset on absence or larger drift. Report maximum simultaneous persistent clusters and unique high-complexity lineages observed, separately from generic genetic lineages. These observations never consume RNG or alter simulation state.

Headless implementation, v5 golden, save/replay continuation, compact UI, three-scale benchmarks and the final100-seed calibration have completed. The product candidate now uses seed42/size512/founders50/technical ceiling2000/mutation100/20°C/regen4. Seed4 uses the same preset for the verified natural showcase. Startup and the New World form record the initial habitat as a tick0 command. Custom API New requests may specify temperature/regeneration; omitted values retain20°C/regen12 semantics.

Phase2 acceptance is INCOMPLETE: final-build native UAT was interrupted by the user's physical Escape and must resume before release. Sustained multicellular/high-complexity cohorts and multiple persistent niche clusters were not observed under the documented thresholds; the implementation supports inherited multi-unit morphology, but does not claim that ecological depth has been demonstrated by the default distribution. See PHASE2_REPORT.md for exact evidence and limitations. Do not create v0.2.0 or merge this branch before remaining gates pass.

## Finalization revision2 / analysis4

The unreleased authoritative v5 rules are now explicitly revision2; details and retained old-golden failure are in PHASE2_SAVE_MIGRATION.md. Candidate D changes only structural mating cliffs, hard-resource Crusher processing and morphology-independent readiness reserve; no persistence threshold, default habitat, mutation distribution or safety ceiling is changed.

Analysis4 adds sustained morphology representatives using the existing normalized9-trait, mean-L1 distance150 clustering and >=8 species members. A representative must remain present with drift<150 from the streak-start profile for1000 sampled ticks. It adds actual feeding-role evidence under the optional viability observer: each100-tick window uses capped food/prey energy credited by real consumption. Resource consumers require >=8 members and>=90% resource income; predator-like consumers require>=8 and>=50% prey income. Roles persist only after1000 consecutive qualifying sampled ticks; report simultaneous roles and each longest duration. Phenotype labels or old cumulative kills alone do not satisfy this test. Warmup=min(ticks/10,5000); sampling100 ticks remains unchanged.

Diagnostic4 adds actual parent-child morphology/lineage retention and preserves exact overlapping-cohort energy/population closure. Draining feeding windows does not drain whole-run totals. Neither analysis nor diagnostics enters State, RNG, persistence, compact UI or the normal desktop simulation loop. Final100-seed results must be separate from the preserved revision1 run.
