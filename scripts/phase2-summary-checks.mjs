import { test } from 'node:test';
import assert from 'node:assert/strict';
import { distribution, summarizePhase2 } from './summarize-phase2.mjs';
import fs from 'node:fs';
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
