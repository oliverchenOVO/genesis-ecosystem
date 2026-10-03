import { readFile, writeFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';

export function distribution(values) {
  if (!values.length) return null;
  if (values.some(v => !Number.isFinite(v))) throw new Error('Nonfinite measurement');
  const ordered = [...values].sort((a,b) => a-b);
  const q = p => ordered[Math.round((ordered.length-1)*p)];
  return {min:ordered[0],median:q(.5),p90:q(.9),p95:q(.95),max:ordered.at(-1),mean:ordered.reduce((a,b)=>a+b,0)/ordered.length};
}
export function summarizePhase2(report) {
  const rows = report.results;
  if (report.simulation_version !== 5 || report.analysis_version !== 3) throw new Error('Expected simulation5 / analysis3');
  if (report.seeds < 100 || report.ticks_per_seed < 100000 || rows.length !== report.seeds || new Set(rows.map(r=>r.seed)).size !== report.seeds) throw new Error('Incomplete or duplicate long-run seed set');
  const valid = rows.filter(r=>!r.error);
  if (valid.some(r=>r.ticks !== report.ticks_per_seed)) throw new Error('Mismatched tick duration');
  const count = predicate => valid.filter(predicate).length;
  const dist = selector => distribution(valid.map(selector));
  const failures = rows.length-valid.length;
  const verified = count(r=>r.phase2.save_load_replay_verified && r.phase2.rng_continuation_ticks===1000);
  return {
    simulation_version:5, rng_version:1, save_format_version:1, analysis_version:3,
    scenario:report.scenario, initial_environment:valid[0]?.initial_environment, config:report.base_config, seeds:report.seeds, ticks_per_seed:report.ticks_per_seed,
    elapsed_seconds:report.elapsed_seconds,technical_failures:failures,save_load_replay_continuation_verified:verified,
    stability_pass:failures===0 && verified===rows.length,
    survival:count(r=>r.final_population>0),natural_world_extinctions:count(r=>r.final_population===0),
    collapse_classification:count(r=>r.classification==='Collapse'),explosion:count(r=>r.classification==='Explosion'),
    classifications: Object.fromEntries(['Collapse','Explosion','Oscillating','Rich','Sterile','Healthy'].map(name=>[name,count(r=>r.classification===name)])),
    speciation_worlds:count(r=>r.species_formed>0),species_formed:valid.reduce((a,r)=>a+r.species_formed,0),
    extinction_worlds:count(r=>r.extinct_species>0),extinct_species:valid.reduce((a,r)=>a+r.extinct_species,0),
    persistent_multicellular_worlds:count(r=>r.phase2.persistent_multicellular),
    longest_multicellular_sampled_ticks:dist(r=>r.phase2.longest_multicellular_sampled_ticks),
    maximum_sampled_complexity:dist(r=>r.phase2.maximum_sampled_complexity),
    median_species_complexity:dist(r=>r.phase2.phase2_metrics.median_species_complexity),
    persistent_high_complexity_lineages:dist(r=>r.phase2.persistent_high_complexity_lineages),
    maximum_persistent_niche_clusters:dist(r=>r.phase2.maximum_persistent_niche_clusters),
    morphology_mean_distance:dist(r=>r.phase2.morphology_distance_distribution.mean),
    final_morphology_max_distance:dist(r=>r.phase2.phase2_metrics.morphology_max_distance),
    morphology_cluster_mean:dist(r=>r.phase2.morphology_cluster_distribution.mean),
    final_morphology_variance:Array.from({length:6},(_,i)=>dist(r=>r.phase2.phase2_metrics.morphology_variance[i])),
    niche_mean_distance:dist(r=>r.phase2.niche_distance_distribution.mean),
    final_niche_max_distance:dist(r=>r.phase2.phase2_metrics.niche_max_distance),
    coexistence_ticks:dist(r=>r.phase2.coexistence_ticks),
    predator_prey_ratio:dist(r=>r.phase2.predator_like_population_distribution.mean/Math.max(1,r.phase2.consumer_population_distribution.mean)),
    predator_starvation_deaths:valid.reduce((a,r)=>a+r.phase2.predator_starvation_deaths,0),
    predations:valid.reduce((a,r)=>a+r.predations,0),
    population_cv:dist(r=>r.population_coefficient_of_variation),resource_cv:dist(r=>r.phase2.resource_coefficient_of_variation),
    final_population:dist(r=>r.final_population),
    birth_death_turnover_per_organism_tick:dist(r=>(r.births+r.deaths)/(Math.max(1,r.population_samples.mean)*r.ticks)),
    species_turnover_per_tick:dist(r=>(r.species_formed+r.extinct_species)/r.ticks),
    safety_ceiling_worlds:count(r=>r.phase2.safety_ceiling_ticks>0),
    safety_ceiling_fraction:dist(r=>r.phase2.safety_ceiling_ticks/r.ticks),
    innovation_species_totals:Array.from({length:5},(_,i)=>valid.reduce((a,r)=>a+r.phase2.innovation_species_counts[i],0)),
    innovation_worlds:Array.from({length:5},(_,i)=>count(r=>r.phase2.innovation_species_counts[i]>0)),
    definitions:'Distributions are across completed worlds. Pair distances include sampled single-species zero values. Multicellular persistence: >=8 and >=25% living multi-unit organisms for1000 consecutive sampled ticks. High complexity: lineage >=8 members, mean C>=200 for1000 ticks. Niche persistence: same representative ID and drift<150 for1000 ticks. Analysis is read-only; see PHASE2_SIMULATION.md.'
  };
}
if (process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href) {
  const [input,output] = process.argv.slice(2);
  if (!input || !output) throw new Error('Usage: node scripts/summarize-phase2.mjs INPUT OUTPUT');
  const report=JSON.parse((await readFile(input,'utf8')).replace(/^\uFEFF/,''));
  const summary=summarizePhase2(report);
  await writeFile(output,JSON.stringify(summary,null,2)+'\n');
  console.log(JSON.stringify(summary,null,2));
}
