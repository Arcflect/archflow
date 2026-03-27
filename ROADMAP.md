# Archflow Roadmap

---

## English

### Phase 0: Repository bootstrap

Goal:
Establish a clear open source foundation.

Scope:
- README
- CONTRIBUTING
- issue forms
- labels
- roadmap
- initial examples directory

### Phase 1: Core design model

Goal:
Define the minimum stable concepts of Archflow.

Scope:
- project definition model
- placement rule model
- artifact definition model
- contract definition model
- prompt definition model

Deliverables:
- schema draft
- sample config files
- terminology glossary

### Phase 2: Minimal CLI

Goal:
Provide the first usable command-line flow.

Scope:
- `archflow init`
- `archflow plan`
- `archflow scaffold`

Deliverables:
- config initialization
- structure generation
- artifact sidecar generation

### Phase 3: AI handoff layer

Goal:
Make each artifact directly usable by lightweight coding models.

Scope:
- `archflow prompt`
- prompt templates
- contract-to-prompt conversion

Deliverables:
- artifact prompt generation
- example prompts
- role-based prompt presets

### Phase 4: Verification

Goal:
Check structural and contract consistency.

Scope:
- `archflow verify`
- required contract checks
- path rule checks
- status checks

Deliverables:
- local verification
- CI example workflow

### Phase 5: Presets and ecosystem fit

Goal:
Make Archflow easier to adopt in real projects.

Scope:
- Rust preset
- generic preset
- example repositories
- GitHub workflow examples

### Longer-term directions

- editor integration
- GitHub Action
- import from existing repo structure
- optional lightweight code-aware checks
- multi-language presets

---

## 日本語

### Phase 0: リポジトリ初期整備

目標:
OSS としての土台を整える。

対象:
- README
- CONTRIBUTING
- issue forms
- labels
- roadmap
- examples ディレクトリの初期化

### Phase 1: コア設計モデル

目標:
Archflow の最小概念を安定化する。

対象:
- project definition model
- placement rule model
- artifact definition model
- contract definition model
- prompt definition model

成果物:
- schema draft
- sample config files
- 用語集

### Phase 2: 最小 CLI

目標:
最初の実用的なコマンドフローを作る。

対象:
- `archflow init`
- `archflow plan`
- `archflow scaffold`

成果物:
- config 初期化
- 構造生成
- artifact sidecar 生成

### Phase 3: AI handoff レイヤ

目標:
各 artifact を軽量モデルへ直接渡せる状態にする。

対象:
- `archflow prompt`
- prompt template
- contract から prompt への変換

成果物:
- artifact prompt 生成
- prompt サンプル
- role ごとの prompt preset

### Phase 4: Verify

目標:
構造と contract の整合を検査できるようにする。

対象:
- `archflow verify`
- contract 必須項目チェック
- path rule チェック
- status チェック

成果物:
- ローカル verify
- CI 用サンプル workflow

### Phase 5: Preset と導入しやすさ

目標:
実プロジェクトへ導入しやすくする。

対象:
- Rust preset
- generic preset
- example repository
- GitHub workflow examples

### 長期的な方向性

- エディタ統合
- GitHub Action
- 既存 repo からのルール逆生成
- optional な軽量コードチェック
- 多言語 preset
