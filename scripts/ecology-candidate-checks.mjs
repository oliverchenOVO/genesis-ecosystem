import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import { evaluateCandidate } from './evaluate-ecology-candidates.mjs'

// Exercise evidence validation against a complete, immutable measured R2 run.
const control = JSON.parse(fs.readFileSync(new URL('../benchmarks/phase2-r3-diagnostics-r2-v3.json', import.meta.url), 'utf8'))

test('paired evaluation rejects altered founder conditions and environments', () => {
  const candidate = structuredClone(control)
  candidate.base_config.mutation_rate++
  assert.throws(() => evaluateCandidate(candidate, control), /Unpaired candidate configuration/)
  candidate.base_config = structuredClone(control.base_config)
  candidate.results[0].initial_environment.regeneration++
  assert.throws(() => evaluateCandidate(candidate, control), /Unpaired per-world/)
})

test('paired evaluation requires continuation and recomputes energy closure', () => {
  const candidate = structuredClone(control)
  candidate.results[0].phase2.rng_continuation_ticks = 999
  assert.throws(() => evaluateCandidate(candidate, control), /Unverified ecological result/)
  candidate.results[0].phase2.rng_continuation_ticks = 1000
  candidate.results[0].phase2.viability.cohorts[0].totals.initial_energy++
  assert.throws(() => evaluateCandidate(candidate, control), /False closure claim/)
})

test('candidate rejection detects complete loss of viable complex cohorts', () => {
  const candidate = structuredClone(control)
  for (const w of candidate.results) {
    w.phase2.persistent_multicellular = false
    w.phase2.persistent_high_complexity_lineages = 0
  }
  const result = evaluateCandidate(candidate, control)
  assert.equal(result.rejection_reasons.length, 2)
  assert.equal(result.required_complexity_and_differentiation_nonzero, false)
})
