import fs from 'node:fs'
import { pathToFileURL } from 'node:url'

const labels = ['C<80', '80<=C<140', '140<=C<200', 'C>=200', 'single-unit', 'multi-unit', 'high-complexity', 'Piercer', 'predator-like']
const rate = (n, d) => Number.isFinite(n) && d > 0 ? n / d : null

export function summarizeViability(run) {
  if (run.simulation_version !== 5 || run.failures !== 0 || run.results.length !== run.seeds || new Set(run.results.map(r => r.seed)).size !== run.seeds) throw Error('Incomplete or invalid viability run')
  const totals = labels.map(() => ({}))
  for (const result of run.results) {
    if (result.ticks !== run.ticks_per_seed || !result.phase2.save_load_replay_verified || ![2,3].includes(result.phase2.viability.diagnostic_version)) throw Error('Unverified run or old diagnostic schema')
    const cohorts = result.phase2.viability.cohorts
    if (cohorts.length !== labels.length) throw Error('Missing cohorts')
    cohorts.forEach((row, i) => {
      if (row.cohort !== labels[i] || row.energy_closure_error !== 0 || row.population_closure_error !== 0) throw Error('Cohort accounting mismatch')
      const t = row.totals
      const energyIn = t.initial_energy + t.offspring_initial_energy + t.food_energy + t.prey_energy
      const energyOut = t.actual_metabolic_energy + t.attack_energy + t.reproductive_energy + t.removed_death_energy + t.final_energy
      if (energyIn !== energyOut || t.initial_population + t.births !== t.deaths + t.final_population) throw Error('False closure claim')
      for (const [key, value] of Object.entries(t)) {
        if (!Number.isSafeInteger(value) || value < 0) throw Error('Invalid diagnostic counter')
        totals[i][key] = (totals[i][key] ?? 0) + value
        if (!Number.isSafeInteger(totals[i][key])) throw Error('Unsafe aggregated counter')
      }
    })
    for (const [field, expected] of [['births', result.births], ['deaths', result.deaths], ['final_population', result.final_population]]) {
      if (cohorts.slice(0, 4).reduce((n, c) => n + c.totals[field], 0) !== expected) throw Error('World/cohort counter mismatch')
    }
  }
  return {
    simulation_version: run.simulation_version, analysis_version: run.analysis_version,
    diagnostic_version: run.results[0].phase2.viability.diagnostic_version, scenario: run.scenario, seeds: run.seeds, ticks_per_seed: run.ticks_per_seed,
    config: run.base_config, initial_environment: run.results[0].initial_environment,
    verification_passes: run.results.length, authoritative_hashes: run.results.map(r => ({seed:r.seed,hash:r.world_hash})),
    cohorts: totals.map((t, i) => ({
      cohort:labels[i], totals:t,
      intake_per_organism_tick:rate(t.food_energy+t.prey_energy,t.organism_ticks),
      expenditure_per_organism_tick:rate(t.actual_metabolic_energy+t.attack_energy+t.reproductive_energy,t.organism_ticks),
      basal_requested_per_tick:rate(t.requested_basal_energy,t.organism_ticks),
      morphology_requested_per_tick:rate(t.requested_morphology_energy,t.organism_ticks),
      movement_requested_per_tick:rate(t.requested_movement_energy,t.organism_ticks),
      thermal_requested_per_tick:rate(t.requested_thermal_energy,t.organism_ticks),
      reproduction_per_tick:rate(t.reproductive_energy,t.organism_ticks),
      food_energy_per_attempt:rate(t.food_energy,t.food_attempt_ticks),
      resource_units_per_attempt:rate(t.resource_units,t.food_attempt_ticks),
      available_resource_per_attempt:rate(t.available_resource_sum,t.food_attempt_ticks),
      feeding_quota_per_attempt:rate(t.feeding_quota_sum,t.food_attempt_ticks),
      feeding_efficiency_per_attempt:rate(t.feeding_efficiency_sum,t.food_attempt_ticks),
      successful_feeding_fraction:rate(t.successful_feeding_ticks,t.food_attempt_ticks),
      completed_lifespan:rate(t.completed_lifespan_sum,t.deaths),
      completed_offspring:rate(t.completed_offspring_sum,t.deaths),
      offspring_per_organism_tick:rate(t.offspring_produced,t.organism_ticks),
      effective_speed:rate(t.effective_speed_sum,t.organism_ticks),
      construction_cost:rate(t.construction_cost_sum,t.organism_ticks),
      stored_energy:rate(t.energy_sum,t.organism_ticks), capacity:rate(t.capacity_sum,t.organism_ticks),
      reproduction_ready_fraction:rate(t.reproduction_ready_ticks,t.organism_ticks),
      mating_success_fraction:rate(t.successful_matings,t.mating_attempts),
      eligible_mate_fraction:rate(t.nearby_eligible_candidates,t.nearby_unpaired_candidates),
      compatible_eligible_mate_fraction:rate(t.nearby_compatible_candidates,t.nearby_eligible_candidates),
      mean_distance_compatible_eligible_fraction:rate(t.nearby_mean_distance_compatible_candidates,t.nearby_eligible_candidates),
      original_loci_compatible_fraction:rate(t.nearby_original_loci_compatible_candidates,t.nearby_eligible_candidates),
      structural_loci_compatible_fraction:rate(t.nearby_structural_loci_compatible_candidates,t.nearby_eligible_candidates),
      segmentation_compatible_fraction:rate(t.nearby_segmentation_compatible_candidates,t.nearby_eligible_candidates),
      mouth_compatible_fraction:rate(t.nearby_mouth_compatible_candidates,t.nearby_eligible_candidates),
      carnivory_compatible_fraction:rate(t.nearby_carnivory_compatible_candidates,t.nearby_eligible_candidates),
      starvation_death_fraction:rate(t.starvation_deaths,t.deaths), predation_death_fraction:rate(t.predation_deaths,t.deaths),
      prey_visible_fraction:rate(t.perceptible_prey_ticks,t.organism_ticks),
      attack_success_fraction:rate(t.kills,t.attacks), failed_attacks:t.attacks-t.kills,
      prey_income_per_tick:rate(t.prey_energy,t.organism_ticks), attack_spent_per_tick:rate(t.attack_energy,t.organism_ticks),
    })),
    definitions:'Whole-run pooled exposure, overlapping inherited cohorts; requested costs can exceed clamped actual expenditure; completed lifetimes exclude survivors recorded as censored. Cohort differences are measured associations, not controlled causal effects. Acceptance persistence definitions are unchanged.'
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const [, , source, destination] = process.argv
  if (!source || !destination) throw Error('Usage: node scripts/summarize-viability.mjs INPUT OUTPUT')
  const summary = summarizeViability(JSON.parse(fs.readFileSync(source, 'utf8').replace(/^\uFEFF/, '')))
  fs.writeFileSync(destination, JSON.stringify(summary, null, 2) + '\n')
}
