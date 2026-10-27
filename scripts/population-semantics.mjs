import fs from 'node:fs'
import { gunzipSync } from 'node:zlib'

// Offline species-gate semantic review over population-semantics observer output.
// Every semantic is judged by the same rule; nothing here feeds back into the simulation.
const SEMANTICS = ['A_lineage', 'B_genetic', 'C_genetic_mouth', 'D_community', 'D_strict_community']
const MIN_MEMBERS = 8
const MIN_BIRTHS = 8
const ISOLATION_PERMILLE = 200
const DIVERGENCE = 100
const WARMUP_WINDOWS = 5
const PERSIST_WINDOWS = 2

const distance = (a, b) => a.reduce((s, v, i) => s + Math.abs(v - b[i]), 0) / a.length | 0
const median = v => { const s = [...v].sort((a, b) => a - b); return s.length ? s[s.length >> 1] : null }
const share = (n, d) => d > 0 ? n / d : null

export function review(run) {
  const worlds = run.worlds.map(w => {
    const p = w.population_semantics
    const births = p.births.filter(b => b.window >= WARMUP_WINDOWS).map(b => b.totals)
    const sum = k => births.reduce((s, b) => s + b[k], 0)
    const graphs = p.lineage_flow_graphs.filter(g => g.window >= WARMUP_WINDOWS).map(g => g.graph)
    const semantics = Object.fromEntries(SEMANTICS.map(name => {
      const windows = p.windows.filter(x => x.window >= WARMUP_WINDOWS)
      const qualified = new Map()
      const sizes = [], lineages = [], isolations = [], counts = []
      let survived = 0, eligible = 0
      windows.forEach((x, idx) => {
        const groups = Object.entries(x.groups[name] ?? {}).filter(([, g]) => g.members >= MIN_MEMBERS)
        counts.push(groups.length)
        const next = windows[idx + 1]?.groups[name] ?? null
        for (const [id, g] of groups) {
          sizes.push(g.members); lineages.push(g.lineages)
          if (next) { eligible++; if ((next[id]?.members ?? 0) >= MIN_MEMBERS) survived++ }
          const total = g.births.within + g.births.cross
          if (total >= MIN_BIRTHS) isolations.push(g.births.cross / total)
          const isolated = total >= MIN_BIRTHS && g.births.cross * 1000 <= total * ISOLATION_PERMILLE
          const divergent = groups.some(([other, h]) => other !== id && distance(g.centroid, h.centroid) >= DIVERGENCE)
          const q = qualified.get(id) ?? { run: 0, best: 0 }
          q.run = isolated && divergent ? q.run + 1 : 0
          q.best = Math.max(q.best, q.run)
          qualified.set(id, q)
        }
        for (const [id, q] of qualified) if (!groups.some(([gid]) => gid === id)) q.run = 0
      })
      const candidates = [...qualified.values()].filter(q => q.best >= PERSIST_WINDOWS).length
      return [name, { candidates, max_concurrent_groups: Math.max(0, ...counts), median_groups_per_window: median(counts),
        median_group_members: median(sizes), median_lineages_per_group: median(lineages),
        median_external_birth_fraction: median(isolations), group_survival_to_next_window: share(survived, eligible) }]
    }))
    return { seed: w.seed, births: sum('births'), cross_lineage: sum('cross_lineage'),
      cross_lineage_same_niche: sum('cross_lineage_same_niche'), cross_lineage_cross_ecotype: sum('cross_lineage_cross_ecotype'),
      cross_ecotype: sum('cross_ecotype'), cross_mouth: sum('cross_mouth'),
      graph: { median_lineages_with_births: median(graphs.map(g => g.lineages_with_births)),
        median_communities: median(graphs.map(g => g.communities)),
        median_strict_communities: median(graphs.map(g => g.strict_communities)),
        within_lineage_birth_fraction: share(graphs.reduce((s, g) => s + g.within_lineage_births, 0), graphs.reduce((s, g) => s + g.births, 0)) },
      semantics }
  })
  const total = k => worlds.reduce((s, w) => s + w[k], 0)
  const summary = Object.fromEntries(SEMANTICS.map(name => {
    const v = worlds.map(w => w.semantics[name])
    return [name, { worlds_with_candidate: v.filter(s => s.candidates > 0).length, total_candidates: v.reduce((s, x) => s + x.candidates, 0),
      max_candidates_in_one_world: Math.max(...v.map(s => s.candidates)),
      median_groups_per_window: median(v.map(s => s.median_groups_per_window)), median_group_members: median(v.map(s => s.median_group_members)),
      median_lineages_per_group: median(v.map(s => s.median_lineages_per_group)), median_external_birth_fraction: median(v.map(s => s.median_external_birth_fraction)),
      median_group_survival_to_next_window: median(v.map(s => s.group_survival_to_next_window)) }]
  }))
  return { version: 1, worlds: worlds.length, post_warmup_from_window: WARMUP_WINDOWS,
    rule: { min_members: MIN_MEMBERS, min_births: MIN_BIRTHS, isolation_permille: ISOLATION_PERMILLE, divergence: DIVERGENCE, persist_windows: PERSIST_WINDOWS },
    births: { total: total('births'), cross_lineage_fraction: share(total('cross_lineage'), total('births')),
      same_niche_share_of_cross_lineage: share(total('cross_lineage_same_niche'), total('cross_lineage')),
      cross_ecotype_share_of_cross_lineage: share(total('cross_lineage_cross_ecotype'), total('cross_lineage')),
      cross_ecotype_fraction: share(total('cross_ecotype'), total('births')), cross_mouth_fraction: share(total('cross_mouth'), total('births')) },
    summary, per_world: worlds,
    definitions: 'Post-warmup windows only. A counterfactual species candidate is a group with >=8 members and >=8 births involving it whose cross-group births are <=20% (unchanged threshold), whose centroid is >=100 genome distance from another concurrent group with >=8 members, in >=2 consecutive 1000-tick windows under the same group id. The same rule is applied to every semantic; no species are created. Same-niche: same mouth and lifetime diet role, morphology distance <150, genome distance <100.' }
}

if (import.meta.url === `file:///${process.argv[1].replace(/\\/g, '/')}` || process.argv[1]?.endsWith('population-semantics.mjs')) {
  const path = process.argv[2]
  const text = path.endsWith('.gz') ? gunzipSync(fs.readFileSync(path)).toString() : fs.readFileSync(path, 'utf8')
  console.log(JSON.stringify(review(JSON.parse(text.replace(/^﻿/, ''))), null, 2))
}
