import fs from 'node:fs'
import { pathToFileURL } from 'node:url'
import { summarizeEcology } from './summarize-ecological-diagnostics.mjs'
import { summarizeViability } from './summarize-viability.mjs'

const mean=v=>v.reduce((a,b)=>a+b,0)/v.length
export function evaluateCandidate(run, control) {
  if (JSON.stringify(run.base_config)!==JSON.stringify(control.base_config) || run.seeds!==control.seeds || run.ticks_per_seed!==control.ticks_per_seed) throw Error('Unpaired candidate configuration')
  const seeds=r=>r.results.map(w=>w.seed).sort((a,b)=>a-b)
  if (JSON.stringify(seeds(run))!==JSON.stringify(seeds(control))) throw Error('Unpaired seeds')
  const {seed:baseSeed,...baseBody}=run.base_config
  for (const w of run.results) {
    const {seed,...body}=w.config
    if (seed!==w.seed || JSON.stringify(body)!==JSON.stringify(baseBody) || JSON.stringify(w.initial_environment)!==JSON.stringify(control.results.find(c=>c.seed===w.seed).initial_environment)) throw Error('Unpaired per-world configuration or environment')
  }
  const ecology=summarizeEcology(run)
  summarizeViability(run)
  const worlds=run.results
  const count=f=>worlds.filter(f).length
  const multicellular=count(w=>w.phase2.persistent_multicellular)
  const highC=count(w=>w.phase2.persistent_high_complexity_lineages>0)
  const morphology=count(w=>w.phase2.maximum_persistent_morphology_clusters>1)
  const niches=count(w=>w.phase2.maximum_persistent_niche_clusters>1)
  const feeding=count(w=>w.phase2.realized_trophic_persistence.maximum_simultaneous_persistent_roles>1)
  const prey=count(w=>w.phase2.realized_trophic_persistence.longest_prey_consumer_sampled_ticks>=1000)
  const collapse=count(w=>w.final_population===0)
  const rejection=[]
  if (control.results.some(w=>w.phase2.persistent_multicellular) && !multicellular) rejection.push('Lost all paired persistent multicellular worlds')
  if (control.results.some(w=>w.phase2.persistent_high_complexity_lineages>0) && !highC) rejection.push('Lost all paired persistent high-C worlds')
  const resourceWindows=worlds.map(w=>Object.entries(w.phase2.viability.ecology.windows_5000_ticks ?? {}).filter(([key])=>Number(key)>=1).map(([,window])=>[0,1].map(channel=>window.feeding_energy_by_mouth.reduce((n,m)=>n+m[channel],0))))
  const finalChannelUsage=resourceWindows.map(w=>w.at(-1) ?? null)
  if (run.results[0].phase2.viability.ecology.channel_names && [0,1].some(channel=>resourceWindows.every(w=>w.length>0&&w.every(row=>row[channel]===0)))) rejection.push('A resource channel is unused throughout every observed post-warmup window')
  const capWorlds=count(w=>w.phase2.safety_ceiling_ticks>0)
  return {scenario:run.scenario,simulation_version:run.simulation_version,rules_revision:run.rules_revision,analysis_version:run.analysis_version,seeds:run.seeds,ticks_per_seed:run.ticks_per_seed,technical_failures:run.failures,verified_save_load_replay_continuations:worlds.length,survival:worlds.length-collapse,natural_collapse:collapse,collapse_classifications:count(w=>w.classification==='Collapse'),safety_ceiling_worlds:capWorlds,maximum_safety_ceiling_fraction:Math.max(...worlds.map(w=>w.phase2.safety_ceiling_ticks/w.ticks)),persistent_multicellular_worlds:multicellular,persistent_high_C_worlds:highC,persistent_high_C_lineages:worlds.reduce((n,w)=>n+w.phase2.persistent_high_complexity_lineages,0),multiple_persistent_morphology_worlds:morphology,multiple_persistent_niche_worlds:niches,multiple_persistent_realized_feeding_worlds:feeding,persistent_prey_income_worlds:prey,speciation_worlds:count(w=>w.species_formed>0),new_species:worlds.reduce((n,w)=>n+w.species_formed,0),extinction_worlds:count(w=>w.extinct_species>0),extinct_species:worlds.reduce((n,w)=>n+w.extinct_species,0),mean_population_CV:mean(worlds.map(w=>w.population_coefficient_of_variation)),mean_resource_CV:mean(worlds.map(w=>w.phase2.resource_coefficient_of_variation)),cross_lineage:ecology.cross_lineage_fraction,cross_habitat:ecology.cross_habitat_fraction,mating_distance_by_seed:ecology.worlds.map(w=>({seed:w.seed,...w.mating_distance})),actual_mouth_diets:ecology.mouths,final_environmental_channel_income:finalChannelUsage,rejection_reasons:rejection,requires_cap_dominance_review:capWorlds>0,required_complexity_and_differentiation_nonzero:multicellular>0&&highC>0&&morphology>0&&niches>0&&feeding>0&&prey>0,definitions:'Paired configurations/seeds and unchanged analytical gates. No new acceptance threshold. Cap fractions reported for independent persistence review; natural collapse and isolated cap contacts are not automatically classified as technical or biological gate failures. Rejection flags only exact total loss of previously nonzero complex cohorts or persistently unused resource channel, not a high incidence quota. Natural collapse is reported separately. Prey persistence uses unchanged observed-income >=50%, members >=8, post-warmup 1000 sampled ticks. Resource windows follow diagnostic schema; compare habitat definitions only when compatible.'}
}
if (process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href) {
  const control=JSON.parse(fs.readFileSync(process.argv[2],'utf8'))
  const candidates=process.argv.slice(4).map(path=>({path,...evaluateCandidate(JSON.parse(fs.readFileSync(path,'utf8')),control)}))
  fs.writeFileSync(process.argv[3],JSON.stringify({control:process.argv[2],candidates},null,2)+'\n')
}
