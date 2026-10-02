# GENESIS — Phase 1 architecture

Status: implementation in progress; no Phase 2 work authorized.

Rust owns all world truth. React/TypeScript will display read-only snapshots and send explicit environmental commands through Tauri 2. A dedicated simulation worker owns the world; rendering frequency is independent of tick count. The headless core has no desktop, browser, network, or LLM dependency.

Small related simulation subsystems live in `sim-core` modules rather than premature cross-crate abstractions. `sim-benchmark` runs the exact same core. Persistence and replay remain independently testable modules. Desktop is a separate workspace member once prerequisites are available.

Only environment commands affect the world. No interface edits genomes or grants traits. Scientific names, history, and charts derive from recorded simulation state. This is artificial life, not a predictive scientific model.
