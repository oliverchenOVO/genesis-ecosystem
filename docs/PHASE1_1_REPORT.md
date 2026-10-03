# Phase 1.1 finalization report

Status: LOCAL GATES PASS; private GitHub CI and release pending verification. No Phase 2 work.

## Delivered scope

Native Windows .genesis Open/Save, Save As, persistent Recent Worlds (5 paths, deduplication, unavailable/remove), session-only dirty markers, paused Save/Discard/Cancel protection for New/Load/Close. Failed/cancelled file operations preserve the world; autosaves do not clear manual dirty state. Application preferences remain outside authoritative world/hash. Small UI fixes prevent evolution pan text selection and wrap file controls at800px.

Calibration runner, deterministic observation/classification, bounded matrix, genuine seed11 showcase, legal density benchmark with headless peak-working-set measurement, and CLI saved-world/replay/continuation validation are included. No LLM enters the simulation.

## Local validation — 2026-10-03

- Rust: **42 PASS** = 26 core + 4 golden + 7 app + 3 calibration + 2 native IPC/capability. Original32 retained, 10 added. No ignored/failing tests.
- Frontend: **21 PASS**, original8 plus13 file-workflow tests.
- `cargo fmt --all -- --check`, workspace clippy/all-targets/locked/-Dwarnings: PASS.
- pnpm formatting, lint, typecheck, tests, production build: PASS.
- Tauri production executable and NSIS installer rebuilt after close capability fix: PASS.
- Production native UAT: PASS, actual controls/dialogs, file cancellation, recent/restart/continuation, dirty guards, organism/species/ancestry/evolution/history, native invalid/checksum/version errors,800px layout. Detailed evidence and limits: PHASE1_1_UAT.md.
- P0:0 observed; unresolved critical P1:0. P1 found during final UAT: guarded close missing destroy capability; minimal permission fix and native retest completed.

## Determinism

Simulation **v4**, RNG **v1**, save format **v1**, original golden fixtures unchanged. `crates/sim-core` has no diff from original Phase1 baseline. Golden/checkpoint tests PASS. Saved showcase107,504 and native saves20,018/20,501 each verify full replay plus1000-tick independent RNG continuation.10 seeds×2000 stress verifies save/load, replay and parallel-world independence: zero failures. **0 replay mismatches**.

Full hashes/RNG/commands/telemetry are recorded in benchmarks/showcase-replay-v4.json, native-save-replay-v4.json, native-restart-replay-v4.json and phase1_1-replay-stress.json. CLI verification intentionally does not modify source saves.

## Ecology evidence and default decision

Preserved **250 seeds×100,000 ticks**:25,000,000 ticks,250/250 survivors,0 failures,0 collapse,32 worlds natural speciation (**12.8%**),35 new species,4 worlds species extinction. Classification: Sterile0,Healthy0,Rich250,Collapse0,Explosion0,Oscillating0.218 Rich worlds have no new species: classification uses persistent lineage diversity as documented. Post-warmup populationCV0 in all worlds is a known hard-cap saturation limitation, despite continual births/deaths.

Completed bounded matrix: capacity1000(8×20k), dense space/resources(8×20k), divergent2x(8×50k), warm28°C(8×20k); all survivors and0 failures/collapse. Numeric distributions/interpretation in ECOLOGY_CALIBRATION.md and machine-readable summaries. No expensive calibration rerun: no authoritative code changed.

**Default unchanged**: seed42,size512,founders50,capacity200,mutation100,20°C,regen12. Short8-seed variants do not justify replacing long-run validated defaults or maximizing species count. Genuine showcase seed11 uses otherwise identical config; first speciation deterministically48,600, founders16,distance118,gen60. No fake history or forced mutations.

## Isolated production benchmark

Windows11, Intel i9-12900H(14 cores/20 threads), Rust1.99.0/MSVC17.14.41, release optimization. No simulation/compiler/native-UAT jobs active during isolated measurements; unrelated user processes/thermal state not controlled. Reference seed11,size512,50 founders/cap200,mutation100,20°C/regen12,no environment commands,50,000 ticks.

| Run | Seconds | Ticks/sec | Final / peak population | Final species |
|---|---:|---:|---:|---:|
| Original Phase1 | 17.1935019 | 2908.075 | 200 / 200 | 2 |
| Phase1.1 isolated1 | 6.2812028 | 7960.259 | 200 / 200 | 2 |
| Phase1.1 isolated2 | 6.3170487 | 7915.089 | 200 / 200 | 2 |
| Phase1.1 isolated3 | 6.2866046 | 7953.419 | 200 / 200 | 2 |

All hashes identical `021373fb7e78ac883b9f216cfb0319007b99f6e732fa77005567a42b9fb64017`; each9962130 organism updates, mean population199.2426,6073 births/5923 deaths/4854 mutations/14 predations. Current repeat spread<1%. Material improvement investigated: authoritative core unchanged and identical workload/hashes, headless timings do not include file UX. Prior background/thermal conditions were not fully recorded; timing is environment-sensitive, **not evidence of a code optimization**. No >15% regression; exact speedup cause cannot be isolated retrospectively. Peak species/memory unavailable in reference format.

## Isolated legal density benchmark

Seed42,size1024,mutation100,20°C,regen100,one worker,2000 ticks. Config and environment command use public validation; no injected organisms. PeakWorkingSet from Windows process sampled250ms; headless process only, excludes WebView/UI and is not instantaneous allocation accounting.

| Starting / cap | Final / peak | Seconds | Ticks/sec | Organism updates/sec | Peak working set bytes | Final / peak species | Failure / extinction |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1000 / 2000 | 2000 | 3.3675459 | 593.904 | 1150811 | 6045696 | 1 / 1 | 0 / 0 |
| 2500 / 5000 | 5000 | 15.8323524 | 126.324 | 611750 | 9834496 | 1 / 1 | 0 / 0 |
| 5000 / 5000 | 5000 | 16.1505833 | 123.835 | 617338 | 8126464 | 1 / 1 | 0 / 0 |

All validate throughout.2k ticks characterize density/performance, not long-run speciation/stability. Earlier high-density files without isolated in filename were concurrent preliminary runs, preserved for provenance and excluded from final timing comparisons. Matrix concurrent timing also must not be compared to isolated runs.

## Windows artifacts

- Portable: `F:\CodeX開發小東東\AI 生態箱：從單細胞一路演化\target\release\genesis-desktop.exe`
- SHA256: `16FB27A061D6F817BDC68FE26897A6C69E1814FDB2110AD23C0A623ABE3C56C9`
- Installer: `F:\CodeX開發小東東\AI 生態箱：從單細胞一路演化\target\release\bundle\nsis\GENESIS_0.1.0_x64-setup.exe`
- SHA256: `EF09F5B0CEDACD1C5B62DF12776A4FF747E398A574E808054F51765821C115F4`

NSIS build successful; production executable launched and native picker works. Interactive installation into the user's system was not performed. Windows x64/WebView2 required; binaries unsigned. Private baseline v0.1.0 is Phase1/1.1 only.

## Git and remote gates

Original8 Phase1 commits preserved. Current finalization will be split into application/native UX, measurement/evidence, and validation documentation commits. No generated target/dist/node_modules/manual test saves or secrets intended for tracking. Remote was absent; authenticated account oliverchenOVO permits private repository creation. Repository, actual remote CI, validated tag and artifact release remain pending until verified; this report must not imply green from YAML alone.

## Limitations and deferred work

Population cap saturation limits net-population volatility. Rich is an analytical category, not a species count. Short parameter variants and2k-tick density cases cannot establish long-run default superiority. Benchmarks depend on hardware/thermal state; headless memory excludes desktop overhead. Native guard cross-product/rare OS permissions/dead-selected inspector not exhaustively retested; automated guard/failure/rotation tests provide coverage. No known critical unresolved issue.

Phase2 remains deferred: multicellular evolution and any later product systems are outside this baseline. Do not advance before all acceptance gates, including actual CI/release where supported. Next recommendation: review this validated baseline and define the next Phase explicitly.
