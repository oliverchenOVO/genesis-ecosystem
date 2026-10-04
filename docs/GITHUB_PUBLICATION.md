# GitHub 公開紀錄

使用者已明確選擇直接把既有 `oliverchenOVO/genesis-ecosystem` 從 private 改為 public，保留完整歷史、原 Releases 與 Actions 紀錄。此操作已完成；沒有建立展示用的替代 repository，也沒有重寫 Git 歷史。

## 保留與更新的內容

- 原 repository ID `1402691891` 保持不變；default branch 仍為 `main`。
- 原 `main` 的 Phase 1／1.1 程式內容保持不變。新增的是中文 README、AI 協作說明與真實截圖。
- Phase 2 程式仍在 `phase2/multicellular-ecology`，沒有為公開而合併到 main；生態驗收仍是 **PHASE 2 INCOMPLETE**。
- 原 v0.1.0 Release ID、兩個 asset ID 與 SHA256 digest 均保持；公開前清單中的 26 個 Actions run ID 均仍可查到。
- 公開前 stars／forks 都為 0；改為公開後仍為 0。後續外部操作可能改變數量。
- main README 提供三張 simulation v5、seed42 的真實截圖、Mermaid 架構、兩個分支的驗收狀態、重現指令與研究限制。來源見 [圖片紀錄](images/README.md)。

## 公開前檢查

Gitleaks v8.30.1 的 Windows 執行檔從官方 Release 取得，下載 zip SHA256 與該 Release checksums 清單比對通過。使用 redacted output 執行：

- `git --log-opts="--all --full-history"`：45 個 commits，約143.25 MB 的歷史差異，沒有 leak 命中。
- 對全部26個當時已完成、可下載的 Actions 日誌執行 `dir`：約6.30 MB，沒有 leak 命中。
- 對新 main 工作目錄執行 `dir`：約1.95 MB，沒有 leak 命中。

這些是 scanner 的實際結果，不保證未知格式或二進位內容不存在秘密。初步 blob／gzip 盤點仍記錄21個歷史 blob 中的個人電腦路徑；依使用者保留完整歷史的選擇，沒有為展示改寫原始證據。詳細 redacted reports 與 API 保留核對留在本機忽略的 `artifacts/`，不複製憑證內容到文件。

官方操作參考：[GitHub repository visibility](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/setting-repository-visibility)；掃描工具：[Gitleaks](https://github.com/gitleaks/gitleaks)。

## 後續維護

專案尚未加入 LICENSE；公開可見性與開源授權是不同狀態。作者個人貢獻仍需依實際參與補充，見 [AI 協作說明](AI_COLLABORATION.md)。短影片與個人網頁部署可以另外製作；本次只完成 repository 公開與圖文 README，不宣稱已部署網頁、完成 Phase 2 installer UAT 或取得新的生態成果。
