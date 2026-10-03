# GENESIS architecture

Status: Phase1/1.1 validated baseline preserved at v0.1.0. Phase2 v5 implemented on phase2/multicellular-ecology; acceptance incomplete. PHASE2_SIMULATION.md and PHASE2_REPORT.md describe the authoritative extensions and remaining gates.

Rust owns all world truth. React/TypeScript displays read-only snapshots and send explicit environmental commands through Tauri 2. A dedicated simulation worker owns the world; rendering frequency is independent of tick count. The headless core has no desktop, browser, network, or LLM dependency.

Small related simulation subsystems live in `sim-core` modules rather than premature cross-crate abstractions. `sim-benchmark` runs the exact same core. Persistence and replay remain independently testable modules. sim-app owns the simulation worker, action protocol and rotating autosaves; sim-server exposes that worker over loopback for browser development. src-tauri is the production Windows desktop member. Its packaged webview uses action IPC and never starts the development server. The v5 starter has50 founders,technical ceiling2000 and20°C/regeneration4, based on100×100000 headless validation; v4's historical capacity200 evidence remains in the baseline reports. Higher-density runs can collapse. Phase2 compact snapshots carry morphology/habitat summaries and detailed genomes remain on-demand; worker pacing and read-only analytics do not change simulation state or RNG.

Only environment commands affect the world. No interface edits genomes or grants traits. Scientific names, history, and charts derive from recorded simulation state. This is artificial life, not a predictive scientific model.

Phase 1.1 adds read-only calibration in sim-benchmark: independent worlds can execute on separate threads, while each world's original single-thread tick order remains intact. Metrics and analytical classification never write state or consume RNG. Results include config, environment, sampling convention, final hash and timing; timings are not deterministic metrics.

sim-app maintains a separate file session: current path, last manually saved tick/command count, and at most five recent paths. `worlds/preferences.json` is atomically replaced application data, outside State, save format, replay and hash. Missing recent files remain visible and removable. Preference persistence errors cannot invalidate a successfully saved world. Autosave does not clear the manual-save dirty marker.

The production frontend uses Tauri's supported dialog plugin with `.genesis` filters. Before New/Load/Close replaces a world, the frontend pauses the worker and reads its fresh dirty marker, then offers Save/Discard/Cancel. Cancellation or failed save/load restores previous playback. File operations are serialized, and close events are intercepted until the decision completes. Simulation progression counts as unsaved changes. Native dialog cancellation does not fall back to `manual.genesis`; that default remains only for existing internal/headless callers.

