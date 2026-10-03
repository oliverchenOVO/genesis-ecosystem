# Phase 2 native production UAT

Status: pending. Phase1 UAT cannot validate v5 morphology or ecology. No claim of Phase2 native UAT pass is made.

Final validation must use a separate Phase2 production build/artifact directory, preserving the v0.1.0 executable/installer and release. Required flows: new fixed-seed v5 world; procedural morphology and inspector; species/evolution/history; productivity/temperature/terrain layers; speeds/MAX/responsiveness; native Save/SaveAs/Recent/dirty guards; close/reopen/load/continue/replay; actual legacy v4 save clear rejection preserving active v5 world; resize and error states.

Record exact build hash, measured seed/tick/population/traits, native action outcomes, replay hash, defects/fixes and retest. Installer build alone does not imply interactive installation was exercised.

Development build92E9AB5D... launched successfully. Seed42/tick0/50founders; organism#2 morphology observed:1segment/1appendage/mass727/armor5.5%/Grazer bite18/locomotion809/sensory188/movement energy rate0.32. Species registry actual mean1.00segments,mass769,armor6.0%,bite22; tree founder glyph visible. Resources/Productivity/Temperature/Terrain layers reflect real cell data. Native Save to artifacts/phase2-development-uat.genesis succeeds. Close succeeds (window/process disappears).

Restart diagnosis: subsequent launches initially exit101, Tauri WebView2 create error HRESULT0x800700AA (resource in use); orphan browser using this app profile remained after close. The orphan is scoped to local.genesis.ecosystem and parent GENESIS process no longer exists; no user browser profile was touched. Recovery and restart/load/continue/replay are still under validation; not markedPASS. Raw startup stderr retained in ignored artifacts/phase2-native-stderr.log and recovered-stderr.log.
