# 觀測介面改善（2026-10-11）

此變更位於 `phase2/multicellular-ecology`，沒有修改 Rust simulation、RNG、保存格式或驗收門檻，也沒有合併到 main／更新 installer。

## 行為改善

- 圖表依實際 sample tick 排列橫軸，不再把不等間隔觀測畫成等間隔；保留舊呼叫未提供 tick 時的等距模式。
- 空資料明示等待第一次取樣；單筆資料顯示測量點。圖表顯示最新 sample tick，讓即時世界數字與最近曲線樣本的差異可辨識。
- 計算圖表上界時不展開整個陣列作為函式參數，避免長歷史超出引擎參數上限；20萬筆資料有回歸案例。沒有據此宣稱整體模擬效能提升。
- Species 頁新增名稱／精確 ID（例如 `#1`）搜尋，可與 All／Living／Extinct 篩選組合；列表與詳情使用同一結果集合。
- 沒有符合結果時不再顯示無關物種詳情；顯示結果數與空結果提示。導覽祖先時會清除搜尋與狀態篩選，使祖先能出現在列表。
- 新增 SVG 瀏覽器圖示，修正 preview 的 favicon 404。

## 驗證

- 前端31項測試通過：原24項保留，新增7項涵蓋不等間隔 tick、20萬筆歷史、單點／空資料與物種查詢。
- Rust v5 golden 7項、Node證據檢查26項通過；Rust／fixtures 檔案沒有差異。
- production build、frontend format／lint／typecheck 通過。
- Browser plugin 不在可用技能中，因此使用 bundled Playwright + Edge headless，連接 production preview `http://127.0.0.1:1420` 與真實 Rust worker。
- 1600×1000、800×1000 實測 Extinct 空列表不顯示舊詳情、ID 搜尋、無結果恢復、Living 篩選，以及 Play／64x／Pause 與 sample tick 標籤；800px 沒有水平溢出。最終 console／page errors 為0。

首次開發服務載入逾時已保留在工作紀錄；之後改用 production preview 完成驗證。初次 console 檢查抓到 favicon404，修復後重新驗證，沒有忽略該錯誤。當時有其他運算工作執行，build／test 用時不作效能比较。

此次沒有重跑大型生態矩陣或建立新的生態結論，也沒有執行新 Windows 原生／installer UAT。搜尋只讀取既有物種資料；篩選與圖表不向核心注入個體或性狀。
