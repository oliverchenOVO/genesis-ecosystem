import fs from 'node:fs'
import { gunzipSync } from 'node:zlib'

// Lossless compressed evidence keeps full diagnostic ledgers reviewable in Git.
export function readCalibration(path) {
  const raw = fs.readFileSync(path)
  const text = (String(path).endsWith('.gz') ? gunzipSync(raw) : raw).toString('utf8')
  return JSON.parse(text.replace(/^\uFEFF/, ''))
}
