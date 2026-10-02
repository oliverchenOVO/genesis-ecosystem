# GENESIS — Phase 1 architecture

Status: implementation in progress; no Phase 2 work authorized.

Rust owns all world truth. React/TypeScript displays read-only snapshots and send explicit environmental commands through Tauri 2. A dedicated simulation worker owns the world; rendering frequency is independent of tick count. The headless core has no desktop, browser, network, or LLM dependency.

Small related simulation subsystems live in `sim-core` modules rather than premature cross-crate abstractions. `sim-benchmark` runs the exact same core. Persistence and replay remain independently testable modules. sim-app owns the simulation worker, action protocol and rotating autosaves; sim-server exposes that worker over loopback for browser development. src-tauri is the production Windows desktop member. Its packaged webview uses the action IPC handler and never starts the development server. The starter UI habitat has 50 founders and capacity 200, based on full 100-seed/50,000-tick validation; both controls remain configurable up to 5000. Higher-density runs can collapse and are retained in benchmark evidence.

Only environment commands affect the world. No interface edits genomes or grants traits. Scientific names, history, and charts derive from recorded simulation state. This is artificial life, not a predictive scientific model.

