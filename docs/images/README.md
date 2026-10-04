# README 圖片來源

三張 PNG 是實際程式截圖，不是 AI 生成的介面概念圖。

- 來源：`phase2/multicellular-ecology`，commit `a03c28145f5eee1f33f4525a8ca57b5e4c797e14`。
- 環境：Windows，Edge headless／Playwright，1600 × 1000；React 開發介面連接同一 Rust sim-app worker 的 loopback adapter。
- 世界：simulation v5，seed 42；世界與個體截圖位於 tick 3664。短程世界仍只有初始物種。
- 操作：Play、64x、Pause、選取個體、Species 與 World 切換。
- 檔案：`world.png` 世界與資源趨勢；`individual.png` 個體與親代性狀；`species.png` 物種登錄與統計。
- 限制：不是 v0.1.0 原生 Windows 執行畫面，也不是 Phase 2 installer UAT；沒有證明穩定複雜生態或新物種分化。曲線最新樣本可以早於即時世界 tick。

原始圖片未做內容合成。截圖用來介紹介面與觀測能力，正式實驗結果應查閱對應分支的 PHASE2_REPORT.md。
