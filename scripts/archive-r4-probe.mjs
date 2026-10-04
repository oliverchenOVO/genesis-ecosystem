import fs from 'node:fs'
import {isDeepStrictEqual} from 'node:util'
import {createHash} from 'node:crypto'
import {gzipSync,gunzipSync} from 'node:zlib'
import {readCalibration} from '../scripts/read-calibration.mjs'
import {evaluateCandidate} from '../scripts/evaluate-ecology-candidates.mjs'
import {writeTemporalSummary} from '../scripts/summarize-temporal-ecology.mjs'
const name=process.argv[2],prefix=`phase2-r4-${name}`,base=`artifacts/${prefix}`
const receipt=JSON.parse(fs.readFileSync(`${base}-exit.json`,'utf8').replace(/^\uFEFF/,''))
if(receipt.exit_code!==0)throw Error('Probe did not finish successfully')
const run=readCalibration(`${base}.json`),control=readCalibration('benchmarks/phase2-r3-candidate-d.json')
const lines=fs.readFileSync(`${base}.jsonl`,'utf8').trim().split('\n')
if(lines.length!==8||run.results.length!==8||run.failures!==0)throw Error('Incomplete probe')
for(const line of lines) {const r=JSON.parse(line);if(!isDeepStrictEqual(r,run.results.find(w=>w.seed===r.seed)))throw Error('Journal differs')}
const metrics=evaluateCandidate(run,control)
fs.writeFileSync(`benchmarks/${prefix}-summary.json`,JSON.stringify(metrics,null,2)+'\n',{flag:'wx'})
writeTemporalSummary(run,`${base}.json`,`${base}-temporal-summary.json`)
const temporal=readCalibration(`${base}-temporal-summary.json`)
const overview=temporal.worlds.map(w=>({seed:w.seed,
  diagnostic_roles:w.windows.map(x=>({width:x.width,persistence:x.persistence.map(p=>({lineage:p.lineage,longest_consecutive_windows:p.longest_consecutive_windows,role_switch_count:p.role_switch_count}))})),
  initial_productivity:w.resource_coherence[0].fields.filter(f=>f.channel.endsWith('productivity')).map(f=>({channel:f.channel,fraction:f.fraction,correlation_length:f.correlation_length,correlation_length_censored:f.correlation_length_censored,components:f.components.length,areas:f.components.map(c=>c.area).sort((a,b)=>a-b)})),
  stock_persistence:['soft','hard'].map(channel=>({channel,mask_jaccard:w.resource_coherence.filter(s=>s.tick>=5000).map(s=>s.fields.find(f=>f.channel===channel&&f.fraction===0.25).persistence?.mask_jaccard)})),
  predator_net_before_reproduction:w.predators.reduce((n,r)=>n+r.net_before_reproduction,0),
  predator_offspring:w.predators.reduce((n,r)=>n+r.budgets.offspring_produced,0)}))
fs.writeFileSync(`benchmarks/${prefix}-review.json`,JSON.stringify({worlds:overview,definitions:'Temporal roles and resource masks are diagnostic, not replacements for original gates. Areas in world units squared; stock persistence top25% sampled1000 ticks. Predator budget includes all windows; fitness is not causal.'},null,2)+'\n',{flag:'wx'})
const hash=b=>createHash('sha256').update(b).digest('hex'),files=[]
for(const suffix of ['.json','.jsonl','-temporal-summary.json']) {
 const raw=fs.readFileSync(base+suffix),gzip=gzipSync(raw,{level:9})
 if(hash(gunzipSync(gzip))!==hash(raw))throw Error('Compression mismatch')
 const destination=`benchmarks/${prefix}${suffix}.gz`;fs.writeFileSync(destination,gzip,{flag:'wx'})
 files.push({destination,raw_bytes:raw.length,gzip_bytes:gzip.length,raw_sha256:hash(raw),gzip_sha256:hash(gzip)})
}
const source=fs.readFileSync(`${base}-source.patch`),binary=fs.readFileSync(`${base}-probe.exe`)
if(hash(binary)!==receipt.executable_sha256.toLowerCase())throw Error('Binary mismatch')
fs.copyFileSync(`${base}-source.patch`,`benchmarks/${prefix}-source.patch`,fs.constants.COPYFILE_EXCL)
const provenance={version:1,stage:'Isolated R4 exploratory candidate, not selected authority or100-seed formal acceptance',source_base:'321ab99f3d4c83a2bb1a365907f03cf29429e6c8',source_patch_sha256:hash(source),executable_sha256:hash(binary),elapsed_seconds:run.elapsed_seconds,workers:run.workers,seeds:8,ticks_per_seed:50000,jsonl_equals_final:true,full_replay_save_load_continuations:8,normal_performance_claim:false,files,world_hashes:run.results.map(r=>({seed:r.seed,hash:r.world_hash}))}
fs.writeFileSync(`benchmarks/${prefix}-provenance.json`,JSON.stringify(provenance,null,2)+'\n',{flag:'wx'})
console.log(JSON.stringify({name,metrics:{MC:metrics.persistent_multicellular_worlds,highC:metrics.persistent_high_C_worlds,morph:metrics.multiple_persistent_morphology_worlds,niche:metrics.multiple_persistent_niche_worlds,feeding:metrics.multiple_persistent_realized_feeding_worlds,prey:metrics.persistent_prey_income_worlds,rejected:metrics.rejection_reasons},elapsed:run.elapsed_seconds,files:files.map(f=>({path:f.destination,bytes:f.gzip_bytes}))}))
