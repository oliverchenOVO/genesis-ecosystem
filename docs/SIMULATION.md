# Simulation version 1

## Tick order

1. Increment tick and regenerate bounded grid resources using fertility.
2. Build uniform spatial buckets; read all positions to evaluate utility intents.
3. Commit movement from those intents.
4. Resolve limited shared food in ascending organism ID order.
5. Resolve predation, then energy metabolism, environmental damage, starvation and aging.
6. Match surviving mature organisms locally; one mating per individual per tick.
7. Crossover haploid parental alleles, then independent bounded point/additive/rare large mutations.
8. Commit births and deaths, ancestry and population counters.
9. Detect extinct species; every 100 ticks evaluate species candidates and sample telemetry.
10. Record population milestones every 1000 ticks.

Utility order on equal scores: flee, hunt, mate, food, explore, rest. Distance ties prefer lower ID. Shared food and conflicts use explicit stable ID priority, rather than container traversal accidents. Sorting IDs is an invariant.

## Biology

14 haploid loci regulate morphology, movement, sensing, metabolism, resource efficiency, reproduction, offspring tendency, mutation tendency, aggression, social affinity, thermal optimum, thermal tolerance, storage and diet. Phenotypes use coupled integer equations (e.g. larger bodies consume more energy and slow movement). Environment changes thermal energy cost and survival, never mutation direction.

Births require two compatible, mature, nearby, energetic parents. Parents donate one third of energy each; the shared birth budget is divided over offspring. Organisms mature at 60 ticks with an 80-tick mating cooldown; aging depends on a metabolism allele. All parent records remain as compact archival records for provenance. No generation is fabricated from elapsed ticks: each organism stores its actual parental generation.

Resources are a 16-unit grid with maximum 1000 per cell. Predatory dietary alleles (>650) reduce plant-food conversion and enable local hunting. Death causes are recorded as starvation, age, predation, or environment. Population capacity limits births, not survivor counts or telemetry.

## Species heuristic

Lineages are inheritance branches. Children diverging by mean absolute allele distance >=120 from the branch founder join a compatible sibling branch or create one. A branch becomes a species only with >=8 living members, >=100 genetic distance from its species founder, >=8 births, <=20% cross-branch mating, and sustained eligibility for 200 ticks (three 100-tick detection samples). A failed sample resets persistence. This is an explicitly approximate artificial-life classification, not a claim about biological species definitions.

Species retain origin, ancestor, founder genome/population, measured genetic distance, deterministic unique name and extinction tick forever. Telemetry stores actual population, cumulative lifecycle counts, mean traits, distribution bounds and mean distance diversity approximation. No scripted evolution or fake chart data exists.
