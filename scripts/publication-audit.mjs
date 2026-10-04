import { execFileSync } from 'node:child_process';
import { gunzipSync } from 'node:zlib';
import { mkdirSync, writeFileSync } from 'node:fs';

// Read-only preliminary publication inventory, not a substitute for secret scanning.
const git = (...args) => execFileSync('git', args, { maxBuffer: 256 * 1024 * 1024 });
const objects = git('rev-list', '--objects', '--all').toString().trim().split('\n');
const metadata = execFileSync('git', ['cat-file', '--batch-check=%(objectname) %(objecttype) %(objectsize)'], {
  input: objects.map((line) => line.split(' ')[0]).join('\n') + '\n',
  maxBuffer: 8 * 1024 * 1024,
}).toString().trim().split('\n');
const signatures = [
  ['credential-pattern', /(?:gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}|sk-proj-[A-Za-z0-9_-]{30,}|AKIA[A-Z0-9]{16}|-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----)/g],
  ['personal-machine-path', /(?:[A-Z]:[\\/]+Users[\\/]+[^\s"<>]+|[A-Z]:[\\/]+CodeX[^\s"<>]+)/gi],
  ['email-address', /[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}/gi],
];
const findings = [];
const skipped = [];
let scanned = 0;
let compressed = 0;
for (let i = 0; i < metadata.length; i++) {
  const [oid, type, size] = metadata[i].split(' ');
  if (type !== 'blob') continue;
  const path = objects[i].slice(oid.length + 1);
  if (+size > 256 * 1024 * 1024) { skipped.push({ oid, path, reason: 'blob exceeds scan limit' }); continue; }
  let buffer = git('cat-file', 'blob', oid);
  if (buffer[0] === 0x1f && buffer[1] === 0x8b) {
    try { buffer = gunzipSync(buffer, { maxOutputLength: 256 * 1024 * 1024 }); compressed++; }
    catch { skipped.push({ oid, path, reason: 'gzip expansion failed or exceeds limit' }); continue; }
  }
  if (buffer.includes(0)) { skipped.push({ oid, path, reason: 'binary; textual signature scan omitted' }); continue; }
  scanned++;
  const text = buffer.toString('utf8');
  for (const [category, expression] of signatures) {
    expression.lastIndex = 0;
    const count = [...text.matchAll(expression)].length;
    if (count) findings.push({ oid, path, category, count });
  }
}
const report = {
  version: 1,
  scanned_head: git('rev-parse', 'HEAD').toString().trim(),
  scope: 'Unique reachable blobs across local --all refs, including bounded gzip expansion; current uncommitted files and remote-only refs are excluded.',
  limitations: 'Signature inventory only. Unknown token formats, encoded secrets, binary payloads, remote-only/deleted refs, GitHub logs/artifacts and third-party rights are not certified safe.',
  scanned_text_blobs: scanned,
  expanded_gzip_blobs: compressed,
  author_email_count: new Set(git('log', '--all', '--format=%ae').toString().trim().split('\n')).size,
  findings,
  skipped,
};
mkdirSync('artifacts', { recursive: true });
writeFileSync('artifacts/github-publication-audit.json', JSON.stringify(report, null, 2) + '\n');
console.log(JSON.stringify({ scanned, compressed, findings: findings.length, credential_findings: findings.filter((f) => f.category === 'credential-pattern').length, skipped: skipped.length }));
