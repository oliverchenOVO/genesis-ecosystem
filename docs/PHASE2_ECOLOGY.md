# v5 ecology rules — initial headless implementation

Current rules revision2 final result:100*100000,100 survivors/0 technical failures/0 caps; persistent multi-unit17/100 and high-C13/100. Multiple persistent morphology/niche clusters and differentiated trophic roles0/100. Phase2 INCOMPLETE; native UAT remains pending under the biological-first execution order. See PHASE2_REPORT.md for exact data, final package paths/hashes and green code-head CI. Earlier revision1/freeze checkpoints below are historical.

World cells retain food, fertility, temperature, elevation and moisture. Seed-derived four-cell habitat bands add coarse temperature/terrain/moisture heterogeneity with bounded fine noise. Cell productivity=fertility×(200+moisture)/1000; regeneration=environment regeneration×productivity/100, capped at1000 resource units/cell. Stable organism-ID order allocates shared food. Spatial temperature affects inherited tolerance; mutation does not respond directionally to habitat.

Elevation represents both resource hardness and rough terrain. Grazer processing factor125−hardness/12 favors soft resources; revision2 crusher65+hardness/12+hardness*clamp(bite_capacity-hardness/20,0,20)/50 favors mechanically processable hard resources; piercer35 sacrifices plant efficiency. Effective plant conversion combines processing, inherited food efficiency and (1000−carnivory)/1000. Intermediate inherited carnivory can support generalist resource/prey use; no species category assigns diet.

Terrain speed=max(1,speed×1000/(1000+roughness×(mass/100+armor/100)/20)). Movement energy=(abs(dx)+abs(dy))×movement coefficient/500. Larger/armored bodies pay more for rough habitats. Thermal loss=excess/(100+mass/50), with inherited tolerance and actual local temperature.

Only non-grazer mouths with bite_capacity×40≥prey mass can attack; prey perception also applies this constraint. Bite capacity=mass/50 + mouth bonus4/12/22. Contact attacks use a10-tick cooldown, cost20+mass/40, deterministic damage=max(1,(bite+aggression/50+(attacker speed−prey speed)/2)×(1000−prey armor)/1000). Fatal attacks reward prey stored energy/2+prey mass/4, bounded by attacker capacity. Speed, size, armor and mouth all affect success/cost/reward. There is no invented kill probability or RNG consumption.

The technical population limit remains a safety ceiling. Resource/metabolic/reproductive balance must be evaluated in new candidate calibrations; existing v4 defaults are not yet replaced. Population saturation is not declared solved merely because new energy formulas exist. Diagnostics, species/niche telemetry, long-run calibration, performance and native UI remain separate acceptance gates.

## Revision2 ecological comparison

Selected D retains default regeneration4, temperature20C,50 founders and ceiling2000. Crusher hard-resource efficiency now depends on actual bite capacity; maintenance, movement and construction remain paid. Bounded D produced persistent inherited multi-unit/high-C cohorts, unlike A/B/C individually; dietary-barrier removal E caused5/8 world extinctions and is rejected. All candidates and provenance are in PHASE2_FINALIZATION.md. Morphology/niche distances and energy-dependent trophic persistence are still acceptance gates; multiple generic genetic lineages or historical kills do not prove ecological differentiation.
