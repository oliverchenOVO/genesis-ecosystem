# GENESIS / AI 生態箱

A deterministic Windows artificial-life ecosystem. Rust owns simulation truth; Tauri 2 and React display its recorded evidence. This is an artificial-life model, not a scientifically predictive simulator.

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
pnpm tauri build
cargo run --release -p sim-benchmark -- --seed 42 --ticks 10000 --population 500
cargo run --release -p sim-benchmark -- --seed 42 --ticks 50000 --population 2000
cargo run --release -p sim-benchmark -- --seed 11 --ticks 50000 --population 50 --limit 200
cargo run --release -p sim-benchmark -- --stress --seeds 100 --ticks 50000 --population 50 --limit 200
```

Windows can run `./scripts/validate.ps1` for the checked validation pipeline, with `-Package` and/or `-FullStress` for the full release gates. Stop the development server before rebuilding its running executable. Outputs go to ignored `artifacts/`.

## Use

Create a world with a seed, world size, starting population, population capacity and mutation preset. The starter habitat uses 50 founders and capacity 200, validated over 100 seeds and 50,000 ticks. Larger capacities up to 5000 remain available; high-density worlds can become extinct. Play/pause and speed control affect how fast ticks execute, never their meaning. Apply temperature and food regeneration to create selection pressures. Click an organism (or use its accessible selector) to inspect phenotype, genome and parents. Species, Evolution and History read actual registries and telemetry; species may take many generations to form.

Save/Load currently use a manual save slot in the platform application data directory under `local.genesis.ecosystem/worlds`. Loading pauses the world. Save results show the exact path. Three autosave slots rotate every 5000 ticks. Browser development uses `artifacts/saves` instead. Verify replay reconstructs the world from its seed, configuration and command history and compares the full world hash.

No LLM, cloud accounts, multiplayer or Phase 2 ecology is implemented. No Git remote is required for local development; never push without explicit authorization.

See [architecture](docs/ARCHITECTURE.md), [simulation](docs/SIMULATION.md), [determinism](docs/DETERMINISM.md), [save format](docs/SAVE_FORMAT.md), [replay](docs/REPLAY.md), and [Phase 1 report](docs/PHASE1_REPORT.md) for exact acceptance status and limitations.


