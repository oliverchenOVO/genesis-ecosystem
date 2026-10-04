import test from 'node:test'
import assert from 'node:assert/strict'
import { dietRole,rolePersistence,summarizeTemporal,writeTemporalSummary } from './summarize-temporal-ecology.mjs'
import { fieldPatches,patchPersistence,correlation } from './resource-coherence.mjs'
import { readCalibration,readTemporal } from './read-calibration.mjs'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { gzipSync } from 'node:zlib'
import { createHash } from 'node:crypto'
const fixture=()=>readCalibration(new URL('../fixtures/ecology-r4-observer-smoke.json.gz',import.meta.url))
function withSidecar(check) {
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'genesis-evidence-'))
  const run=fixture(),r=run.results[0],e=r.phase2.viability.ecology
  const original=structuredClone(run),raw=Buffer.from(JSON.stringify(e.r4_temporal))
  const file=path.join(root,'world.json.gz'),source=path.join(root,'run.json')
  fs.writeFileSync(file,gzipSync(raw))
  delete e.r4_temporal
  e.r4_temporal_ref={version:1,path:'world.json.gz',seed:r.seed,tick:r.ticks,
    sha256:createHash('sha256').update(raw).digest('hex'),uncompressed_bytes:raw.length}
  try { check({run,r,e,original,raw,file,source,root}) }
  finally {
    for(const name of fs.readdirSync(root))fs.unlinkSync(path.join(root,name))
    fs.rmdirSync(root)
  }
}
test('compressed per-world evidence and streamed summaries preserve all ledgers',()=>withSidecar(({run,original,source,root})=> {
  assert.deepEqual(summarizeTemporal(run,{sourcePath:source}),summarizeTemporal(original))
  const output=path.join(root,'summary.json')
  writeTemporalSummary(run,source,output)
  assert.deepEqual(JSON.parse(fs.readFileSync(output,'utf8')),summarizeTemporal(original))
  const before=fs.readFileSync(output)
  assert.throws(()=>writeTemporalSummary(run,source,output),/EEXIST/)
  assert.deepEqual(fs.readFileSync(output),before)
}))
test('temporal references reject corruption, swapped worlds and escaping paths',()=>withSidecar(({r,e,raw,file,source})=> {
  const ref=e.r4_temporal_ref
  assert.throws(()=>readTemporal(r),/identity/)
  ref.seed++
  assert.throws(()=>readTemporal(r,source),/identity/)
  ref.seed--
  ref.path='../world.json.gz'
  assert.throws(()=>readTemporal(r,source),/path/)
  ref.path='world.json.gz'
  ref.uncompressed_bytes++
  assert.throws(()=>readTemporal(r,source),/integrity/)
  ref.uncompressed_bytes--
  const altered=Buffer.from(raw);altered[20]^=1
  fs.writeFileSync(file,gzipSync(altered))
  assert.throws(()=>readTemporal(r,source),/integrity/)
  fs.writeFileSync(file,Buffer.from('invalid gzip'))
  assert.throws(()=>readTemporal(r,source))
}))
test('realized roles honor exact energy boundaries and zero income',()=> {
  assert.equal(dietRole([70,30,0]),'SoftSpecialist')
  assert.equal(dietRole([30,70,0]),'HardSpecialist')
  assert.equal(dietRole([25,25,50]),'PreySpecialist')
  assert.equal(dietRole([69,31,0]),'Mixed')
  assert.equal(dietRole([0,0,0]),'NoIncome')
})
test('role persistence breaks at extinct, low population and missing windows',()=> {
  const rows=[['SoftSpecialist',true],['SoftSpecialist',true],['Mixed',true],['HardSpecialist',true],['HardSpecialist',false],['HardSpecialist',true]]
    .map(([role,qualified],i)=>({lineage:1,start:i*1000,end:(i+1)*1000,role,qualified}))
  rows.push({lineage:1,start:8000,end:9000,role:'HardSpecialist',qualified:true})
  const [s]=rolePersistence(rows)
  assert.equal(s.longest_consecutive_windows.SoftSpecialist,2)
  assert.equal(s.longest_consecutive_windows.HardSpecialist,1)
  assert.equal(s.role_switch_count,2)
  assert.equal(s.transitions['SoftSpecialist->SoftSpecialist'],1)
})
test('four-neighbour components distinguish contiguous fields and checkerboard',()=> {
  const contiguous=fieldPatches([1,1,0,0,1,1,0,0,0,0,0,0,0,0,0,0],4,0.25)
  const isolated=fieldPatches([1,0,1,0,0,0,0,0,1,0,1,0,0,0,0,0],4,0.25)
  assert.equal(contiguous.components.length,1)
  assert.equal(contiguous.components[0].area,1024)
  assert.equal(contiguous.components[0].perimeter,128)
  assert.equal(isolated.components.length,4)
  assert.equal(contiguous.nearest_same_rich_mean,16)
  assert.equal(isolated.nearest_same_rich_mean,32)
})
test('empty and tied uniform fields do not fabricate spatial variation',()=> {
  const empty=fieldPatches(Array(16).fill(0),4,0.25)
  const full=fieldPatches(Array(16).fill(1),4,0.25)
  assert.equal(empty.rich_cells,0)
  assert.equal(full.rich_cells,16)
  assert.equal(full.correlation_length,null)
  assert.equal(full.correlation_length_censored,false)
  assert.equal(correlation([1,1],[1,2]),null)
  assert.throws(()=>fieldPatches([1],2,0.25),/geometry/)
})
test('patch persistence reports disappearance rather than invented identities',()=> {
  const a=fieldPatches([1,1,0,0,0,0,0,0,0],3,0.25)
  const b=fieldPatches([0,0,0,0,0,0,0,1,1],3,0.25)
  const same=patchPersistence(a,a),moved=patchPersistence(a,b)
  assert.equal(same.mask_jaccard,1)
  assert.equal(same.matches[0].area_ratio,1)
  assert.equal(moved.mask_jaccard,0)
  assert.equal(moved.matches[0].current,null)
})
test('temporal analysis rejects partial and unverified evidence',()=> {
  assert.throws(()=>summarizeTemporal({failures:0,seeds:1,results:[]}),/Incomplete/)
  assert.throws(()=>summarizeTemporal({failures:0,seeds:1,ticks_per_seed:1000,results:[{seed:0,ticks:1000,phase2:{}}]}),/Unverified/)
})
test('actual R3 observer fixture closes independent temporal and individual ledgers',()=> {
  const run=fixture(),summary=summarizeTemporal(run)
  assert.equal(summary.worlds[0].world_hash,run.results[0].world_hash)
  assert.equal(summary.worlds[0].windows[0].width,1000)
  assert.ok(summary.worlds[0].windows[0].rows.length>0)
  assert.equal(summary.worlds[0].resource_coherence.length,2)
  assert.ok(summary.worlds[0].individual_fitness.length>0)
})
test('income, boundary stock and occupancy corruption cannot pass temporal checks',()=> {
  for(const mutate of [
    t=>t.samples[100][1].energy[0]++,
    t=>t.samples[1000][1].energy_stock++,
    t=>t.samples[100][1].habitat[0]++,
    t=>delete t.samples[200],
  ]) {
    const run=fixture();mutate(run.results[0].phase2.viability.ecology.r4_temporal)
    assert.throws(()=>summarizeTemporal(run),/mismatch/)
  }
})
test('individual fitness and density-controlled matings require actual event conservation',()=> {
  for(const mutate of [
    t=>t.individual_fitness_1000[0].totals.offspring++,
    t=>t.mate_search_density[0].eligible_candidates++,
    t=>t.mate_search_density[0].successful_pairs++,
  ]) {
    const run=fixture();mutate(run.results[0].phase2.viability.ecology.r4_temporal)
    assert.throws(()=>summarizeTemporal(run),/mismatch/)
  }
})
test('resource snapshots must cover the entire measured run',()=> {
  const run=fixture();run.results[0].phase2.viability.ecology.r4_temporal.resource_snapshots.pop()
  assert.throws(()=>summarizeTemporal(run),/Incomplete resource/)
})
