import { test } from 'node:test';
import assert from 'node:assert/strict';
import { distribution, summarizePhase2 } from './summarize-phase2.mjs';
import fs from 'node:fs';
import { createHash } from 'node:crypto';
import { summarizeViability } from './summarize-viability.mjs';
test('cannot publish acceptance numbers from partial or duplicate long runs',()=>{
  assert.throws(()=>summarizePhase2({simulation_version:5,analysis_version:3,seeds:100,ticks_per_seed:100000,results:[]}),/Incomplete/);
  assert.throws(()=>summarizePhase2({simulation_version:5,analysis_version:3,seeds:100,ticks_per_seed:100000,results:Array.from({length:100},()=>({seed:1}))}),/duplicate/);
});
test('cross-world distributions preserve zeros and reject nonfinite values',()=>{
  assert.deepEqual(distribution([0,4,2]),{min:0,median:2,p90:4,p95:4,max:4,mean:2});
  assert.equal(distribution([]),null);
  assert.throws(()=>distribution([Infinity]),/Nonfinite/);
});

test('analysis4 cannot reinterpret revision1 or omit persistence evidence',()=>{
  const old=JSON.parse(fs.readFileSync(new URL('../benchmarks/phase2-final-calibration.json',import.meta.url),'utf8').replace(/^\uFEFF/,''));
  assert.equal(summarizePhase2(old).rules_revision,1);
  assert.throws(()=>summarizePhase2({...old,analysis_version:4}),/Missing revision2/);
  assert.throws(()=>summarizePhase2({...old,analysis_version:4,rules_revision:2}),/acceptance measurements/);
});
test('viability summary verifies actual cohort/world energy and population closure',()=>{
  const run=JSON.parse(fs.readFileSync(new URL('../benchmarks/phase2-prefinal-viability-v2.json',import.meta.url),'utf8').replace(/^\uFEFF/,''));
  const summary=summarizeViability(run);
  assert.equal(summary.verification_passes,8);
  assert.equal(summary.cohorts.length,9);
  const falseClosure=structuredClone(run);
  falseClosure.results[0].phase2.viability.cohorts[0].totals.food_energy+=1;
  assert.throws(()=>summarizeViability(falseClosure),/False closure/);
  const falseWorld=structuredClone(run);
  falseWorld.results[0].births+=1;
  assert.throws(()=>summarizeViability(falseWorld),/World\/cohort/);
  assert.throws(()=>summarizeViability({...run,results:run.results.slice(1)}),/Incomplete/);
});

test('genuine revision2 save and independent lineage continuation evidence agree',()=>{
  const read = name => JSON.parse(fs.readFileSync(new URL(`../benchmarks/${name}.json`,import.meta.url),'utf8').replace(/^\uFEFF/,''));
  const verified=read('phase2-r2-showcase-verified');
  const inspector=read('phase2-r2-showcase-lineages');
  const provenance=read('phase2-r2-showcase-provenance');
  const save=fs.readFileSync(new URL(`../${provenance.path}`,import.meta.url));
  assert.equal(createHash('sha256').update(save).digest('hex').toUpperCase(),provenance.sha256);
  assert.equal(verified.verified,true);
  assert.equal(inspector.hash,verified.restored.hash);
  assert.equal(inspector.continuation.hash,verified.continuation_hash);
  assert.equal(inspector.living_lineages.reduce((s,c)=>s+c.population,0),verified.restored.population);
  assert.ok(verified.restored.phase2_metrics.multicellular_population*4>=verified.restored.population);
  const sustained=inspector.living_lineages.filter(c=>c.population>=8&&c.mean_complexity>=200&&c.offspring_already_produced_by_living_members>0&&inspector.continuation.saved_high_complexity_lineage_sampled_streaks[c.lineage_id]>=1000);
  assert.ok(sustained.length>0);
  for(const c of sustained){
    assert.equal(c.living_members_with_real_parents,c.population);
    assert.ok(inspector.continuation.samples.every(s=>s.qualifying_high_complexity_lineages.includes(c.lineage_id)));
  }
});
