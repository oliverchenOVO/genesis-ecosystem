# R4 bounded resource-patch experiments

Status: all three patch-only candidates rejected. Predator capture-only experiment also completed and rejected; see the final PHASE2_REPORT.md. No rules4 authority has been selected; rules3 and the original biological gates remain in force.

Each isolated candidate uses the same seeds0..7 ×50000 ticks, size512, population50, cap2000, regeneration4, mutation100 and temperature2000 as the frozen R3 control. Only the resource productivity field changes. Source is preserved as exact binary Git patches based on321ab99, with executable SHA256 and complete JSON/JSONL evidence. Diagnostic runs are not normal-performance measurements.

The prototype interpolates a pure integer seeded coarse field at cell centers, multiplies soft productivity by the field and hard productivity by its complement, and caches both factors in cells. It never draws from the world RNG. Both channels stay positive everywhere. Each channel is normalized against its original soil-weighted total potential production; integer rounding reduces global potential by less than0.1%, rather than adding global food. Existing terrain, initial stocks, feeding efficiency, damage, movement, mating radius16 and compatibility are unchanged. Regrowth remainders use the corresponding scaled denominator. Three extra core tests cover interpolation/normalization and save/load/replay continuation; all60 prototype all-feature core library tests passed per geometry. These are isolated probes with distinct payload tags, not final R4 golden or full authoritative regression validation.

| Candidate | Survival | Persistent MC | High-C worlds | Morphology>1 | Niche>1 | Actual feeding roles>1 | Persistent prey income | Diagnostic elapsed seconds |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Frozen R3 control | 8/8 | 3/8 | 2/8 | 0/8 | 0/8 | 0/8 | 0/8 | 850.84 |
| Patch32 | 7/8 | 0/8 | 0/8 | 0/8 | 0/8 | 0/8 | 0/8 | 907.14 |
| Patch64 | 8/8 | 0/8 | 2/8 | 0/8 | 0/8 | 0/8 | 0/8 | 1731.76 |
| Patch128 | 7/8 | 0/8 | 0/8 | 0/8 | 0/8 | 0/8 | 0/8 | 1310.72 |

All24 candidate worlds passed save/load, full replay and1000-tick continuation, with zero technical failures or safety-cap contacts. Patch32/128 each had one natural extinction. Both resource channels were used in every surviving world's post-warmup observations; no resource-monoculture rejection occurred. Gene-flow and movement/extent observations remain available in the full temporal ledgers. No walls or geography-dependent sterility were introduced.

All patch geometries lose every paired persistent multicellular world; Patch32/128 also lose every persistent high-C world. Thus none meets the complexity-preservation requirement. A lower cross-lineage mating fraction does not rescue those failures. No patch is selected and no patch/predator combination is justified.

The fields do not guarantee large realized production patches: the original fine-scale soil factors still multiply the coarse field. For Patch32 the initial top25% soft-productivity correlation length is32–48 units, hard48–64; Patch64 soft48, hard48–64. Nominal generator scale is not interchangeable with measured resource correlation length. Full Patch128 values, component areas/perimeters, temporal patch overlaps, both1000/5000 diet windows and predator budgets are in the companion review and temporal-summary archives. Soft/Hard diet persistence remains diagnostic and cannot count toward the original resource/prey coexistence gate.

`benchmarks/phase2-r4-patch{32,64,128}-provenance.json` identifies every lossless compressed raw/derived archive and source patch by SHA256. The journal rows were independently compared with final results; temporal, individual, density and energy/population ledgers all closed before archiving. `scripts/archive-r4-probe.mjs NAME` reproduces archival validation from fresh complete `artifacts/phase2-r4-NAME.*` inputs and refuses existing outputs. The analyzer additionally rejects rules4+ candidates if a resource channel is wholly unused after warmup in strictly more than half the worlds; missing post-warmup windows remain unknown rather than fabricated zero usage. This strengthens the R4 rejection rule without changing any ecological acceptance definition.
