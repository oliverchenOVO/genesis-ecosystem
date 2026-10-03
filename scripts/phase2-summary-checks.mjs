import { test } from 'node:test';
import assert from 'node:assert/strict';
import { distribution, summarizePhase2 } from './summarize-phase2.mjs';
test('cannot publish acceptance numbers from partial or duplicate long runs',()=>{
  assert.throws(()=>summarizePhase2({simulation_version:5,analysis_version:3,seeds:100,ticks_per_seed:100000,results:[]}),/Incomplete/);
  assert.throws(()=>summarizePhase2({simulation_version:5,analysis_version:3,seeds:100,ticks_per_seed:100000,results:Array.from({length:100},()=>({seed:1}))}),/duplicate/);
});
test('cross-world distributions preserve zeros and reject nonfinite values',()=>{
  assert.deepEqual(distribution([0,4,2]),{min:0,median:2,p90:4,p95:4,max:4,mean:2});
  assert.equal(distribution([]),null);
  assert.throws(()=>distribution([Infinity]),/Nonfinite/);
});
