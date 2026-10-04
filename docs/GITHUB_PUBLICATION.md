# GitHub 公開準備

狀態：準備中；repository 保持私人。這份清單準備公開審閱材料，不代表產品發佈、生態驗收通過或已授權改變 repository 可見性。

## 公開方案

最保守的方案是先公開個人網頁上的專案摘要、架構與真實展示影片，保留研究 repository 私人。若需要公開程式碼，先完成下列阻擋項，再選擇公開既有 repository，或由已審查檔案建立獨立展示 repository。後者可減少歷史資料曝露，但必須明示是來源專案的展示快照；不能偽造開發歷史。

目前 GitHub 預設分支為 `main`，R4 收尾位於 `phase2/multicellular-ecology`。訪客只看預設分支時不會自動看見最新 R4 材料。公開時需明確連結展示分支；不應為展示直接把未完成 Phase 2 合併進受保護的 baseline。

R4 收尾 HEAD 的 tracked 檔案合計約 427.94 MiB，主要是研究證據；這不是 clone 大小或完整歷史大小。個人網頁可只展示摘要、架構與少量真實媒體，再連結證據。若採展示快照，先定義需要哪些證據及來源連結，不在原 repository 中刪除 frozen archives 來縮小展示。

## 已準備

- README 導覽、[作品介紹](PORTFOLIO.md)、[AI 協作說明](AI_COLLABORATION.md) 與完整驗收報告。
- 原始測量與候選拒絕理由；區分 R3 正式校準與 R4 有界實驗。
- 可重跑的初步資訊曝露盤點：`node scripts/publication-audit.mjs`，結果寫入被忽略的 `artifacts/github-publication-audit.json`，不輸出憑證內容。
- CI 明確只給 `contents: read`，checkout 不保留 credentials；沒有加入部署或發佈權限。

### 初步盤點結果

以 R4 收尾 `83b13e7` 的本機 `--all` refs 為範圍：掃描 655 個文字 blob，展開 18 個 gzip blob；54 個二進位 blob 未做文字 signature 掃描。已知憑證格式命中 0，email 文字命中 0，但有 21 個歷史 blob、共 89 次個人電腦路徑命中。Git author email 仍有 1 個不同值，與檔案內 email 搜尋是不同範圍，需要作者確認是否適合公開。

路徑命中包含 `benchmarks/phase2-finalization-provenance/`、R2/R3 provenance 與 production hash 紀錄、`benchmarks/windows-artifacts.json`，以及部分歷史報告。詳細 blob ID／檔名保存在本機忽略的 audit JSON，沒有把命中的內容複製進公開文件。原始 frozen evidence 尚未改寫；作者應選擇接受這些路徑曝露，或在獨立展示快照中明示遮罩，保留私人原件與對照來源。只修改最新檔案不能移除 Git 歷史中的內容。

這些數字是初步盤點，不是「沒有秘密」的保證。scanner 有 bounded gzip 展開與已知 signature 的限制；遠端才有的 refs、附件與 CI 日誌尚未逐項檢查。

## 公開前的阻擋項

| 項目 | 必須完成的決策／檢查 |
| --- | --- |
| 歷史資訊 | 初步 signature 盤點不是完整 secret scan。對預定公開的全部分支／tag 執行專用工具掃描，確認個人路徑、email、舊文件與壓縮證據是否可公開。真實秘密若曾進入歷史，先撤銷／輪替，再規劃清理歷史 |
| GitHub 遠端資料 | 人工審閱 Actions 日誌、artifact、release、issue、PR、wiki、設定與協作者資訊；本機 blob 掃描沒有涵蓋這些範圍 |
| LICENSE | 尚未選定，不擅自加入 MIT／Apache 等授權；由作者決定是否允許再散布與修改，並確認第三方依賴與資產義務 |
| 設計資產 | 確認現有設計圖、圖示、字型與其他媒體的來源與公開權利。設計圖應標示為概念圖，不能冒充執行截圖 |
| 作者貢獻 | 作者補完並確認 AI_COLLABORATION.md 的個人貢獻說明，供推甄審閱與面試解釋 |
| 展示材料 | 製作本版實際截圖與短影片，保留 seed／版本；不要宣稱短影片證明穩定物種分化 |
| 展示入口 | 決定公開完整歷史或整理過的展示快照、要連結哪個分支，並確認最終 HEAD CI |
| 公開操作 | 作者明確授權後，才改 visibility、建立公開 repository 或部署個人網頁 |

改為公開時，GitHub Actions 歷史與日誌也可能公開；參見 [GitHub visibility 文件](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/setting-repository-visibility)。沒有完成專用歷史掃描前，不應把這份準備清單標示成「已通過安全公開審查」。

公開研究材料不要求先完成所有生態驗收，但必須保留 **PHASE 2 INCOMPLETE** 的事實。CI 產生的 Windows installer 是回歸產物，尚未完成本版原生與安裝器 UAT，不應以合格穩定版對外提供。
