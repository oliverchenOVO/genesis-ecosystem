# GENESIS / AI 生態箱

A deterministic Windows artificial-life ecosystem. Rust owns simulation truth; Tauri 2 and React display its recorded evidence. This is an artificial-life model, not a scientifically predictive simulator.

Current branch implements simulation v5 / Phase2 and is **INCOMPLETE** pending native production UAT and sustained complex-niche evidence. Protected v0.1.0 remains the accepted v4 baseline. See [Phase2 report](docs/PHASE2_REPORT.md) for exact tests,100-seed calibration, benchmark results and review artifacts. v4 saves require v0.1.0 and are explicitly rejected by v5.

## Run

Prerequisites: Rust 1.99.0, Microsoft C++ Build Tools + Windows SDK, WebView2, Node 24, pnpm 11.19.0.

```powershell
pnpm install --frozen-lockfile
pnpm tauri dev
```

For browser development, run `cargo run -p sim-server` and `pnpm dev` in separate terminals, then open http://127.0.0.1:1420. The loopback server is a development/testing adapter to the same Rust application worker; it is not included in the desktop product.

```powershell
cargo test --release --workspace --locked
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
pnpm format:check
pnpm lint
pnpm typecheck
pnpm test
pnpm build
$env:CARGO_TARGET_DIR = 'target/phase2-review'
pnpm tauri build
Remove-Item Env:CARGO_TARGET_DIR
cargo run --release -p sim-benchmark -- --seed 42 --ticks 10000 --population 500
cargo run --release -p sim-benchmark -- --seed 42 --ticks 50000 --population 2000
cargo run --release -p sim-benchmark -- --seed 11 --ticks 50000 --population 50 --limit 200
cargo run --release -p sim-benchmark -- --stress --seeds 100 --ticks 50000 --population 50 --limit 200
```

Windows can run `./scripts/validate.ps1` for the checked validation pipeline, with `-Package` and/or `-FullStress` for the full release gates. Stop the development server before rebuilding its running executable. Outputs go to ignored `artifacts/`.

## Use

Create a world with a seed, size, founders, technical safety ceiling, mutation and initial habitat. The v5 starter uses seed42,50 founders,ceiling2000,20°C/regeneration4, validated headlessly over100 seeds×100000 ticks with no ceiling hits. The genuine seed4 showcase first speciates at35000 ticks; examples/phase2-seed4-tick50000.genesis retains a verified two-species world. This does not establish persistent multicellular or niche-rich evolution. Higher-density worlds can become extinct. Play/pause and speed affect tick execution rate, never tick meaning. Click an organism to inspect inherited morphology, costs, genome and parents; Species, Evolution and History read actual records. Habitat layers show real productivity,temperature and terrain.

Save/Load currently use a manual save slot in the platform application data directory under `local.genesis.ecosystem/worlds`. Loading pauses the world. Save results show the exact path. Three autosave slots rotate every 5000 ticks. Browser development uses `artifacts/saves` instead. Verify replay reconstructs the world from its seed, configuration and command history and compares the full world hash.

No LLM, cloud accounts or multiplayer is implemented. The existing Git remote is private. Keep v0.1.0 binaries in target/release intact; Phase2 production review builds use the separate target/phase2-review directory.

See [architecture](docs/ARCHITECTURE.md), [simulation](docs/SIMULATION.md), [determinism](docs/DETERMINISM.md), [save format](docs/SAVE_FORMAT.md), [replay](docs/REPLAY.md), and [Phase 1 report](docs/PHASE1_REPORT.md) for exact acceptance status and limitations.


