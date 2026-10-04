import fs from 'node:fs'
import path from 'node:path'
import { createHash } from 'node:crypto'
import { fileURLToPath } from 'node:url'
import { gunzipSync } from 'node:zlib'

// Lossless compressed evidence keeps full diagnostic ledgers reviewable in Git.
export function readCalibration(path) {
  const raw = fs.readFileSync(path)
  const text = (String(path).endsWith('.gz') ? gunzipSync(raw) : raw).toString('utf8')
  return JSON.parse(text.replace(/^\uFEFF/, ''))
}

// Load one world at a time; the caller retains control over peak memory.
export function readTemporal(result, sourcePath) {
  const ecology = result.phase2?.viability?.ecology
  if (ecology?.r4_temporal) {
    if (ecology.r4_temporal_ref) throw Error('Ambiguous temporal evidence')
    return ecology.r4_temporal
  }
  const ref = ecology?.r4_temporal_ref
  if (!ref) return undefined
  if (!sourcePath || ref.version !== 1 || ref.seed !== result.seed || ref.tick !== result.ticks)
    throw Error('Invalid temporal reference identity')
  if (typeof ref.path !== 'string' || !ref.path.endsWith('.json.gz') || path.isAbsolute(ref.path)
      || /(^|[\\/])\.\.([\\/]|$)/.test(ref.path) || ref.path.includes(':'))
    throw Error('Invalid temporal reference path')
  const root = fs.realpathSync(path.dirname(sourcePath instanceof URL ? fileURLToPath(sourcePath) : sourcePath))
  const target = fs.realpathSync(path.resolve(root, ref.path))
  const relative = path.relative(root, target)
  if (relative.startsWith('..') || path.isAbsolute(relative)) throw Error('Temporal reference escapes evidence directory')
  const raw = gunzipSync(fs.readFileSync(target))
  if (raw.length !== ref.uncompressed_bytes || createHash('sha256').update(raw).digest('hex') !== ref.sha256)
    throw Error('Temporal reference integrity mismatch')
  const temporal = JSON.parse(raw.toString('utf8'))
  if (temporal.version !== 1) throw Error('Unsupported temporal evidence version')
  return temporal
}
