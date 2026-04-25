# Before & After: What Batonel Produces

Batonel converts high-level architectural intentions into an executable project skeleton complete with strictly enforced sidecar contracts. 

To understand Batonel's core value, here is a visual breakdown of a project **before** and **after** running `batonel scaffold`.

---

## 1. Before: The Architectural Intent

The starting point is a single `project.baton.yaml` configuration. This file defines the architecture style, workspace shape, and the distinct modules/features to be built.

**`project.baton.yaml`**
```yaml
batonel:
  schema_version: "1"
  preset:
    id: rust-clean-hexagonal

project:
  name: rust-clean-hexagonal-app
  architecture_style: clean-hexagonal
  language: rust

workspace:
  enabled: true
  members:
    - crates/domain
    - crates/application
    - crates/adapters/http
    - crates/adapters/db

modules:
  - name: user
    features:
      - create_user
      - user_entity
      - user_repository_port
      - create_user_handler
      - postgres_user_repository
```

At this stage, you only have intent. There is no code, no directory structure, and no boundary enforcement.

---

## 2. The Execution

You run the primary generation command:

```bash
batonel scaffold
```

Batonel reads the preset and projects the intent onto the filesystem. 

---

## 3. After: The Executable Architecture

The output is much more than empty folders. Batonel generates three crucial layers of value: **Structure**, **Contracts**, and **Prompts**.

### Value Signal 1: Predictable Structure

The physical workspace is generated precisely according to the architecture style (in this case, Hexagonal Architecture).

```text
.
├── .batonel/
│   ├── contracts/             # Generated sidecar contracts per artifact
│   └── prompts/               # Generated AI handoff prompts
├── crates/
│   ├── adapters/
│   │   ├── db/src/repositories/postgres_user_repository.rs
│   │   └── http/src/handlers/create_user_handler.rs
│   ├── application/
│   │   ├── src/ports/outbound/user_repository.rs
│   │   └── src/usecases/create_user.rs
│   └── domain/
│       └── src/entities/user.rs
└── project.baton.yaml
```

Every piece of the `user` module has been placed in exactly the right directory according to its architectural role.

### Value Signal 2: Explicit Contracts

For every generated source file, Batonel produces a sidecar `.contract.yaml` file. This contract dictates the rules of engagement for that specific artifact. 

Here is the contract generated for the `user` entity:

**`.batonel/contracts/user.contract.yaml`**
```yaml
name: user
module: user
role: entity
path: crates/domain/src/entities/user.rs

responsibilities:
  - "Represent a core business concept"
  - "Protect domain invariants"
  - "Model the user entity"

must_not:
  - "Depend on application or adapter concerns"
  - "Contain HTTP or database-specific logic"
  - "Contain persistence access"

allowed_dependencies:
  - "crates/domain"

forbidden_dependencies:
  - "crates/application"
  - "crates/adapters/http"
  - "crates/adapters/db"

outputs:
  - "User"
```

> [!NOTE] 
> Because this rule is codified, running `batonel verify` in the future will automatically catch if someone accidentally imports `crates/adapters/db` into the domain layer.

### Value Signal 3: AI Handoff Prompts

Batonel automatically translates the strict YAML contracts into human- and AI-readable Markdown prompts. When delegating the implementation of `user.rs` to an LLM or a junior developer, you simply hand them this generated prompt:

**`.batonel/prompts/user.prompt.md`**
```markdown
# Artifact Prompt: user

Implement the `user` artifact.

## Role
entity

## Responsibilities
- Represent a core business concept
- Protect domain invariants
- Model the user entity

## Must not
- Depend on application or adapter concerns
- Contain HTTP or database-specific logic
- Contain persistence access

## Allowed dependencies
- crates/domain

## Forbidden dependencies
- crates/application
- crates/adapters/http
- crates/adapters/db

## Completion criteria
- The entity strictly protects its domain invariants.
- Methods represent business rules, not just generic getters/setters.
- No application, transport, or persistence details leak into this layer.
```

---

## Summary of Value

By generating a Batonel project, you get:

1. **Preset Fit**: Zero-configuration alignment with proven architectural patterns.
2. **Structure**: Directories and files placed exactly where they belong.
3. **Contracts**: Enforceable boundaries living right next to the code.
4. **Prompts**: Instant, high-quality context to feed into AI coding assistants (like Copilot or Cursor).
5. **Verification**: A continuous capability to run `batonel verify` to prove your code still matches your architecture.

---

## 日本語

# Before & After: Batonel の生成物

Batonel は、高レベルなアーキテクチャの意図を、厳密に強制されたサイドカー契約（contract）を伴う実行可能なプロジェクトの骨格（skeleton）へと変換します。

Batonel のコアバリューを理解するために、`batonel scaffold` を実行する **Before（前）** と **After（後）** のプロジェクトを視覚的に分解してみましょう。

---

## 1. Before: アーキテクチャの意図

出発点は単一の `project.baton.yaml` 設定ファイルです。このファイルでは、アーキテクチャスタイル、ワークスペースの構成、および構築する個別のモジュール/機能（features）を定義します。

**`project.baton.yaml`**
```yaml
batonel:
  schema_version: "1"
  preset:
    id: rust-clean-hexagonal

project:
  name: rust-clean-hexagonal-app
  architecture_style: clean-hexagonal
  language: rust

workspace:
  enabled: true
  members:
    - crates/domain
    - crates/application
    - crates/adapters/http
    - crates/adapters/db

modules:
  - name: user
    features:
      - create_user
      - user_entity
      - user_repository_port
      - create_user_handler
      - postgres_user_repository
```

この段階では「意図」しか存在しません。コードも、ディレクトリ構造も、境界の強制もまだありません。

---

## 2. 実行 (The Execution)

主要な生成コマンドを実行します：

```bash
batonel scaffold
```

Batonel はプリセットを読み込み、意図をファイルシステム上に投影します。

---

## 3. After: 実行可能なアーキテクチャ

出力結果は単なる空のフォルダではありません。Batonel は、**構造 (Structure)**、**契約 (Contracts)**、**プロンプト (Prompts)** という 3 つの重要な価値層を生成します。

### 価値 1: 予測可能な構造

物理的なワークスペースは、アーキテクチャスタイル（この場合はクリーンアーキテクチャ/ヘキサゴナルアーキテクチャ）に正確に従って生成されます。

```text
.
├── .batonel/
│   ├── contracts/             # artifact ごとに生成されたサイドカー契約
│   └── prompts/               # AI への引き継ぎ用（handoff）プロンプト
├── crates/
│   ├── adapters/
│   │   ├── db/src/repositories/postgres_user_repository.rs
│   │   └── http/src/handlers/create_user_handler.rs
│   ├── application/
│   │   ├── src/ports/outbound/user_repository.rs
│   │   └── src/usecases/create_user.rs
│   └── domain/
│       └── src/entities/user.rs
└── project.baton.yaml
```

`user` モジュールのすべての要素が、そのアーキテクチャ上の役割に応じて正確なディレクトリに配置されます。

### 価値 2: 明示的な契約 (Explicit Contracts)

生成されたソースファイルごとに、Batonel はサイドカーである `.contract.yaml` ファイルを生成します。この契約は、その特定の artifact に対するルールを規定します。

以下は `user` エンティティに対して生成された契約の例です：

**`.batonel/contracts/user.contract.yaml`**
```yaml
name: user
module: user
role: entity
path: crates/domain/src/entities/user.rs

responsibilities:
  - "Represent a core business concept"
  - "Protect domain invariants"
  - "Model the user entity"

must_not:
  - "Depend on application or adapter concerns"
  - "Contain HTTP or database-specific logic"
  - "Contain persistence access"

allowed_dependencies:
  - "crates/domain"

forbidden_dependencies:
  - "crates/application"
  - "crates/adapters/http"
  - "crates/adapters/db"

outputs:
  - "User"
```

> [!NOTE] 
> このルールはコード化されているため、将来 `batonel verify` を実行した際、誰かが誤って `crates/adapters/db` を domain レイヤーにインポートしていれば、自動的に検知されます。

### 価値 3: AI 引き継ぎプロンプト (AI Handoff Prompts)

Batonel は厳密な YAML の契約を、人間と AI の両方が読みやすい Markdown プロンプトに自動翻訳します。`user.rs` の実装を LLM や他の開発者に委譲する際は、単にこの生成されたプロンプトを渡すだけです：

**`.batonel/prompts/user.prompt.md`**
```markdown
# Artifact Prompt: user

Implement the `user` artifact.

## Role
entity

## Responsibilities
- Represent a core business concept
- Protect domain invariants
- Model the user entity

## Must not
- Depend on application or adapter concerns
- Contain HTTP or database-specific logic
- Contain persistence access

## Allowed dependencies
- crates/domain

## Forbidden dependencies
- crates/application
- crates/adapters/http
- crates/adapters/db

## Completion criteria
- The entity strictly protects its domain invariants.
- Methods represent business rules, not just generic getters/setters.
- No application, transport, or persistence details leak into this layer.
```

---

## 価値のまとめ (Summary of Value)

Batonel プロジェクトを生成することで、以下のメリットが得られます：

1. **プリセットの適合 (Preset Fit)**: 実績のあるアーキテクチャパターンに、設定なしで適合します。
2. **構造 (Structure)**: ディレクトリとファイルが、本来あるべき場所に正確に配置されます。
3. **契約 (Contracts)**: コードのすぐ横に、強制可能な境界ルールが存在します。
4. **プロンプト (Prompts)**: AI コーディングアシスタント（Copilot や Cursor など）に入力するための、即座に利用可能な高品質のコンテキストが得られます。
5. **検証 (Verification)**: `batonel verify` を実行することで、コードがアーキテクチャに一致しているかを継続的に証明する能力が得られます。
