import fs from 'node:fs'
import { pathToFileURL } from 'node:url'
import { readCalibration, readTemporal } from './read-calibration.mjs'
import { resourceCoherence, correlation } from './resource-coherence.mjs'
const ratio=(n,d)=>d>0?n/d:null
const roles=['SoftSpecialist','HardSpecialist','PreySpecialist','Mixed']
export function dietRole(energy) {
  if(energy.length!==3||energy.some(n=>!Number.isSafeInteger(n)||n<0))throw Error('Invalid realized energy')
  const total=energy.reduce((a,b)=>a+b,0)
  if(!total)return 'NoIncome'
  if(energy[0]*10>=total*7)return roles[0]
  if(energy[1]*10>=total*7)return roles[1]
  if(energy[2]*2>=total)return roles[2]
  return roles[3]
}
export function rolePersistence(rows) {
  const ids=[...new Set(rows.map(r=>r.lineage))].sort((a,b)=>a-b)
  return ids.map(lineage=> {
    const selected=rows.filter(r=>r.lineage===lineage).sort((a,b)=>a.end-b.end)
    const longest=Object.fromEntries(roles.map(r=>[r,0])),transitions={}
    let previous=null,streak=0,switches=0
    for(const r of selected) {
      if(!r.qualified||!roles.includes(r.role)){previous=null;streak=0;continue}
      const contiguous=previous&&r.start===previous.end
      if(contiguous) {
        const key=`${previous.role}->${r.role}`;transitions[key]=(transitions[key]??0)+1
        switches+=Number(previous.role!==r.role)
      }
      streak=contiguous&&r.role===previous.role?streak+1:1
      longest[r.role]=Math.max(longest[r.role],streak);previous=r
    }
    return {lineage,longest_consecutive_windows:longest,role_switch_count:switches,transitions}
  })
}
function sumBudgets(budgets) {
  const total={}
  for(const b of budgets)for(const [k,v] of Object.entries(b))total[k]=(total[k]??0)+v
  return total
}
function windows(t,width,ticks) {
  const result=[]
  for(let start=0;start+width<=ticks;start+=width) {
    const samples=Array.from({length:width/100},(_,i)=>t.samples[start+(i+1)*100])
    if(samples.some(s=>!s))throw Error('Missing temporal sample')
    const ids=new Set(samples.flatMap(s=>Object.keys(s)))
    for(let w=start/1000;w<(start+width)/1000;w++)for(const id of Object.keys(t.lineage_budgets_1000[w]??{}))ids.add(id)
    for(const id of ids) {
      const rows=samples.map(s=>s[id]),energy=[0,0,0],totals={}
      const populations=rows.map(r=>r?.population??0)
      for(const r of rows)if(r) {
        r.energy.forEach((v,i)=>energy[i]+=v)
        for(const k of ['population','complexity','mass','bite_capacity','carnivory','x','y','x2','y2','visible_prey','nearby_mates','compatible_mates'])totals[k]=(totals[k]??0)+r[k]
      }
      const budgets=Array.from({length:width/1000},(_,i)=>t.lineage_budgets_1000[start/1000+i]?.[id]??{})
      const b=sumBudgets(budgets),population=totals.population??0,mean=key=>ratio(totals[key]??0,population)
      const source=energy.reduce((a,b)=>a+b,0)
      const initial=t.samples[start]?.[id],final=t.samples[start+width]?.[id]
      const popClosure=(initial?.population??0)+(b.births??0)-(b.deaths??0)-(final?.population??0)
      const energyClosure=(initial?.energy_stock??0)+(b.offspring_initial_energy??0)+(b.food_energy??0)+(b.prey_energy??0)-(b.actual_metabolic_energy??0)-(b.attack_energy??0)-(b.reproductive_energy??0)-(b.removed_death_energy??0)-(final?.energy_stock??0)
      if(popClosure!==0||energyClosure!==0)throw Error(`Temporal closure mismatch lineage${id} tick${start}`)
      if(energy[0]+energy[1]!== (b.food_energy??0)||energy[2]!== (b.prey_energy??0))throw Error('Temporal diet/budget mismatch')
      const sums=key=>rows.reduce((a,r)=>a.map((v,i)=>v+(r?.[key]?.[i]??0)),Array(key==='habitat'?8:key==='mouth'?3:2).fill(0))
      const fields=t.resource_snapshots[0]
      const production=[0,0]
      for(const r of rows)if(r)for(const [cell,n] of Object.entries(r.cells)) {
        production[0]+=fields.soft_productivity[cell]*n
        production[1]+=fields.hard_productivity[cell]*n
      }
      result.push({lineage:Number(id),start,end:start+width,qualified:populations.every(n=>n>=8),after_warmup:start>=5000,
        role:dietRole(energy),energy,fractions:energy.map(n=>ratio(n,source)),population_min:Math.min(...populations),
        mean_living_population:population/(width/100),final_population:final?.population??0,
        mean_complexity:mean('complexity'),mean_body_mass:mean('mass'),mean_bite_capacity:mean('bite_capacity'),mean_carnivory:mean('carnivory'),
        mean_position:[mean('x'),mean('y')],spatial_variance:population?[Math.max(0,mean('x2')-mean('x')**2),Math.max(0,mean('y2')-mean('y')**2)]:null,
        habitat:sums('habitat'),mouth:sums('mouth'),mean_local_stocks:sums('local_stock').map(v=>ratio(v,population)),
        mean_processing_efficiency:sums('processing_efficiency').map(v=>ratio(v,population)),budgets:b,
        mean_uncapped_productivity_numerators:production.map(v=>ratio(v,population)),
        mean_visible_prey:mean('visible_prey'),mean_nearby_eligible_mates:mean('nearby_mates'),mean_compatible_mates:mean('compatible_mates'),
        offspring_per_living_organism:ratio(b.offspring_produced??0,population/(width/100)),
        offspring_per_organism_tick:ratio(b.offspring_produced??0,b.organism_ticks??0),
        successful_matings_per_attempt:ratio(b.successful_matings??0,b.mating_attempts??0),
        net_before_reproduction:(b.food_energy??0)+(b.prey_energy??0)-(b.actual_metabolic_energy??0)-(b.attack_energy??0),
        mean_completed_lifespan:ratio(b.completed_lifespan_sum??0,b.deaths??0),
        completed_offspring_per_death:ratio(b.completed_offspring_sum??0,b.deaths??0)})
    }
  }
  return result
}
export function summarizeTemporal(run, {sourcePath} = {}) {
  if(run.failures!==0||run.results.length!==run.seeds||new Set(run.results.map(r=>r.seed)).size!==run.seeds)throw Error('Incomplete temporal run')
  const worlds=run.results.map(r=> {
    const e=r.phase2?.viability?.ecology,t=readTemporal(r,sourcePath)
    if(!t||t.version!==1||t.sample_interval!==100||t.budget_interval!==1000||!Array.isArray(t.individual_fitness_1000)||!Array.isArray(t.mate_search_density)||r.ticks!==run.ticks_per_seed||!r.phase2.save_load_replay_verified||r.phase2.rng_continuation_ticks!==1000||r.ticks%1000)throw Error('Unverified temporal result')
    const expected=Array.from({length:r.ticks/100+1},(_,i)=>String(i*100))
    if(!Array.isArray(t.resource_snapshots)||t.resource_snapshots.length!==r.ticks/1000+1||t.resource_snapshots.some((s,i)=>s.tick!==i*1000))throw Error('Incomplete resource snapshots')
    if(JSON.stringify(Object.keys(t.samples))!==JSON.stringify(expected))throw Error('Temporal sample grid mismatch')
    const energy=[0,0,0]
    for(const sample of Object.values(t.samples))for(const s of Object.values(sample)) {
      s.energy.forEach((v,i)=>energy[i]+=v)
      if(s.habitat.reduce((a,b)=>a+b,0)!==s.population||s.mouth.reduce((a,b)=>a+b,0)!==s.population||Object.values(s.cells).reduce((a,b)=>a+b,0)!==s.population)throw Error('Temporal occupancy mismatch')
    }
    const observed=e.feeding.reduce((a,f)=>a.map((v,i)=>v+f.totals.channels[i]),[0,0,0])
    if(JSON.stringify(energy)!==JSON.stringify(observed))throw Error('Temporal whole-run energy mismatch')
    const density=t.mate_search_density
    for(const [key,budget] of [['searches','initiating_mate_searches'],['unpaired_candidates','nearby_unpaired_candidates'],['eligible_candidates','nearby_eligible_candidates'],['compatible_candidates','nearby_compatible_candidates']]) {
      if(density.reduce((n,r)=>n+r[key],0)!==Object.values(e.lineage_budgets).reduce((n,b)=>n+b[budget],0))throw Error('Density-controlled mating ledger mismatch')
    }
    if(density.reduce((n,r)=>n+r.successful_pairs,0)!==e.mating.pairs)throw Error('Density-controlled successful pair mismatch')
    const byWidth=[1000,5000].map(width=> {
      const rows=windows(t,width,r.ticks),fitness=roles.map(role=> {
        const selected=rows.filter(r=>r.after_warmup&&r.qualified&&r.role===role)
        return {role,windows:selected.length,lineages:[...new Set(selected.map(r=>r.lineage))],
          // Retain individual world/lineage/window rows; pooled rates are supplemental.
          budgets:sumBudgets(selected.map(r=>r.budgets))}
      })
      const habitat_diet_correlation=[...new Set(rows.map(r=>r.lineage))].map(lineage=> {
        const selected=rows.filter(r=>r.lineage===lineage&&r.after_warmup&&r.qualified&&roles.includes(r.role))
        return {lineage,windows:selected.length,soft_stock_vs_soft_fraction:correlation(selected.map(r=>r.mean_local_stocks[0]),selected.map(r=>r.fractions[0])),hard_stock_vs_hard_fraction:correlation(selected.map(r=>r.mean_local_stocks[1]),selected.map(r=>r.fractions[1])),visible_prey_vs_prey_fraction:correlation(selected.map(r=>r.mean_visible_prey),selected.map(r=>r.fractions[2])),soft_productivity_vs_soft_fraction:correlation(selected.map(r=>r.mean_uncapped_productivity_numerators[0]),selected.map(r=>r.fractions[0])),hard_productivity_vs_hard_fraction:correlation(selected.map(r=>r.mean_uncapped_productivity_numerators[1]),selected.map(r=>r.fractions[1]))}
      })
      return {width,rows,fitness,habitat_diet_correlation,persistence:rolePersistence(rows.filter(r=>r.after_warmup))}
    })
    const predators=Object.entries(t.predator_lineage_budgets_1000).flatMap(([window,rows])=>Object.entries(rows).map(([lineage,b])=>({window:Number(window),lineage:Number(lineage),budgets:b,
      net_before_reproduction:b.food_energy+b.prey_energy-b.actual_metabolic_energy-b.attack_energy,
      net_after_reproduction:b.food_energy+b.prey_energy-b.actual_metabolic_energy-b.attack_energy-b.reproductive_energy,
      offspring_per_organism_tick:ratio(b.offspring_produced,b.organism_ticks)})))
    const individual_fitness=t.individual_fitness_1000.map(r=> {
      const b=r.totals
      return {...r,offspring_per_organism_tick:ratio(b.offspring,b.organism_ticks),successful_matings_per_attempt:ratio(b.successful_matings,b.mating_attempts),
        starvation_per_organism_tick:ratio(b.starvation_deaths,b.organism_ticks),
        completed_lifespan:ratio(b.completed_lifespan_sum,b.deaths),completed_offspring:ratio(b.completed_offspring_sum,b.deaths),
        net_before_reproduction:b.energy.reduce((a,b)=>a+b,0)-b.actual_metabolic_energy-b.attack_energy,
        mean_processing_efficiency:b.efficiency.map(n=>ratio(n,b.samples)),mean_local_stock:b.stock.map(n=>ratio(n,b.samples))}
    })
    for(const [window,lineages] of Object.entries(t.lineage_budgets_1000))for(const [lineage,b] of Object.entries(lineages)) {
      const selected=individual_fitness.filter(r=>r.window===Number(window)&&r.lineage===Number(lineage))
      for(const [budget,fitness] of [['organism_ticks','organism_ticks'],['offspring_produced','offspring'],['successful_matings','successful_matings'],['deaths','deaths'],['actual_metabolic_energy','actual_metabolic_energy'],['reproductive_energy','reproductive_energy'],['attack_energy','attack_energy']]) {
        if(selected.reduce((n,r)=>n+r.totals[fitness],0)!==b[budget])throw Error('Individual fitness ledger mismatch')
      }
      const channels=selected.reduce((a,r)=>a.map((v,i)=>v+r.totals.energy[i]),[0,0,0])
      if(channels[0]+channels[1]!==b.food_energy||channels[2]!==b.prey_energy)throw Error('Individual diet ledger mismatch')
    }
    return {seed:r.seed,world_hash:r.world_hash,windows:byWidth,predators,individual_fitness,mate_search_density:density,local_density_samples:t.local_density_samples,resource_coherence:resourceCoherence(t.resource_snapshots)}
  })
  return {version:1,rules_revision:run.rules_revision,analysis_version:run.analysis_version,seeds:run.seeds,ticks_per_seed:run.ticks_per_seed,worlds,
    definitions:'Diagnostic roles: soft>=70%, hard>=70%, prey>=50%, otherwise Mixed; zero credited income NoIncome. Qualification requires>=8 at every100-tick endpoint of full disjoint windows. Role gaps and missing windows break streaks. Warmup excludes first5000ticks. All original biological gates unchanged. Window fitness tracks completed deaths separately from censored living organisms. Snapshot eligibility opportunities are not actual matching attempts. Spatial masks include all positive cells tied at quantile cutoff; uniform fields have null autocorrelation. Correlation length is first axial16-unit lag with Pearson<=exp(-1), capped at128 and reported censored if not crossed. No movement path length inference.'}
}
export function writeTemporalSummary(run, sourcePath, outputPath) {
  if(run.failures!==0||run.results.length!==run.seeds||new Set(run.results.map(r=>r.seed)).size!==run.seeds)
    throw Error('Incomplete temporal run')
  // Exclusive output and sequential worlds avoid one multi-gigabyte JSON string.
  const fd=fs.openSync(outputPath,'wx')
  try {
    const {worlds, ...metadata}=summarizeTemporal({...run,seeds:0,results:[]},{sourcePath})
    metadata.seeds=run.seeds
    fs.writeSync(fd,JSON.stringify(metadata).slice(0,-1)+',"worlds":[')
    for(let i=0;i<run.results.length;i++) {
      const result=summarizeTemporal({...run,seeds:1,results:[run.results[i]]},{sourcePath})
      fs.writeSync(fd,(i?',':'')+JSON.stringify(result.worlds[0]))
    }
    fs.writeSync(fd,']}\n')
    fs.fsyncSync(fd)
  } finally { fs.closeSync(fd) }
}
if(process.argv[1]&&import.meta.url===pathToFileURL(process.argv[1]).href) {
  const run=readCalibration(process.argv[2])
  if(process.argv[3])writeTemporalSummary(run,process.argv[2],process.argv[3])
  else console.log(JSON.stringify(summarizeTemporal(run,{sourcePath:process.argv[2]}),null,2))
}
