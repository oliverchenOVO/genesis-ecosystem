# Phase 1.1 Ecology Calibration

Simulation v4 / RNG v1 remains unchanged. The original Phase 1 configuration and artifacts are preserved, including the 100×50,000 stress result and isolated seed-11 sustained benchmark. Product default remains 50 founders / capacity 200 / size 512 / mutation 100 until completed measurements justify a change.

## Formal runner

`sim-benchmark --calibrate --seeds 250 --ticks 100000 --workers 8 --population 50 --limit 200 --temperature-sweep --label "Phase1 Baseline extended" --output benchmarks/calibration-baseline.jsonl`

Standard output is a JSON report; `--output` flushes one JSON object per completed seed so progress survives interruption. World workers are independent. Each world is validated every 1000 ticks and at completion. Each row stores seed/config/environment/hash and lifecycle/measurement results; errors are reported per seed and fail the runner. No metrics feed back into simulation.

Hardware: Windows 11, Intel i9-12900H, 14 cores / 20 logical processors, Rust 1.99.0 / MSVC. Batch throughput includes concurrent world execution and must not be compared to isolated benchmark timing.

## Sampling and interpretation

- Population workload counts living organisms before **every** tick. Peak population comes from the authoritative lifecycle counter.
- Analytical samples every 100 ticks, after warmup `min(ticks/10,5000)`. Minimum after warmup and persistence durations are sampled estimates, not claims about every intermediate tick.
- Diversity is mean absolute genome distance to the current population centroid, averaged over all 14 loci. It is an approximation, not pairwise distance. Final phenotype variances cover body size, speed, vision, metabolism, tolerance and aggression. Food is mean resource units per cell. Temperature includes the configured ambient value and spatial offset distribution.
- Mean completed lifespan uses all ancestry deaths; living individuals are right-censored and excluded. Zero deaths produces null, not a fictional mean.
- Population volatility is the coefficient of variation of post-warmup samples. A large swing is an adjacent sample change of at least 10% of configured capacity.
- A persistent lineage has at least eight living members for 1000 consecutive sampled ticks. Richness also tracks consecutive multi-species samples. These are analytical indicators, not new species rules.
- Summary distributions report min, median, p90, p95, max and mean. Quantiles select the nearest observed rank; conditional first-speciation statistics exclude worlds without speciation and explicitly report their count.

## Deterministic classification (priority order)

1. **Collapse**: final population zero, or population below `max(2,capacity/20)` for at least 1000 consecutive sampled ticks after warmup.
2. **Explosion**: final population exceeds configured capacity. The current core hard cap should prevent this; do not label ordinary cap saturation as uncontrolled growth.
3. **Oscillating**: population CV ≥ 0.25 and at least four large adjacent-sample swings.
4. **Rich**: at least two living species for 1000 consecutive sampled ticks, or at least three persistent lineages and mean centroid distance ≥ 30.
5. **Sterile**: mean centroid distance < 10, CV < 0.02 and fewer births than initial population.
6. **Healthy**: remaining surviving worlds with turnover/diversity evidence. This is a screening category, not a scientific guarantee.

Lack of a new species alone never fails a world. Classification output is separate from authority and uses no LLM.

## Bounded parameter matrix

All use existing legal configuration or existing recorded environment commands; no core tuning knobs are added.

| Label | Seeds / ticks | Founders / limit | Size | Mutation | Temperature / regeneration | Hypothesis |
|---|---|---|---|---|---|---|
| Phase1 Baseline extended | 250 / 100,000 | 50 / 200 | 512 | 100 | 18–22°C seed sweep / 12 | Longer observation distinguishes slow divergence from sterile ecology |
| capacity-1000 | 8 / 20,000 | 50 / 1000 | 512 | 100 | 20°C / 12 | Determine whether 200 is simply the configured carrying cap |
| dense-resource-space | 8 / 20,000 | 500 / 2000 | 1024 | 100 | 20°C / 60 | More cells/resources and less initial crowding may sustain denser ecology |
| divergent-2x | 8 / 50,000 | 50 / 200 | 512 | 200 | 20°C / 12 | Test mutation sensitivity without changing speciation criteria |
| warm-habitat | 8 / 20,000 | 50 / 200 | 512 | 100 | 28°C / 12 | Test existing inherited thermal pressure |

Food capacity remains 1000/cell and seeded fertility/temperature heterogeneity stays unchanged. Reproduction threshold/cost, metabolism, movement cost and predation are genome-derived or fixed v4 rules, not independent config fields. Their coupling is examined through measured traits, turnover and predation; adding independent rule parameters would change the authoritative model and is deferred unless current results demonstrate a correctness defect.

## Prior result investigation

The 9/100 second-species outcomes at 50,000 ticks were a finite observation window. In initial production UAT, unchanged seed 11 naturally produced its first new species at tick 48,600 and by tick 701,520 had 22 recorded species, 9 living, with historical extinctions. That single long run is evidence of capacity for divergence, not a population-wide rate estimate.

The 200 final population matches the selected hard cap; it does not measure unconstrained carrying capacity. Wider-cap and resource/space experiments test that distinction. Gene flow, local mating, random recombination and branch persistence can delay recognition: genome divergence alone is insufficient because ≥8 founders, births, limited cross-lineage gene flow and sustained eligibility are required. Standing phenotype pressure is coupled to energy and diet, and strong predation can collapse dense founder populations. The matrix and trait variances provide evidence before any rule change.

## Showcase

Official **Showcase · Seed 11** is selectable in New World: seed 11, size 512, founders 50, capacity 200, mutation 100, default 20°C/regen 12. Only seed and legal config are fixed. Population growth, allele variation, lineages and speciation are real simulation outcomes. Original example `examples/seed11-107504.genesis` remains available as evidence from a genuine run. Replay validation is required separately from visual interest.

## Completed results

Completed results follow; isolated performance is recorded in PHASE1_1_REPORT.md.

### Final measurements and decision

The completed 250 × 100,000 run is retained: 25,000,000 ticks, 250 survivors, zero failures/collapse, 32 worlds with natural speciation (12.8%), 35 new species in total, four worlds with one species extinction each (1.6%). No authoritative simulation code changed after this run. UI, preferences and file permissions cannot invalidate its world results.

| Category | Sterile | Healthy | Rich | Collapse | Explosion | Oscillating |
|---|---:|---:|---:|---:|---:|---:|
| Worlds | 0 | 0 | 250 | 0 | 0 | 0 |

218 Rich worlds formed no new species: their persistent lineage/diversity measurements satisfy the separate Rich rule. Rich does not mean a multi-species ecosystem. Population CV is zero after warmup in all 250 worlds: this baseline has continual turnover at a hard cap, but little net population variation. Median births/deaths are 11,364/11,214, completed lifespan 1,762.93 ticks, predations 31 (maximum 310), persistent lineages 8 (range 3–12). These limits must remain visible when interpreting the classification.

| Baseline distribution | Min | Median | p90 | p95 | Max |
|---|---:|---:|---:|---:|---:|
| Final / peak / minimum population after warmup | 200 | 200 | 200 | 200 | 200 |
| New species | 0 | 0 | 1 | 1 | 2 |
| Final living species | 1 | 1 | 2 | 2 | 3 |
| First speciation tick, conditional n=32 | 26,800 | 67,800 | 91,700 | 92,300 | 98,600 |
| Final centroid distance | 44.81 | 59.84 | 67.47 | 69.58 | 76.83 |

All variants completed; original JSON/JSONL and derived `*-summary.json` files are intentionally versioned. Population min/median/max and peak equal the listed final value in each variant. Timing below is concurrent batch timing, not isolated performance.

| Scenario | Survived | Final / peak | Speciation worlds | Extinction worlds | CV median | Diversity median | Seconds/world median | TPS median | Fail / collapse | Classification |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| Baseline extended | 250/250 | 200 | 32/250 | 4/250 | 0 | 59.84 | 32.70 | 3092.55 | 0/0 | Rich 250 |
| capacity-1000 | 8/8 | 1000 | 0/8 | 0/8 | 0 | 38.19 | 82.78 | 276.07 | 0/0 | Healthy 8 |
| dense-resource-space | 8/8 | 2000 | 1/8 | 0/8 | 0 | 33.58 | 208.75 | 105.44 | 0/0 | Healthy 6, Rich 2 |
| divergent-2x | 8/8 | 200 | 3/8 | 0/8 | 0 | 62.51 | 22.85 | 2439.24 | 0/0 | Rich 8 |
| warm-habitat | 8/8 | 200 | 0/8 | 0/8 | 0 | 36.45 | 6.53 | 4239.06 | 0/0 | Healthy 8 |

**Keep the current product default**: seed 42, size 512, 50 founders, capacity 200, mutation multiplier 100, 20°C, food regeneration 12. The 8-seed, shorter variants do not provide strong long-run evidence for replacement; dense variants also change several factors. More species alone is not the objective. Capacity-1000 confirms the 200 population plateau is the configured cap. No defect justifies a simulation version or golden change.

Showcase seed 11 uses the same default configuration except seed. Its verified first speciation is tick 48,600, Photovorus minor-2, founding generation 60, 16 founders, genome distance 118. The genuine saved example at tick 107,504 has 200 organisms and two living species. `benchmarks/showcase-replay-v4.json` verifies restored hash `9942ea1d1033f6db883e6830aa5489bf457db646eb3493a8139ffa54f28f659a` and 1000-tick continuation hash `113628ad459de576452240cff339879b3a4c96bb5257696e7d1234c46b75280d`. No events or mutations are scripted.


