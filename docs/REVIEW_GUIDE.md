# GENESIS 審閱與重現指南

這份指南供研究所推甄、技術面試與作品集審閱使用。它把專案的主張連結到可閱讀的程式、原始紀錄與可執行的驗證；個人貢獻應搭配 [AI 協作揭露](AI_COLLABORATION.md) 閱讀。

## 先看這三件事

1. **工程問題**：如何把世界狀態、介面與分析分開，使固定 seed 的世界能在保存、載入與 replay 後重現？
2. **研究方法**：如何保留不同候選規則的實驗資料，並在結果不符合門檻時拒絕候選，而不是強制產生展示效果？
3. **已知限制**：main 是已驗收的 v4 基準；Phase 2 v5 尚未通過複雜生態驗收。headless 驗證、原生驗收與科學有效性是不同層次。

## 約兩分鐘：看成果與版本

從 [README](../README.md) 的真實截圖、架構與驗收表了解系統，再看 [v0.1.0 Release](https://github.com/oliverchenOVO/genesis-ecosystem/releases/tag/v0.1.0) 與 [Actions](https://github.com/oliverchenOVO/genesis-ecosystem/actions)。README 圖片來自 Phase 2，不能據此推論 main 已包含該分支的所有功能。

## 約十分鐘：由主張追到證據

| 主張／要回答的問題 | 程式或定義 | 可查的測量／驗證 |
| --- | --- | --- |
| 固定 seed 為何能重現？ | [RNG](../crates/sim-core/src/rng.rs)、[世界更新](../crates/sim-core/src/world.rs)、[determinism 說明](DETERMINISM.md) | [golden 測試](../crates/sim-core/tests/golden.rs)；不要把 benchmark 用時視為 deterministic 指標 |
| 保存後如何確認世界沒有改變？ | [persistence](../crates/sim-core/src/persistence.rs)、[replay](../crates/sim-core/src/replay.rs) | [保存示範的驗證 JSON](../benchmarks/showcase-replay-v4.json) |
| v0.1.0 驗收包含哪些操作？ | [Phase 1.1 報告](PHASE1_1_REPORT.md) | [原生 UAT 紀錄](PHASE1_1_UAT.md)；只適用對應版本 |
| 如何判定候選規則失敗？ | [Phase 2 R4 報告](https://github.com/oliverchenOVO/genesis-ecosystem/blob/phase2/multicellular-ecology/docs/PHASE2_REPORT.md) | [40 世界候選矩陣](https://github.com/oliverchenOVO/genesis-ecosystem/blob/phase2/multicellular-ecology/benchmarks/phase2-r4-candidate-matrix.json)；原始門檻未降低 |
| 怎麼防止「看起來合理」的分析掩蓋壞資料？ | [時間序列分析檢查](https://github.com/oliverchenOVO/genesis-ecosystem/blob/phase2/multicellular-ecology/scripts/temporal-ecology-checks.mjs) | 同檔案中的缺漏、損壞與收支不一致案例；只讀分析不改寫世界 |

## 實際驗證：保存檔 → replay → 再走1000 ticks

這個範例使用 **main 的 simulation v4** 和已提交的真實 seed11 世界，沒有重新等待十萬個 ticks，也沒有為示範植入物種。第一次 Rust 編譯可能需要較久；replay 仍會重建完整107504 ticks，時間依機器而異。

```powershell
git clone https://github.com/oliverchenOVO/genesis-ecosystem.git
cd genesis-ecosystem
git switch main
cargo test --release --locked -p sim-core --test golden
cargo run --release --locked -p sim-benchmark -- --verify-save examples/seed11-107504.genesis --continue 1000
```

預期輸出重點：

| 欄位 | 預期值 |
| --- | --- |
| `verified` | `true` |
| `restored.seed`／`restored.tick` | `11`／`107504` |
| `restored.population`／`restored.species` | `200`／`2` |
| `restored.hash` | `9942ea1d1033f6db883e6830aa5489bf457db646eb3493a8139ffa54f28f659a` |
| `continuation_ticks` | `1000` |
| `continuation_hash` | `113628ad459de576452240cff339879b3a4c96bb5257696e7d1234c46b75280d` |

輸出應與 [已保存的驗證紀錄](../benchmarks/showcase-replay-v4.json) 相符。此命令驗證完整 replay 與載入後續跑，不修改來源保存檔；它不驗證本機原生對話框或安裝流程。不要用 Phase 2 的 v5 執行檔載入這個 v4 保存檔。

若要看介面，依 README 啟動桌面或本機開發模式，載入 `examples/seed11-107504.genesis`，觀察 Species／History 並執行 Verify replay。瀏覽器開發模式與 Windows 原生檔案對話框的操作方式不同，原生驗收請對照 UAT 紀錄。

## 面試可以討論的設計取捨

- 為什麼比較完整世界 hash，而不是只比較人口數或畫面？
- RNG 呼叫次序、觀測工具與 worker／UI 分離如何影響重現性？
- 為什麼保留失敗的四個生態候選？哪些新證據才足以支持下一個規則變更？
- population hard cap 為什麼會限制生態結果的解讀？
- AI 協助產生程式後，作者如何理解、驗證與負責最後的設計？

這些是討論題，不是代寫個人經歷或能力證明。README／測試能提供專案證據，作者仍應以自己實際參與的工作回答。
