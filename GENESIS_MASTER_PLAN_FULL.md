# GENESIS — AI 生態箱：從單細胞一路演化
## Master Product & Development Plan

> 這份文件是 GENESIS 專案的**完整總規劃**。  
> 範圍從最初產品構想到最終完整版本，包含產品定位、玩法、技術架構、資料模型、simulation architecture、determinism、LLM 邊界、UI / UX、MVP、Phase 1、Phase 1.1、Phase 2、Phase 2.x、Phase 3、測試、benchmark、save/replay、Git/GitHub、Release 與最終驗收策略。  
>  
> 這不是單一 Phase 的報告，也不是只描述目前程式，而是「整個專案從開始到完成應該怎麼做」的總設計文件。

---

# 0. 專案總覽

## 專案名稱

**GENESIS — AI 生態箱：從單細胞一路演化**

中文工作名：

**AI 生態箱**

核心類型：

- Artificial Life
- Evolution Sandbox
- Ecosystem Simulation
- Interactive Natural History
- Deterministic Simulation Product
- Evolution Research / Visualization Tool

最終平台：

- Windows Desktop
- 可執行 EXE
- 可安裝版本
- GitHub 公開展示
- 可放入推甄作品集與個人網站

---

# 1. 最初產品概念

GENESIS 的核心不是「很多生物在畫面上移動」。

核心問題是：

> 如果玩家不控制任何生物，只改變環境，生命會不會真的透過遺傳、突變、競爭、繁殖與自然選擇，形成不同的物種與生態位？

玩家不是養寵物，也不是 RPG 玩家。

玩家的身份更接近：

- 生態系統設計者
- 自然史觀察者
- 演化實驗者
- 人工生命研究者

玩家不可以直接對生物下令：

- 變快
- 變大
- 變聰明
- 變成掠食者
- 獲得抗寒能力
- 直接進化
- 強迫產生新物種

玩家只能改變：

- 溫度
- 地形
- 食物
- 資源分布
- 天敵
- 災害
- 環境壓力
- 生態條件

之後觀察選擇壓力如何改變整個族群。

---

# 2. 產品核心循環

基本 gameplay loop：

```text
建立世界
    ↓
放入最初生命
    ↓
開始模擬
    ↓
觀察族群
    ↓
改變環境
    ↓
形成 selection pressure
    ↓
突變 / 遺傳 / 繁殖 / 死亡
    ↓
allele frequency 改變
    ↓
族群性狀漂移
    ↓
生態位形成
    ↓
lineage divergence
    ↓
speciation / extinction
    ↓
Evolution Tree 更新
    ↓
研究「為什麼發生」
    ↓
再次干預
```

最終產品要讓玩家感受到：

> 我沒有告訴它要怎麼進化，但我可以回頭看到它為什麼變成現在這個樣子。

---

# 3. 最重要的產品差異化

GENESIS 最重要的差異不是「AI」。

而是：

# Evolution has provenance

每一個：

- 物種
- lineage
- extinction
- trait shift
- population change
- ecological transition

都應該可以追溯到真正 simulation 發生過的資料。

例如：

```text
Velocipodus borealis

Origin:
Generation 12,481

Ancestor:
Velocipodus minor

Founder population:
214

Primary divergence:
Cold tolerance +31%
Body mass +18%
Energy storage +24%
Vision -12%
Speed -7%
```

更重要的是：

```text
Temperature:
21.2°C → 8.4°C

Allele MTR-41A:
4.1% → 73.2%

Winter survival:
+14.8%

Lifetime offspring:
+9.7%
```

這些不能由 LLM 自己編。

必須來自 simulation telemetry。

---

# 4. 五個產品支柱

## 4.1 Evolution is real

演化必須來自：

```text
Genome
  ↓
Phenotype
  ↓
Behavior / Physiology
  ↓
Survival / Reproduction
  ↓
Allele Frequency
  ↓
Population Divergence
```

禁止：

```text
if environment == cold:
    cold_resistance += 10
```

正確：

```text
cold environment
→ high-tolerance individuals survive longer
→ reproduce more
→ inherited alleles spread
```

---

## 4.2 Evolution has provenance

所有重要生物學事實都要有來源。

UI 說：

> 這個物種變得更耐寒。

系統應該能指出：

- 哪些 genes 改變
- phenotype 怎麼變
- survival 怎麼變
- reproduction 怎麼變
- 族群頻率怎麼變
- 何時產生 lineage divergence

---

## 4.3 Evolution Tree 是核心產品功能

Evolution Tree 不是附加頁面。

它應該和 World View 一樣重要。

例如：

```text
Proto Cell
│
├── Microvorus
│    ├── Microvorus minor
│    └── Velocipodus
│         ├── V. ruber
│         └── V. borealis
│
└── Photovorus
     ├── P. alba
     └── P. giganteus
```

點擊任何 branch 可以看到：

- origin tick / generation
- ancestor
- genetic distance
- founder population
- trait changes
- morphology changes
- niche changes
- extinction state

---

## 4.4 Procedural Natural History

GENESIS 最終應該產生一個世界自己的自然史。

例如：

```text
Generation 4,188
First persistent predator lineage

Generation 7,920
Northern migration

Generation 12,481
Velocipodus borealis diverges

Generation 18,204
Major extinction

Generation 31,772
Independent armor evolution
```

玩家最後得到的不只是「模擬」。

而是一段：

**Procedural Natural History**

---

## 4.5 Replayable Science

任何世界都應可以：

```text
Seed: ...
Replay
```

同樣：

- simulation version
- seed
- config
- ordered command sequence

必須得到相同世界。

未來還要能：

```text
Generation 10,000
       │
       ├─ World A
       │   normal temperature
       │
       └─ World B
           -8°C
```

做 fork experiment。

---

# 5. 遊戲規則邊界

玩家可以控制：

- seed
- world size
- starting population
- environment
- temperature
- food
- resource regeneration
- terrain
- disasters
- ecological pressure
- simulation speed

玩家不能控制：

- individual genome
- individual trait
- forced mutation
- forced speciation
- species name outcome
- survival outcome
- direct intelligence upgrade
- direct predator conversion

核心原則：

> 玩家操控原因，不操控演化結果。

---

# 6. 整體系統架構

推薦 architecture：

```text
┌──────────────────────────────┐
│       Desktop Frontend       │
│ React + TypeScript           │
│                              │
│ World / Species / Evolution  │
│ History / Analytics          │
└──────────────┬───────────────┘
               │
       Snapshot / Commands
               │
┌──────────────▼───────────────┐
│       Application Layer      │
│                              │
│ Session / Save / Replay      │
│ Preferences / File UX        │
└──────────────┬───────────────┘
               │
┌──────────────▼───────────────┐
│       Simulation Core        │
│            Rust              │
│                              │
│ World                        │
│ Genome                       │
│ Phenotype                    │
│ Ecology                      │
│ Evolution                    │
│ Species                      │
│ Lineages                     │
│ RNG                          │
└──────────────┬───────────────┘
               │
┌──────────────▼───────────────┐
│ Persistence / Telemetry      │
│ Save                         │
│ Replay                       │
│ Checkpoints                  │
│ Statistics                   │
└──────────────┬───────────────┘
               │
┌──────────────▼───────────────┐
│ Optional AI Layer            │
│ Phase 3+                     │
│ Naturalist / Narration       │
└──────────────────────────────┘
```

Simulation Core 必須可以：

- 沒有 UI
- 沒有網路
- 沒有 LLM
- 沒有 Tauri

獨立 headless execution。

---

# 7. 技術棧

## 7.1 Simulation

**Rust**

原因：

- deterministic control
- performance
- memory safety
- native Windows
- benchmark 容易
- 型別與資料邊界清楚
- 適合長期 simulation core

推薦 workspace：

```text
crates/
├─ sim-model/
├─ sim-core/
├─ sim-save/
├─ sim-replay/
├─ sim-benchmark/
└─ sim-app/
```

---

## 7.2 Desktop

**Tauri 2**

理由：

- Windows EXE
- Rust backend
- Web frontend
- native file access
- native dialog
- package/install
- 比 Electron 更自然地與 Rust core 結合

---

## 7.3 Frontend

- React
- TypeScript
- Vite
- pnpm

Frontend 不得自己重新實作 simulation。

只允許：

- send commands
- request snapshots
- render
- inspect data

---

## 7.4 Visualization

World：

- Canvas
- WebGL
- 後續需要時才考慮 WebGPU

Charts：

- ECharts 或成熟 chart library

Evolution Tree：

- SVG / Canvas
- lineage model 自己維護
- 不綁死某個 chart library

---

# 8. Determinism

Determinism 是一級產品功能。

必須保證：

```text
same simulation version
same seed
same config
same commands
=
same checkpoint hashes
```

---

## 8.1 Authoritative numeric model

權威 state 優先使用：

- integer
- scaled integer
- fixed-point style

例如：

```text
temperature = 1842
```

代表：

```text
18.42°C
```

Frontend 才轉 float 顯示。

---

## 8.2 RNG

建立 centralized deterministic RNG。

禁止：

- rand() 隨便呼叫
- Math.random()
- system clock
- UI randomness

進入 simulation truth。

Logical streams：

```text
WORLD
RESOURCE
BEHAVIOR
REPRODUCTION
MUTATION
DISASTER
```

目的：

某 subsystem 多抽一次 RNG，不要把其他 subsystem 整條序列打亂。

---

## 8.3 RNG Version

Save 必須保存：

- seed
- RNG version
- RNG state

例如：

```text
RNG_VERSION = 1
```

如果 RNG algorithm 改變：

必須 version bump。

---

# 9. Simulation Tick Architecture

Simulation tick 不得與 render FPS 綁定。

```text
Render FPS != Simulation ticks
```

固定 pipeline 例：

```text
1. Environment
2. Resource regeneration
3. Spatial index
4. Perception
5. Intent
6. Movement
7. Feeding
8. Predation / combat
9. Metabolism
10. Damage
11. Death
12. Mate matching
13. Reproduction
14. Crossover
15. Mutation
16. Birth
17. Population accounting
18. Lineage update
19. Species detection
20. Historical events
21. Telemetry
22. Checkpoint hash
```

每次順序固定。

---

# 10. Intent / Resolution 模型

不要：

```text
A.update()
→ A 直接殺死 B
→ B 沒機會執行
```

避免 iteration-order dependency。

推薦：

```text
read immutable state
→ produce intent
→ deterministic resolution
→ commit
```

---

# 11. Spatial Architecture

Phase 1 / 2D 世界：

```text
continuous logical 2D
+
uniform spatial hash
```

不要每隻生物掃全部其他生物。

避免：

```text
O(N²)
```

Spatial index 支援：

- nearby organisms
- food lookup
- mate lookup
- predator/prey
- local environment

---

# 12. World Data Model

概念：

```text
WorldState
├─ tick
├─ generation
├─ config
├─ environment
├─ resources
├─ organisms
├─ species
├─ lineages
├─ ancestry
├─ history
├─ telemetry
└─ rng_state
```

---

# 13. Entity IDs

使用 deterministic strong IDs：

```text
OrganismId
GenomeId
LineageId
SpeciesId
EventId
```

禁止 authoritative UUID v4。

---

# 14. Genome Model

最初 genome 不應只是 RPG stats。

概念：

```text
Genome
├─ morphology
├─ metabolism
├─ sensory
├─ locomotion
├─ reproduction
├─ behavior
└─ regulation
```

Phase 1 先簡化。

基本 traits：

- body size
- speed
- vision
- basal metabolism
- food efficiency
- reproduction threshold
- offspring tendency
- mutation tendency
- aggression
- cooperation / social affinity
- temperature optimum
- temperature tolerance
- energy storage

---

# 15. Genome → Phenotype

Genome 和 phenotype 分離。

```text
Genome
  ↓
derive_phenotype()
  ↓
Phenotype
```

Gameplay 永遠讀 authoritative phenotype。

環境也可以影響 expression / realized performance。

所以：

```text
Genome != Phenotype
```

---

# 16. Organism Model

概念：

```text
Organism {
    id
    birth_tick

    parent_a
    parent_b

    genome
    phenotype

    position
    velocity

    energy
    health
    age

    species_id
    lineage_id

    behavior_state
}
```

避免每個 organism 存：

- UI strings
- description
- translated text
- narrative

---

# 17. Metabolism

所有 organism 每 tick 消耗 energy。

依據：

- body size
- basal metabolism
- movement
- temperature suitability
- morphology complexity
- activity

Food 補充 energy。

Energy 不足：

```text
health ↓
survival ↓
reproduction ↓
```

---

# 18. Resources

Phase 1：

- 一種主要環境 resource
- grid / field-based resource
- maximum capacity
- regeneration
- fertility modifier

不要每個 food 粒子都是 heavyweight entity。

Phase 2：

- soft resource
- hard resource
- prey energy
- spatial patching
- resource niches

---

# 19. Behavior

不使用 LLM。

Phase 1：

Utility AI。

Intent：

- SeekFood
- SeekMate
- Flee
- Hunt
- Explore
- Rest

例如：

```text
Eat      0.78
Mate     0.62
Flee     0.12
Explore  0.31
```

tie-break deterministic。

---

# 20. Reproduction

條件：

- maturity
- sufficient energy
- mate compatibility
- distance
- local availability

流程：

```text
Parent A
    \
     inheritance/crossover
    /
Parent B
      ↓
Offspring Genome
      ↓
Mutation
      ↓
Phenotype
```

Child 保存 parent IDs。

---

# 21. Mutation

Phase 1：

- small point mutation
- rare larger mutation

Phase 2：

- morphology loci
- segment change
- armor shift
- sensory shift
- locomotion shift
- feeding morphology change

重要：

Mutation 對 fitness 必須是 blind。

禁止 environment-directed mutation。

---

# 22. Selection

Selection 不寫成特殊規則。

例如：

錯誤：

```text
cold → evolve cold resistance
```

正確：

```text
cold
→ tolerant organisms retain more energy
→ higher survival
→ higher reproduction
→ allele spreads
```

---

# 23. Lineage

Lineage 是一級資料。

至少記：

```text
LineageRecord
- id
- origin_tick
- ancestor
- founder info
- population
- representative genome
```

Lineage != Species。

---

# 24. Species

Species 不能每個 mutation 都成立。

判定可以依：

- genetic distance
- population cluster
- gene flow
- reproductive compatibility
- minimum population
- persistence

例如：

```text
candidate population >= threshold
AND
genetic distance >= threshold
AND
gene flow sufficiently low
AND
persists for N ticks
→ SpeciationEvent
```

---

# 25. Scientific Naming

Phase 1 scientific name 必須 deterministic。

例如：

```text
Velocipodus ruber
Microvorus minor
Photovorus alba
```

輸入：

- lineage
- phenotype
- species ID
- seed

名字建立後永久保存。

LLM 不負責 authoritative scientific name。

---

# 26. Extinction

Species population 歸零並達確認條件後：

```text
ExtinctionEvent
```

Species 在 Evolution Tree 永遠保留。

---

# 27. History

History 保存重要事件：

- speciation
- extinction
- population milestone
- environment intervention
- ecological transition
- major innovation（後續 Phase）

不保存每次 movement / feeding。

---

# 28. Telemetry

固定 interval 取 aggregate snapshot。

World：

- population
- species count
- food abundance
- temperature
- birth rate
- death rate
- diversity

Species：

- population
- body size
- speed
- vision
- metabolism
- tolerance
- morphology
- niche（Phase 2）

Telemetry 支援：

- chart
- analysis
- explanation
- AI Naturalist

---

# 29. Deterministic / LLM 邊界

## 永遠 deterministic

- RNG
- mutation
- inheritance
- reproduction
- movement
- metabolism
- death
- resources
- disasters
- predation
- species detection
- lineage
- extinction
- history truth
- scientific naming
- save
- replay
- world hash

---

## LLM 可以做

Phase 3+：

- Naturalist descriptions
- common names
- documentary narration
- natural-language explanation
- Q&A assistant
- high-level interpretation

LLM 只能讀：

- telemetry
- species record
- history
- lineage
- environment

不能修改 world truth。

---

# 30. Save Architecture

正式檔案：

```text
.genesis
```

內容概念：

```text
SaveManifest
SimulationVersion
RngVersion
SaveVersion
WorldState
SpeciesRegistry
LineageRegistry
Ancestry
History
Telemetry
CommandLog
Checksum
```

Phase 1 最終：

```text
save format v1
simulation v4
RNG v1
```

---

# 31. Save Safety

流程：

```text
temporary file
→ complete write
→ checksum
→ fsync / sync
→ atomic replace
```

損壞檔案不能 crash。

---

# 32. Autosave

輪替：

```text
autosave A
autosave B
autosave C
```

Autosave 不能：

- consume RNG
- affect world
- clear manual dirty state

---

# 33. Replay

Replay 不錄 frame。

記：

- seed
- config
- ordered commands
- checkpoint hashes
- final tick

Replay：

```text
initial state
+
commands
→ deterministic rebuild
```

---

# 34. World Hash

Canonical authoritative state hash。

不包含：

- UI
- wall clock
- memory addresses
- unstable map ordering

用途：

- golden tests
- replay
- save continuation
- CI regression

---

# 35. Fork Experiment（後續）

未來：

```text
checkpoint at generation 10,000
        ↓
    ┌── World A
    └── World B
```

比較：

- species
- diversity
- extinction
- morphology
- population
- niches

---

# 36. UI / UX 方向

整體風格：

**Modern Natural History Research Station**

不是：

- debug UI
- engineering control panel
- default HTML

主要 workspace：

```text
World
Species
Evolution
History
Analytics
```

---

# 37. World View

顯示：

- ecosystem canvas
- organism movement
- resources
- environment
- simulation state

Header：

- tick / generation
- population
- species count
- seed
- speed
- pause

---

# 38. Time Controls

至少：

```text
Pause
1x
4x
16x
64x
MAX
```

MAX：

優先 simulation。

可以降低 render frequency。

---

# 39. Organism Rendering

Phase 1 不需要精美 sprite。

但不能全部完全一樣。

Visual variables：

- size
- aspect ratio
- body tint
- predator marker
- motion trail
- morphology hints（Phase 2）

---

# 40. Organism Inspector

顯示：

- ID
- species
- lineage
- age
- energy
- parents
- children
- genome summary
- phenotype
- traits

Phase 2 加：

- segments
- mass
- armor
- mouth
- sensory
- locomotion
- complexity

---

# 41. Species View

顯示：

- scientific name
- living/extinct
- origin
- ancestor
- population
- trait averages
- morphology
- distribution
- population history
- trait history

---

# 42. Evolution Tree

支援：

- zoom
- pan
- select node
- ancestor/descendant

Phase 2：

顯示 morphology delta。

---

# 43. History View

Chronological feed：

- generation/tick
- speciation
- extinction
- environment commands
- important population events

---

# 44. Charts

Phase 1：

- total population
- species count
- food abundance
- selected traits

Phase 2：

- morphology
- niche
- feeding role
- lineage temporal diet

---

# 45. MVP

MVP 目的：

> 證明這是真的演化，不是 scripted animation。

只做：

- seeded world
- genome
- phenotype
- food
- movement
- metabolism
- death
- reproduction
- mutation
- inheritance
- temperature
- spatial hash
- deterministic RNG
- world hash
- headless benchmark

驗收：

```text
100 seeds
×
50,000 ticks

0 crashes
0 invalid state
0 replay mismatch
```

---

# 46. Phase 1 — First Real Product Slice

目標：

> 一個可以打開展示，而且真的可以看到演化發生的 Windows application。

---

## 46.1 Simulation

- genome
- inheritance
- mutation
- metabolism
- food
- perception
- movement
- death
- reproduction
- predation
- aggression
- cooperation affinity
- temperature pressure
- spatial hash
- generations

---

## 46.2 Evolution

- lineage tracking
- genetic distance
- species candidate
- speciation
- extinction
- deterministic scientific naming
- origin generation
- ancestry

---

## 46.3 Product UI

- World
- Organism Inspector
- Species
- Evolution
- History
- telemetry charts
- seed input
- environment controls
- speed controls

---

## 46.4 Infrastructure

- save
- load
- replay
- golden tests
- benchmark
- Windows build
- GitHub Actions

---

# 47. Phase 1 Acceptance Gates

Simulation：

- real inheritance
- real mutation
- real reproduction
- real death
- real predation
- real lineage
- real species
- real extinction

Determinism：

```text
same seed + config + commands
→ same hashes
```

Stability：

```text
100 seeds × 50,000 ticks
```

UI：

- World
- Species
- Evolution
- History

Persistence：

- save
- load
- replay
- autosave

Distribution：

- Windows EXE builds and launches

---

# 48. Phase 1 實際完成基線

Phase 1 最終核心版本：

```text
simulation v4
RNG v1
save v1
```

Phase 1 初始驗證：

- Rust tests：32 PASS
- Frontend tests：8 PASS
- build / lint / typecheck / formatting PASS
- 100 seeds × 50,000 ticks
- 0 failures
- 100 worlds survived
- 9 worlds formed second species

Reference sustained ecology：

```text
50,000 ticks
17.19 sec
≈ 2,908 ticks/sec
final population 200
2 species
```

---

# 49. Phase 1.1 — Productization & Calibration

Phase 1.1 不是 Phase 2。

目的：

> 把 Phase 1 做成真正可發版 baseline。

四件事：

1. Native UAT
2. Ecology Calibration
3. Native File UX
4. GitHub / CI / Release Baseline

---

# 50. Phase 1.1 Native File UX

加入：

- Native Save
- Save As
- Load
- Recent Worlds
- `.genesis`
- dirty state
- Save / Discard / Cancel
- invalid save handling
- corruption handling
- version errors

---

# 51. Phase 1.1 Ecology Calibration

正式：

```text
250 seeds × 100,000 ticks
```

結果：

```text
250/250 survive
0 failures
0 collapse
32/250 natural speciation
12.8%
35 new species
4 worlds with species extinction
```

Classification：

```text
Rich       250
Sterile      0
Healthy      0
Collapse     0
Explosion    0
Oscillating  0
```

重要限制：

Phase 1 hard population cap 使：

```text
post-warmup population CV = 0
```

這是 Phase 2 要改善的地方。

---

# 52. Phase 1.1 Default Config

保留：

```text
seed 42
world size 512
founders 50
population capacity 200
mutation 100
temperature 20°C
regen 12
```

Showcase：

```text
seed 11
```

首次 speciation：

```text
tick 48,600
```

不是 scripted。

---

# 53. Phase 1.1 Final Tests

最終：

- Rust 42 PASS
- Frontend 21 PASS
- fmt PASS
- clippy PASS
- lint PASS
- typecheck PASS
- frontend build PASS
- Tauri build PASS
- native UAT PASS

Determinism：

- simulation v4 unchanged
- RNG v1 unchanged
- golden unchanged
- replay mismatch 0

---

# 54. Phase 1.1 Benchmark

Reference isolated：

```text
50,000 ticks
median ≈ 6.287 sec
≈ 7,953 ticks/sec
```

同樣 world hash。

High density：

```text
5,000 organisms
2,000 ticks
≈ 123.83 TPS
```

Phase 1 benchmark 和 Phase 1.1 isolated timing 不應直接被解讀成程式優化，因為 host/background conditions 不完全一致。

---

# 55. Phase 1.1 Release

建立：

```text
v0.1.0
```

Windows：

- portable EXE
- NSIS installer

CI：

- Linux
- Windows

Remote CI 真實通過後才 tag/release。

---

# 56. Phase 2 — Multicellular Evolution & Ecological Niches

Phase 2 主題：

> 從 scalar-trait organisms 進入 structured morphology 與 ecological niches。

Phase 2 不做：

- LLM
- disease
- parasite
- civilization
- neural networks
- culture
- 3D

---

# 57. Phase 2 Morphology

加入：

- segment count
- body aspect ratio
- appendages
- locomotion
- mouth type
- armor
- sensory investment
- body mass
- storage

Morphology：

- inheritable
- mutable
- costly
- simulation-relevant

---

# 58. Multicellular Transition

禁止：

```text
multicellular = true
```

必須由 morphology 自然形成。

例如：

```text
segment_count > 1
```

但有 trade-off。

Benefit：

- storage
- armor
- bite
- sensory
- thermal inertia

Cost：

- metabolism
- movement
- reproduction
- food need

---

# 59. Feeding Niches

至少：

- Grazer
- Crusher
- Piercer

但不是 hard species enum。

Role 應由 phenotype / behavior 形成。

---

# 60. Armor / Defense

Armor：

- reduces predation damage / success
- increases movement or maintenance cost

沒有 free upgrade。

---

# 61. Locomotion

Derived from：

- mass
- appendages
- shape
- armor

影響：

- speed
- movement cost
- turning

---

# 62. Sensory

Trade-off：

```text
better perception
↔
higher energy cost
```

---

# 63. Dynamic Carrying Capacity

Phase 2 必須弱化 Phase 1 hard cap。

Technical safety cap 保留。

但 normal ecology 應由：

- resources
- metabolism
- reproduction
- predation
- environment

限制 population。

---

# 64. Phase 2 Species Data

Species 加：

- morphology summary
- niche summary
- founder morphology snapshot
- origin environment
- trait delta

Evolution Tree：

顯示 morphology branch changes。

---

# 65. Phase 2 Innovation Events

例如：

- ArmorEmergence
- LargeBodyTransition
- PredatorySpecialization
- SensoryExpansion
- MajorMorphologyShift

必須：

```text
threshold + persistence
```

不是單一 mutation。

---

# 66. Phase 2 Calibration Metrics

新增：

- morphology diversity
- niche diversity
- multicellular incidence
- high-complexity lineage incidence
- predator/prey coexistence
- actual prey-income
- feeding role persistence
- population CV
- resource CV
- gene flow

---

# 67. Phase 2 Acceptance

Phase 2 最終要證明：

- persistent multicellular > 0
- persistent high-C > 0
- >1 persistent morphology cluster in some worlds
- >1 persistent niche cluster in some worlds
- >1 persistent realized feeding role in some worlds
- persistent actual prey-income cohort in some worlds
- no deterministic regression
- native UAT PASS
- final release v0.2.0

---

# 68. Phase 2.x — Ecological Stabilization

如果 Phase 2 morphology 能活，但 niche 不穩定：

進 Phase 2.x。

研究：

- temporal lineage diet
- resource patch coherence
- specialist/generalist fitness
- predator energy
- predator reproduction
- mating/gene flow
- spatial overlap

---

# 69. Temporal Diet Analysis

對 lineage 取 window：

```text
soft_energy_fraction
hard_energy_fraction
prey_energy_fraction
```

Role：

- SoftSpecialist
- HardSpecialist
- PreySpecialist
- Mixed

重點：

判斷 specialization 是否能持續。

---

# 70. Habitat Coherence

量：

- connected patches
- resource autocorrelation
- correlation length
- patch persistence
- spatial overlap

比較：

```text
resource patch scale
vs
movement scale
vs
mating radius
```

---

# 71. Specialist vs Generalist

如果 generalist 太強：

specialization 永遠不會穩定。

比較：

- offspring
- energy surplus
- survival
- mating
- starvation

不要直接強制角色。

---

# 72. Predator Bottleneck

量：

- prey encounters
- hunts
- kills
- prey energy
- movement cost
- attack cost
- reproduction
- compatible mates
- offspring
- starvation

判斷 bottleneck：

- energy
- prey density
- mate scarcity
- reproduction cost

---

# 73. Phase 2.x Rule

每次改 ecology：

先：

```text
diagnose
→ bounded candidate matrix
→ paired seeds
→ reject bad candidates
→ freeze rules
→ full 100×100k
```

禁止盲調。

---

# 74. Phase 3 — Natural History Machine

Phase 3 才加入 AI。

主題：

> 把 deterministic simulation history 轉成可理解的自然史。

---

# 75. AI Naturalist

玩家可以問：

- 為什麼這個 species 變大？
- 為什麼它滅絕？
- 為什麼北方都是 armor？
- 為什麼 predator 消失？

LLM 不能猜。

先取得 structured evidence：

```json
{
  "temperature_change": -11.4,
  "body_mass_change": 0.18,
  "survival_advantage": 0.148,
  "offspring_advantage": 0.097
}
```

LLM 只把它解釋成人話。

---

# 76. Documentary Mode

自動生成：

```text
The Age of Giants
Generation 81,000–123,000
```

內容來源：

- history
- species transitions
- extinction
- ecology
- morphology
- telemetry

---

# 77. Common Names

Scientific name：

deterministic。

Common name：

可以 LLM。

例如：

```text
Velocipodus borealis
Common name: Northern Ember Runner
```

---

# 78. Natural History Report

匯出：

- PDF / Markdown
- species timeline
- evolution tree
- important events
- major innovations
- extinction
- environment shifts

---

# 79. Experiment Comparison

Phase 3：

World A / B comparison。

例如：

```text
A: stable temperature
B: -8°C
```

比較：

- population
- species
- diversity
- morphology
- extinction
- niche

AI 可做 summary，但 raw values 必須 deterministic。

---

# 80. Full Replay / Time Travel

未來可：

- jump generation
- replay historical era
- inspect extinct organisms/species
- compare branch state

---

# 81. Export

最終可輸出：

- PNG Evolution Tree
- species cards
- CSV
- JSON
- GIF
- MP4 timelapse
- natural-history report

---

# 82. Modding（後期）

可設定：

```toml
mutation_rate = ...
world_size = ...
resource_regen = ...
```

但 mod 也要：

- version
- deterministic config
- replay provenance

---

# 83. 測試策略

分層：

## Unit

- genome
- mutation
- phenotype
- metabolism
- resources
- spatial hash
- reproduction
- predation
- species detection
- naming

## Integration

- save/load
- replay
- worker
- Tauri IPC
- command routing
- UI state

## Property

- IDs unique
- parent older than child
- no invalid species ID
- no invalid lineage
- energy valid
- no NaN
- no orphan references

## Golden

fixed seed + commands → fixed hash。

---

# 84. Golden Replay Policy

Simulation algorithm 改變：

必須：

1. old golden fail as expected
2. reason documented
3. simulation/rules version decision
4. new golden reviewed
5. old baseline archived

禁止：

只為了讓測試過而更新 hash。

---

# 85. Stress Testing

每個 Phase 都需要 multi-seed。

Phase 1：

```text
100 × 50,000
```

Phase 1.1：

```text
250 × 100,000
```

Phase 2：

至少：

```text
100 × 100,000
```

---

# 86. Benchmark Levels

Small：

```text
~200–500 organisms
```

Medium：

```text
~1,000–2,000
```

High Density：

```text
~5,000+
```

記：

- ticks/sec
- organism updates/sec
- memory
- peak population
- species
- ecology metrics

---

# 87. Benchmark 原則

不要把：

空世界高 TPS

當持續生態性能。

Benchmark 必須記：

- alive ticks
- average population
- organism updates

Performance regression：

必須 like-for-like comparison。

---

# 88. Native UAT

每次 Release 前：

實際 production EXE。

驗：

- launch
- restart
- New World
- speed
- MAX
- inspect
- Save
- Save As
- Load
- dirty guard
- recent worlds
- continue
- replay
- invalid save
- version errors
- resize
- installer

---

# 89. Release Gate

不能只：

```text
cargo test PASS
```

就 release。

必須：

- local regression
- native UAT
- remote CI
- Windows artifact
- SHA256
- working tree clean

---

# 90. Git Workflow

Main：

永遠代表：

```text
validated baseline
```

Feature branch：

```text
feat/...
phase2/...
```

不要把 incomplete research 直接 merge main。

---

# 91. Commit Strategy

小而有意義。

例如：

```text
feat(sim): add deterministic RNG streams
feat(genome): add inheritance
feat(evolution): add lineage tracking
feat(save): add versioned binary saves
feat(ui): add evolution tree
test(sim): add golden replay
perf(sim): add calibration runner
```

---

# 92. GitHub CI

至少：

Linux：

- fmt
- clippy
- Rust tests
- deterministic checks

Windows：

- frontend format
- lint
- typecheck
- frontend tests
- Rust tests
- Tauri build
- NSIS

Release：

- artifact upload
- checksums

---

# 93. Public Repository Strategy

公開 repo 應呈現：

- validated `main`
- research branches
- screenshots
- architecture
- review guide
- replay evidence
- limitations
- AI collaboration disclosure

不應：

- 隱瞞失敗 candidates
- 把 incomplete Phase 說成完成
- commit secrets
- commit local machine paths
- fake benchmark

---

# 94. Portfolio Positioning

作品集可強調：

### Engineering

- Rust deterministic simulation
- replay
- save versioning
- CI
- Windows packaging
- performance

### CS / Algorithm

- spatial hash
- deterministic event pipeline
- fixed seed
- population simulation
- species clustering
- lineage graph

### AI

- Artificial Life
- Evolution
- future LLM layer with authoritative boundary

### Research discipline

- hypotheses
- calibration
- rejected candidates
- acceptance gates

---

# 95. AI Collaboration Principle

AI 可以協助：

- architecture discussion
- implementation
- testing strategy
- diagnostics
- documentation
- UX iteration

但專案公開說明應清楚標示：

- AI 協作範圍
- 人的設計決策
- 驗證責任
- 不把 AI 產出直接視為真實結果

---

# 96. 最終產品願景

最終 GENESIS 應該讓玩家：

1. 建立一個生命世界
2. 讓生命從簡單形態開始
3. 改變環境
4. 看到自然選擇
5. 看到 morphology 變化
6. 看到 niches
7. 看到 predator/prey
8. 看到 speciation
9. 看到 extinction
10. 查看完整 Evolution Tree
11. replay history
12. fork experiment
13. 問 AI Naturalist「為什麼」
14. export natural history

---

# 97. 最終完成標準

一個真正「完成」的 GENESIS 必須同時具備：

## Simulation

- deterministic
- stable
- replayable
- emergent evolution
- multicellular/morphology
- ecological niches
- extinction
- lineage

## Product

- polished Windows UI
- native file UX
- installer
- crash-safe saves
- history
- charts
- evolution tree

## Analysis

- telemetry
- benchmarks
- calibration
- experiment comparison
- provenance

## AI

- optional
- evidence-grounded
- never authoritative

## Engineering

- CI
- versioned save
- versioned simulation
- golden tests
- release artifacts
- public documentation

---

# 98. 專案 Phase Roadmap

```text
Phase 0
產品定義 / 架構

        ↓

MVP
Deterministic evolution proof

        ↓

Phase 1
First Real Product Slice

        ↓

Phase 1.1
UAT / Calibration / File UX / v0.1.0

        ↓

Phase 2
Multicellular Evolution & Ecological Niches

        ↓

Phase 2.x
Ecological Stabilization / Temporal Diet / Habitat / Predator

        ↓

Phase 3
AI Naturalist / Documentary / Experiment Comparison

        ↓

Final Product
Natural History Machine
```

---

# 99. 各 Phase 不可跨越的 Gate

## MVP → Phase 1

必須證明：

- deterministic
- inheritance
- mutation
- reproduction
- no crash

## Phase 1 → Phase 1.1

必須：

- product UI
- save/replay
- Windows build

## Phase 1.1 → Phase 2

必須：

- native UAT
- calibration
- remote CI
- v0.1.0

## Phase 2 → Phase 3

必須：

- morphology persistence
- niche differentiation
- trophic differentiation
- replay
- native UAT
- v0.2.0

不能因為 Phase 3 比較華麗就跳過。

---

# 100. 專案最重要的三條規則

## Rule 1

**世界真相不由 LLM 決定。**

## Rule 2

**任何生物學敘述都要有 simulation evidence。**

## Rule 3

**每一 Phase 都必須 build、test、benchmark、validate、release 才算完成。**

---

# 101. 最終產品一句話

> GENESIS 是一個 deterministic artificial-life ecosystem，玩家不直接控制生命，而是透過環境壓力觀察 genome、phenotype、competition、reproduction 與 natural selection 如何形成可追溯、可重播、可分析的演化歷史。

---

# 102. 最終開發哲學

不要追求：

- 功能數量
- AI 噱頭
- 花俏動畫
- scripted「進化」
- 看起來像科學但無法驗證的故事

優先順序永遠是：

```text
Causality
> Determinism
> Testability
> Provenance
> Stability
> Performance
> UX
> Narrative
```

最終產品不是：

> AI 幫你講一個演化故事。

而是：

> simulation 真的產生了一段演化史，而 AI 最後只負責幫你理解它。

---

# 103. 專案當前與規劃文件的關係

這份 Master Plan 描述的是整個專案從開始到最終的方向。

實際版本驗證應以各 Phase report 為準，例如：

```text
docs/PHASE1_REPORT.md
docs/PHASE1_1_REPORT.md
docs/PHASE1_1_UAT.md
docs/ECOLOGY_CALIBRATION.md
docs/DETERMINISM.md
docs/SAVE_FORMAT.md
docs/REPLAY.md
docs/ARCHITECTURE.md
docs/PHASE2_REPORT.md
```

Master Plan 不應覆蓋或改寫正式驗收證據。

---

# 104. 結語

GENESIS 最終應該同時具備三種價值：

### 作為作品

打開就能展示。

### 作為工程專案

可測試、可 replay、可 benchmark、可 release。

### 作為 artificial-life 系統

結果不是 scripted，而是真正由 genome、environment、selection 和 reproduction 形成。

只要這三層都成立，GENESIS 就不再是一個「AI 生態 Demo」，而是一個可以持續研究、展示與擴充的完整 deterministic evolution product。
