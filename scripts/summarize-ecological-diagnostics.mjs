import fs from 'node:fs'
import { pathToFileURL } from 'node:url'
import { readCalibration } from './read-calibration.mjs'

const ratio = (n, d) => d > 0 ? n / d : null
const distribution = values => {
  const v = values.filter(x => x !== null).sort((a,b) => a-b)
  return v.length ? {min:v[0],median:v[Math.floor((v.length-1)/2)],max:v.at(-1),mean:v.reduce((a,b)=>a+b,0)/v.length} : null
}
export function summarizeEcology(run) {
  if (run.failures !== 0 || run.results.length !== run.seeds || new Set(run.results.map(r=>r.seed)).size !== run.seeds) throw Error('Incomplete ecological run')
  const mouths = Array.from({length:3},(_,mouth)=>({mouth,food_events:0,prey_events:0,food_energy:0,prey_energy:0,hardness:0,productivity:0,starvation_deaths:0,offspring:0,organism_ticks:0,channels:[0,0,0]}))
  const worlds = run.results.map(r => {
    if (r.ticks !== run.ticks_per_seed || !r.phase2.save_load_replay_verified || r.phase2.rng_continuation_ticks !== 1000 || ![1,2,3,4].includes(r.phase2.viability.ecology.version)) throw Error('Unverified ecological result')
    const e = r.phase2.viability.ecology, m = e.mating
    if (m.lineage_edges.reduce((n,v)=>n+v.count,0) !== 2*m.pairs || m.child_edges.reduce((n,v)=>n+v.count,0) !== 2*r.births) throw Error('Mating event ledger mismatch')
    if (e.version >= 2) {
      const windows=Object.values(e.windows_5000_ticks)
      if (windows.reduce((n,w)=>n+w.mating_pairs,0) !== m.pairs || windows.reduce((n,w)=>n+w.cross_lineage,0) !== m.cross_lineage || windows.reduce((n,w)=>n+w.cross_habitat,0) !== m.cross_habitat) throw Error('Temporal mating ledger mismatch')
      for (const w of windows) {
        if (Object.values(w.gene_flow).flatMap(row=>Object.values(row)).reduce((a,b)=>a+b,0) !== w.mating_pairs*2) throw Error('Temporal gene-flow mismatch')
      }
      const windowEnergy=windows.reduce((n,w)=>n+w.feeding_energy_by_mouth.flat().reduce((a,b)=>a+b,0),0)
      if (windowEnergy !== e.feeding.reduce((n,v)=>n+v.totals.food_energy+v.totals.prey_energy,0)) throw Error('Temporal energy ledger mismatch')
    }
    const observedFood = e.feeding.reduce((n,v)=>n+v.totals.food_energy,0)
    const observedPrey = e.feeding.reduce((n,v)=>n+v.totals.prey_energy,0)
    if (observedFood !== r.phase2.viability.cohorts.slice(0,4).reduce((n,v)=>n+v.totals.food_energy,0) || observedPrey !== r.phase2.viability.cohorts.slice(0,4).reduce((n,v)=>n+v.totals.prey_energy,0)) throw Error('Feeding event ledger mismatch')
    for (const f of e.feeding) {
      const out=mouths[f.mouth], t=f.totals
      for (const k of ['food_events','prey_events','food_energy','prey_energy','hardness','productivity']) out[k]+=t[k]
      t.channels.forEach((v,i)=>out.channels[i]+=v)
    }
    for (const [mouth,b] of Object.entries(e.mouth_budgets)) {
      mouths[mouth].starvation_deaths+=b.starvation_deaths
      mouths[mouth].offspring+=b.offspring_produced
      mouths[mouth].organism_ticks+=b.organism_ticks
    }
    const overlap = Object.fromEntries(['spatial','habitat','feeding_hardness','feeding_productivity','prey_size','realized_energy_sources'].map(k=>[k,distribution(e.overlap.map(v=>v[k]))]))
    return {seed:r.seed,world_hash:r.world_hash,final_population:r.final_population,successful_mating_pairs:m.pairs,cross_lineage_fraction:ratio(m.cross_lineage,m.pairs),cross_habitat_fraction:ratio(m.cross_habitat,m.pairs),cross_species_fraction:ratio(m.cross_species,m.pairs),mating_distance:m.distance,mean_genetic_distance:ratio(m.genetic_distance_sum,m.pairs),overlap,major_persistent_lineages:e.major_persistent_lineages,gene_flow_matrix:e.gene_flow_matrix,dispersal:e.lineages.filter(v=>e.major_persistent_lineages.includes(v.id)).map(v=>({lineage:v.id,dispersal:v.dispersal,sampled_habitat_switches:v.totals.habitat_switches,birth_habitat_mismatch_fraction:ratio(v.totals.parent_habitat_distance,v.totals.samples)})),persistent_multicellular:r.phase2.persistent_multicellular,persistent_high_complexity_lineages:r.phase2.persistent_high_complexity_lineages,morphology_clusters:r.phase2.maximum_persistent_morphology_clusters,niche_clusters:r.phase2.maximum_persistent_niche_clusters,realized_feeding_roles:r.phase2.realized_trophic_persistence?.maximum_simultaneous_persistent_roles}
  })
  return {simulation_version:run.simulation_version,rules_revision:run.rules_revision,ecological_diagnostic_versions:[...new Set(run.results.map(r=>r.phase2.viability.ecology.version))],seeds:run.seeds,ticks_per_seed:run.ticks_per_seed,failures:run.failures,mouths:mouths.map(m=>({...m,mean_food_hardness:ratio(m.hardness,m.food_events),mean_productivity:ratio(m.productivity,m.food_events+m.prey_events),mean_food_energy:ratio(m.food_energy,m.food_events),mean_prey_energy:ratio(m.prey_energy,m.prey_events),prey_energy_fraction:ratio(m.prey_energy,m.food_energy+m.prey_energy),starvation_deaths_per_organism_tick:ratio(m.starvation_deaths,m.organism_ticks),offspring_per_organism_tick:ratio(m.offspring,m.organism_ticks)})),cross_lineage_fraction:distribution(worlds.map(w=>w.cross_lineage_fraction)),cross_habitat_fraction:distribution(worlds.map(w=>w.cross_habitat_fraction)),worlds}
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const result=summarizeEcology(readCalibration(process.argv[2]))
  if (process.argv[3]) fs.writeFileSync(process.argv[3],JSON.stringify(result,null,2)+'\n')
  else process.stdout.write(JSON.stringify(result,null,2)+'\n')
}
