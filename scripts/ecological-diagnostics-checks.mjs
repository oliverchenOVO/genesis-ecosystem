import test from 'node:test'
import assert from 'node:assert/strict'
import { summarizeEcology } from './summarize-ecological-diagnostics.mjs'

test('reject partial, duplicate and unverified ecological evidence', () => {
  assert.throws(()=>summarizeEcology({failures:0,seeds:2,results:[]}))
  assert.throws(()=>summarizeEcology({failures:0,seeds:2,results:[{seed:1},{seed:1}]}))
  assert.throws(()=>summarizeEcology({failures:1,seeds:0,results:[]}))
})
test('reject fabricated successful pair and child event totals', () => {
  const result={seed:0,ticks:100,phase2:{save_load_replay_verified:true,viability:{ecology:{version:1,mating:{pairs:1,lineage_edges:[],child_edges:[]}}}}}
  assert.throws(()=>summarizeEcology({seeds:1,ticks_per_seed:100,failures:0,results:[result]}),/ledger mismatch/)
  result.phase2.viability.ecology.mating.lineage_edges=[{count:2}]
  result.births=1
  assert.throws(()=>summarizeEcology({seeds:1,ticks_per_seed:100,failures:0,results:[result]}),/ledger mismatch/)
})
test('realized diet fractions and mean event energy keep distinct units', () => {
  const ecology={version:1,mating:{pairs:1,lineage_edges:[{count:2}],child_edges:[{count:4}],cross_lineage:0,cross_species:0,cross_habitat:0},feeding:[{mouth:2,totals:{food_events:2,prey_events:1,food_energy:400,prey_energy:600,hardness:200,productivity:60,channels:[400,0,600]}}],mouth_budgets:{},overlap:[],lineages:[],major_persistent_lineages:[]}
  const r={seed:0,ticks:100,births:2,phase2:{save_load_replay_verified:true,viability:{ecology,cohorts:[{totals:{food_energy:400,prey_energy:600}},...Array.from({length:3},()=>({totals:{food_energy:0,prey_energy:0}}))]}}}
  const s=summarizeEcology({seeds:1,ticks_per_seed:100,failures:0,results:[r]})
  assert.equal(s.mouths[2].mean_food_energy,200)
  assert.equal(s.mouths[2].mean_prey_energy,600)
  assert.equal(s.mouths[2].prey_energy_fraction,0.6)
})
