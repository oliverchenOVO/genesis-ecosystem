# Phase 1.1 production Windows UAT

首次驗收：原始 Phase 1 release executable（commit 687f7f4），2026-10-03。
使用 computer-use 的 `@oai/sky` 操作實際 Windows 視窗；頁面為 `http://tauri.localhost/`，沒有 dev server。
此表持續更新；初測通過不代表修改後的最終驗收通過。

| Scenario | Expected | Actual / initial evidence | Severity / fix | Final retest |
|---|---|---|---|---|
| Launch / New World | 固定 seed 建立合法世界 | seed 11、512、50/200、mutation 100，成功啟動 | 無 | PASS; see final retest |
| Pause / resume / speed / MAX | 控制 pacing、不改世界規則 | 1x → MAX → 1x → pause，停在 27,600；載入後 MAX 繼續至 701,520 | 無 | PASS; see final retest |
| Environment | 草稿只在 Apply 記錄命令 | tick 27,600 套用 18°C / regen 9；recorded pressure 更新 | 無；完整 command replay 另重測 | PASS; see final retest |
| Organism | 顯示真實親代、lineage、性狀與 genome | #3397 generation 34、parents #3201 × #3203、lineage #1、14 loci、age 1713、energy 2255/2258 | 無；死亡選取待重測 | PASS; see final retest |
| Species | living/extinct 及歷史保留 | tick 701,520 共 22 records / 9 living；Photovorus minor-2 origin 48,600、generation 60、extinct 512,954、ancestor Silvavita prima-1 | 無 | PASS; see final retest |
| Evolution | zoom / pan / node jump | 100% → 120%；drag 改變節點位置；minor-2 跳至正確 Species 詳情 | P2：pan 同時選取文字；加 user-select:none | PASS; see final retest |
| History | 真實 population / divergence events | first candidate 46,900；speciation 48,600，distance 118 / founders 16；人口每 1000 ticks 記錄 | 無；環境與滅絕 feed 待最終驗收 | PASS; see final retest |
| Save → close → reopen → Load | 完整恢復世界 | 27,600 / seed 11 / 200 population / generation 39；telemetry 277 samples 恢復；可繼續自然演化 | P2：固定 manual.genesis；改 native picker / Save As | PASS; see final retest |
| Replay | 已知 run checkpoint match | production Verify replay：tick 27,600、hash prefix 8ce637e48fba4f7f | 無 | PASS; see final retest |
| New / Load / Close unsaved | Save / Discard / Cancel | 原始程式沒有 dirty guard，僅文字提醒 | P1：建立 application session tracking 與保護流程 | PASS; see final retest |
| Native picker / recent / errors | 選檔取消安全、錯誤可讀 | 原始程式沒有 native picker / recents | P1：本 Phase 實作 | PASS; see final retest |
| Resize / minimum / empty / high speed | 可操作、無 clipping | 初始 1440×1000 可操作；footer 需捲動；no selection / empty chart 可讀 | P2：必要 layout 重測 | PASS; see final retest |

P0：crash、corruption、replay mismatch、data loss、invariant violation。
P1：核心流程錯誤或重大互動失敗。P2：可用性、標籤、layout。P3：純 polish。
初測問題與 final native close P1 已修復並驗收；詳見下列最終結果。

## Final production retest — 2026-10-03

Native executable only, actual Windows UI through `@oai/sky`, no dev server. Native file UX was tested on SHA256 CE832E2A09C066938B721CA1EEFF68764C2BA395396C2078442871FB7482C0F5. Final close-permission fix changed only the native capability and added a Rust regression test; identical frontend was retained. Final executable SHA256 16FB27A061D6F817BDC68FE26897A6C69E1814FDB2110AD23C0A623ABE3C56C9 was launched and used for the remaining native UAT and close/restart retest.

| Gate | Measured outcome | Status |
|---|---|---|
| Fixed seed / New | seed 11, size512, 50/200, mutation100; running, then paused tick20,018 | PASS |
| New dirty protection | Cancel returned to configured New dialog, old seed42 retained; retry Discard created seed11 | PASS |
| Simulation | Play/Pause,4x,MAX,return1x; resumed saved world20,018→20,501; final build MAX/1x/Pause also exercised | PASS |
| Environment | Draft18°C/regen9 did not change recorded20°C/12 until Apply; command tick20,018 marked dirty at same tick | PASS |
| Save | Actual Windows Save dialog, GENESIS filter, filename entered without extension → uat-phase1_1-first.genesis, 64,015 bytes | PASS |
| Save As / cancellation | New native dialog prefilled current filename; Cancel kept old path/world; second Save As wrote distinct copy, 64,015 bytes | PASS |
| Save path reuse | Close→Save reused copy path without native dialog; saved command at20,018, later overwritten safely at20,501 | PASS |
| Close guard | Cancel preserved world; Save initially exposed P1 below, fixed build Save then normal exit confirmed; final dirty Discard also normal exit | PASS |
| Restart / native Load | Reopened production build, actual Open dialog and GENESIS filter; loaded copy seed11 tick20,501 population200 species1 generation31 | PASS |
| Load guard | Recent Load Cancel preserved initial seed42 and dialog; retry Discard loaded actual seed11 world | PASS |
| Recent | Both saved paths persisted across restart; moved test file marked unavailable/disabled, Remove deleted entry only; actual file retained | PASS |
| Invalid / corrupt / version | Native Open loaded text-invalid, flipped-checksum, simulation-v99 fixtures; readable errors, unchanged seed11/tick20,501, no crash | PASS |
| Organism / lineage | #2560 generation26, parents2368×2544, lineage1, age1694, health100%, six traits and14 genome loci | PASS |
| Species / ancestry | Genuine example107,504 restored200 organisms / two species. minor-2:38 living, tick48,600/gen60/founders16/distance118; ancestor navigation to prima-1:162 | PASS |
| Evolution | 100→120%, pan visibly moved nodes without selecting text; panned minor-2 node opened correct Species details | PASS |
| History | Population every1000 ticks; divergence candidate46,900/48,400 and real speciation48,600; links/ancestry retained | PASS |
| Extinction / empty state | Valid command60°C/regen0 at107,504; minor-2 extinct107,538, prima-1 extinct107,595, zero population. History records command/extinctions, extinct Species retains traits/ancestor | PASS |
| Native replay | Restored20,501 world verified `0a63103c8e1fc417…` matching full CLI report | PASS |
| Resize | Actual Windows Size action narrowed to800px; header, five file controls and all speed buttons visible and usable; vertical scroll intentional | PASS |

### P1 discovered and fixed

The guarded close saved successfully but did not terminate: Tauri `onCloseRequested` invokes `destroy()` when allowed, while capability permitted only `close`. Added `core:window:allow-destroy` scoped to main. This follows the installed Tauri API and [official window API documentation](https://v2.tauri.app/reference/javascript/api/namespacewindow/). Regression test validates close/destroy permissions and main-window scope. Rebuilt both production artifacts, then Close→Save, window absence, restart→native Load→replay, and Close→Discard passed. No simulation behavior changed.

### Save authority and continuation evidence

`benchmarks/native-save-replay-v4.json`: tick20,018, seed11, population200, species1, lineages2, telemetry201, one environment command18°C/9, all four RNG states retained. Full restored hash `1ea2dde5074bba71d55362991e2cfe864aa7a4c3f581727517cb1a6432461119`; independently replayed/restored worlds advance1000 to identical `5f9a131091cce9a93f0d1a4844d0d377b0d9036243f65f86ed7fc348993f1f70`.

`benchmarks/native-restart-replay-v4.json`: Close→Save after native continuation tick20,501, generation31, telemetry206, unchanged command history; restored hash `0a63103c8e1fc417f3cd37e85b524a2334179b37a908b14b4cbb8b46359bd8ef`; independent continuation1000 hash `9cc8a7ffc92edd112f21dc0045cd72d0ab52210b704c9b2c3f18bcad17b4e166`.

UI snapshots and filesystem evidence are real. Invalid fixtures and manual UAT saves remain ignored in artifacts/. Original genuine showcase save is unchanged. Not every OS-permission edge case or every guard-option cross-product was manually exercised: shared guard failure/cancel/MAX-resume logic, recent cap5, all corruption cases and real rotating autosaves are covered by automated tests. Dead selected-organism message was not separately repeated in this final session; actual empty/extinct state and preserved ancestry were verified. No known P0 or unresolved critical P1.

