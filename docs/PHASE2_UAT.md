# Phase 2 native production UAT

Status: INCOMPLETE. Phase1 UAT cannot validate v5 morphology or ecology. No claim of Phase2 native UAT pass is made.

Final validation must use a separate Phase2 production build/artifact directory, preserving the v0.1.0 executable/installer and release. Required flows: new fixed-seed v5 world; procedural morphology and inspector; species/evolution/history; productivity/temperature/terrain layers; speeds/MAX/responsiveness; native Save/SaveAs/Recent/dirty guards; close/reopen/load/continue/replay; actual legacy v4 save clear rejection preserving active v5 world; resize and error states.

Record exact build hash, measured seed/tick/population/traits, native action outcomes, replay hash, defects/fixes and retest. Installer build alone does not imply interactive installation was exercised.

Development build92E9AB5D... launched successfully. Seed42/tick0/50founders; organism#2 morphology observed:1segment/1appendage/mass727/armor5.5%/Grazer bite18/locomotion809/sensory188/movement energy rate0.32. Species registry actual mean1.00segments,mass769,armor6.0%,bite22; tree founder glyph visible. Resources/Productivity/Temperature/Terrain layers reflect real cell data. Native Save to artifacts/phase2-development-uat.genesis succeeds. Close succeeds (window/process disappears).

Restart diagnosis: subsequent launches initially exit101, Tauri WebView2 create error HRESULT0x800700AA (resource in use); orphan browser using this app profile remained after close. The orphan is scoped to local.genesis.ecosystem and parent GENESIS process no longer exists; no user browser profile was touched. Recovery and restart/load/continue/replay are still under validation; not markedPASS. Raw startup stderr retained in ignored artifacts/phase2-native-stderr.log and recovered-stderr.log.

Profile isolation fix: create the main WebView after installing the real simulation worker, with data directory app_data/webview-sim-v5. The existing app_data/worlds save/preferences directory remains unchanged. A production launch after this fix records application_startup, simulation_created(seed42) and actual webview_page_loaded(http://tauri.localhost/) in phase2-profile-fix-stdout.log with no startup panic. Native capture/input was then stopped by the user's physical Escape key. No further Computer Use input was issued. Therefore restart/Load/continue/replay, legacy rejection UX, MAX responsiveness and final-build UI UAT remain unverified, even though headless equivalents pass.

An additional review production build is saved under target/phase2-review/release, so the running diagnostic build is not overwritten. It includes the initial-habitat form/atomic backend and profile isolation. Building an installer is not an installation or native acceptance pass. Final review build hashes must be refreshed after any calibrated default change. No final release/tag is permitted until these remaining UAT gates pass.
