import fs from 'node:fs'
import { createHash } from 'node:crypto'
import { pathToFileURL } from 'node:url'
import { readCalibration } from './read-calibration.mjs'

const ratio = (n, d) => d > 0 ? n / d : null
const count = n => {
  if (!Number.isSafeInteger(n) || n < 0) throw Error('Invalid diagnostic counter')
  return n
}
const roles = ['SoftSpecialist', 'HardSpecialist', 'PreySpecialist', 'Mixed', 'NoIncome']
const totalsKeys = ['food_energy', 'prey_energy', 'actual_metabolic_energy', 'attack_energy',
  'reproductive_energy', 'organism_ticks', 'offspring_produced']

function fitnessRate(row) {
  const b = row.totals
  if (!roles.includes(row.role) || b.energy.length !== 3 || b.stock.length !== 2 || b.efficiency.length !== 2)
    throw Error('Invalid fitness group')
  for (const n of [...b.energy, ...b.stock, ...b.efficiency, b.samples, b.organism_ticks,
    b.actual_metabolic_energy, b.attack_energy, b.reproductive_energy, b.offspring]) count(n)
  const income = count(b.energy.reduce((a, n) => a + n, 0))
  const net = income - b.actual_metabolic_energy - b.attack_energy
  return { organism_ticks: b.organism_ticks, income_per_tick: ratio(income, b.organism_ticks),
    net_before_reproduction_per_tick: ratio(net, b.organism_ticks),
    net_after_reproduction_per_tick: ratio(net - b.reproductive_energy, b.organism_ticks),
    offspring_per_tick: ratio(b.offspring, b.organism_ticks),
    mean_local_stock: b.stock.map(n => ratio(n, b.samples)),
    mean_processing_efficiency: b.efficiency.map(n => ratio(n, b.samples)) }
}

export function predatorPeriod(rows, start, end) {
  const totals = Object.fromEntries(totalsKeys.map(k => [k, 0]))
  const seen = new Set()
  for (const row of rows) {
    count(row.window); count(row.lineage)
    const key = `${row.window}:${row.lineage}`
    if (seen.has(key)) throw Error('Duplicate predator lineage/window')
    seen.add(key)
    for (const k of totalsKeys) count(row.budgets[k])
    if (row.window * 1000 >= start && row.window * 1000 < end)
      for (const k of totalsKeys) totals[k] = count(totals[k] + row.budgets[k])
  }
  const net = totals.food_energy + totals.prey_energy - totals.actual_metabolic_energy - totals.attack_energy
  return { start, end, totals, net_before_reproduction: net,
    net_after_reproduction: net - totals.reproductive_energy,
    net_before_reproduction_per_tick: ratio(net, totals.organism_ticks),
    prey_income_fraction: ratio(totals.prey_energy, totals.food_energy + totals.prey_energy),
    offspring_per_tick: ratio(totals.offspring_produced, totals.organism_ticks) }
}

export function windowProfitability(summary) {
  if (summary.version !== 1 || summary.analysis_version !== 5 || summary.ticks_per_seed < 5000
    || summary.ticks_per_seed % 1000 || summary.worlds.length !== summary.seeds)
    throw Error('Incomplete diagnostic summary')
  const seeds = new Set()
  return summary.worlds.map(world => {
    count(world.seed)
    if (seeds.has(world.seed)) throw Error('Duplicate diagnostic seed')
    seeds.add(world.seed)
    const groups = new Map()
    for (const r of world.individual_fitness) {
      count(r.window); count(r.lineage)
      if (r.window * 1000 >= summary.ticks_per_seed) throw Error('Out-of-range fitness window')
      const key = `${r.window}:${r.lineage}`
      if (!groups.has(key)) groups.set(key, new Map())
      const group = groups.get(key)
      if (group.has(r.role)) throw Error('Duplicate fitness lineage/window/role')
      group.set(r.role, fitnessRate(r))
    }
    const comparisons = []
    for (const [key, group] of groups) {
      const [window, lineage] = key.split(':').map(Number)
      if (window < 5 || !group.get('Mixed')?.organism_ticks) continue
      for (const role of roles.slice(0, 3)) {
        const specialist = group.get(role), mixed = group.get('Mixed')
        if (!specialist?.organism_ticks) continue
        comparisons.push({ window, lineage, role, specialist, mixed,
          net_advantage_per_tick: specialist.net_before_reproduction_per_tick - mixed.net_before_reproduction_per_tick,
          offspring_advantage_per_tick: specialist.offspring_per_tick - mixed.offspring_per_tick })
      }
    }
    if (world.predators.some(r => r.window * 1000 >= summary.ticks_per_seed))
      throw Error('Out-of-range predator window')
    return { seed: world.seed, world_hash: world.world_hash,
      predator_warmup: predatorPeriod(world.predators, 0, 5000),
      predator_post_warmup: predatorPeriod(world.predators, 5000, summary.ticks_per_seed),
      comparisons,
      matched_roles: roles.slice(0, 3).map(role => {
        const rows = comparisons.filter(r => r.role === role)
        return { role, comparisons: rows.length,
          specialist_net_higher: rows.filter(r => r.net_advantage_per_tick > 0).length,
          specialist_offspring_higher: rows.filter(r => r.offspring_advantage_per_tick > 0).length,
          mean_net_advantage_per_tick: ratio(rows.reduce((n, r) => n + r.net_advantage_per_tick, 0), rows.length) }
      }) }
  })
}

export function profitabilityReview(paths) {
  return { version: 1, authority_changed: false,
    definitions: 'Reuses sealed temporal summaries; no simulation execution. Same seed/lineage/disjoint1000-tick window, post5000 warmup, realized individual fitness groups with nonzero organism-ticks. All observed groups included; these are not >=8-endpoint qualified lineage roles or acceptance gates. Rates use each group organism-ticks; matched means are unweighted across comparisons. Net subtracts actual total metabolism and attack costs; movement allocation is unavailable. Reproductive debit shown separately; birth/death stock transfers are not ecological income. Stocks/efficiencies use observed samples. Warmup and post-warmup predator budgets are separate. Missing groups/zero denominators remain absent/null. Descriptive diet conditioning cannot establish causal survival, generalist dominance, or a movement intervention.',
    scenarios: paths.map(source => {
      const raw = fs.readFileSync(source), summary = readCalibration(source)
      return { source, source_sha256: createHash('sha256').update(raw).digest('hex'),
        rules_revision: summary.rules_revision, seeds: summary.seeds, ticks_per_seed: summary.ticks_per_seed,
        worlds: windowProfitability(summary) }
    }) }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href)
  console.log(JSON.stringify(profitabilityReview(process.argv.slice(2)), null, 2))
