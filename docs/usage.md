# Archflow Prompt Generation: Usage Examples

This document provides typical commands to initialize and verify prompt generation across the different example architectures provided in this repository.

## Prerequisites
Ensure the binary is built and available. You can run it via `cargo run` from the project root.

---

## Preset Bootstrap: `archflow init`

Use `archflow init` as the minimal startup command.

Default initialization (no preset):

```bash
cargo run -- init
```

Preset-based initialization:

```bash
# Generic layered starter
cargo run -- init --preset generic-layered

# Rust clean/hexagonal starter
cargo run -- init --preset rust-clean-hexagonal

# Dry-run preview (no files are written)
cargo run -- init --preset generic-layered --project-name my-app --dry-run
```

Optional immediate override:

```bash
cargo run -- init --preset generic-layered --project-name my-app
```

Generated files in current directory:

- `project.arch.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- `artifacts.plan.yaml` (when included by the chosen preset)
- `policy.profile.yaml`
- `guard.sidecar.yaml`

Behavior notes:

- existing files are skipped (not overwritten)
- unknown preset id fails with an error and available preset ids
- empty/whitespace `--project-name` fails with an explicit error
- deeper customization is done by editing generated files after init

Onboarding e2e check for core presets:

```bash
bash scripts/onboarding_e2e_init_plan.sh --preset generic-layered --project-name e2e-generic-service
bash scripts/onboarding_e2e_init_plan.sh --preset rust-clean-hexagonal --project-name e2e-rust-service
```

The check validates:

- `init --dry-run` does not write files
- `init` generates the expected root config files
- `plan` output is stable across identical repeated runs

---

日本語メモ（Onboarding e2e）:

- `scripts/onboarding_e2e_init_plan.sh` は、core preset に対する `init -> plan` の最小e2e検証です。
- `--dry-run` の非破壊性、初期ファイル生成、`plan` 出力の再現性を確認します。

---

## 1. Minimal Example
A flat architecture with simple domain and application layers.

```bash
# Navigate to the archflow configuration directory
cd examples/minimal/archflow

# Step A: Generate missing contract and source placeholders
cargo run --manifest-path ../../../Cargo.toml -- scaffold

# Step B: Generate a prompt for an Entity
cargo run --manifest-path ../../../Cargo.toml -- prompt user

# Step C: Generate a prompt for a Usecase
cargo run --manifest-path ../../../Cargo.toml -- prompt create_user
```

## 2. Generic Layered Example
A traditional N-tier layered architecture.

```bash
cd examples/generic-layered/archflow

# Generate sidecars
cargo run --manifest-path ../../../Cargo.toml -- scaffold

# Generate a prompt for an HTTP Controller (Handler)
cargo run --manifest-path ../../../Cargo.toml -- prompt create_user_controller

# Generate a prompt for a Repository Port
cargo run --manifest-path ../../../Cargo.toml -- prompt user_repository
```

## 3. Rust Clean Hexagonal Example
A sophisticated Hexagonal (Ports & Adapters) architecture with crate isolation.

```bash
cd examples/rust-clean-hexagonal/archflow

# Generate sidecars
cargo run --manifest-path ../../../Cargo.toml -- scaffold

# Generate a prompt for a Port Implementation (Infrastructure)
cargo run --manifest-path ../../../Cargo.toml -- prompt postgres_user_repository

# Generate a prompt for an HTTP Handler Adapter
cargo run --manifest-path ../../../Cargo.toml -- prompt create_user_handler
```

---

## Output Options

### Compact Mode
Optimized for smaller LLM context windows or lightweight models, stripping metadata headers and list spacing.
```bash
cargo run --manifest-path [PATH_TO_CARGO_TOML] -- prompt [ARTIFACT] --mode compact
```

### Standard Mode (Default)
Human-readable Markdown with clear headers and full context.
```bash
cargo run --manifest-path [PATH_TO_CARGO_TOML] -- prompt [ARTIFACT] --mode standard
```

---

## Minimal CI Example: `archflow verify`

Use the workflow file below as a minimal GitHub Actions example:

- `.github/workflows/archflow-verify-example.yml`

This example runs `archflow verify` for each bundled example fixture:

- `examples/minimal/archflow`
- `examples/generic-layered/archflow`
- `examples/rust-clean-hexagonal/archflow`

Each matrix run also uploads the execution log as a workflow artifact:

- `archflow-verify-log-examples-minimal-archflow`
- `archflow-verify-log-examples-generic-layered-archflow`
- `archflow-verify-log-examples-rust-clean-hexagonal-archflow`

Core command pattern used in CI:

```bash
cd examples/minimal/archflow
cargo run --manifest-path ../../../Cargo.toml -- verify
```

Expected behavior:

- exit code `0`: verification succeeded (with or without warnings)
- exit code `1`: verification failed (at least one `Fail` check)

This is intentionally minimal and demonstrates the automation path without
introducing a full CI platform design.

---

## PR Gate Example: `archflow audit --strict`

Use the workflow file below as a baseline PR gate setup:

- `.github/workflows/archflow-audit-pr-gate.yml`

This example runs `archflow audit --strict` for each bundled example fixture:

- `examples/minimal/archflow`
- `examples/generic-layered/archflow`
- `examples/rust-clean-hexagonal/archflow`

Audit output includes rule-level diagnostics:

- `rule_id`
- `severity` (`error` or `warn`)
- remediation hints

Policy baseline behavior:

- loads `policy.profile.yaml` when present
- falls back to built-in minimum policy profile when missing
- supports project-level exceptions via explicit `overrides`

Core command pattern used in CI:

```bash
cd examples/minimal/archflow
cargo run --manifest-path ../../../Cargo.toml -- audit --strict
```

Expected behavior:

- exit code `0`: no errors and no warnings
- exit code `1`: at least one error, or warnings when `--strict` is enabled

---

## Conservative Remediation: `archflow fix`

`archflow fix` introduces conservative automation boundaries.

Policy in this phase:

- auto-fix only low-risk structural gaps (currently: missing root config files)
- keep semantic or architectural decisions as review-required
- include patch previews for review-required findings

Dry-run preview (recommended default workflow):

```bash
cargo run -- fix --dry-run
```

Limited apply mode (low-risk fixes only):

```bash
cargo run -- fix --apply
```

Current classification examples:

- auto-fixable:
	- missing `project.arch.yaml` / `placement.rules.yaml` / `artifacts.plan.yaml` / `contracts.template.yaml`
- review-required:
	- artifact references undefined module
	- artifact uses undefined role
	- explicit artifact path deviates from role default path

`--apply` never auto-applies review-required findings. Instead it emits patch-oriented
previews and exits non-zero so human review remains mandatory.

---

## Preset Registry Prototype: `preset-publish` / `preset-install`

Archflow provides a prototype local registry workflow for preset sharing.

### Publish a preset package

```bash
cargo run -- preset-publish --preset-dir presets/generic-layered --registry-dir .archflow/registry
```

Publish validation checks:

- `preset.yaml` must parse and `name` must match directory name
- `package.version` and compatibility versions must be semver (`x.y.z`)
- `package.visibility` must be `public` or `private`
- required includes must contain and resolve to existing files
- duplicate `(id, version)` entries are rejected

### Install a preset package

```bash
# latest compatible version
cargo run -- preset-install --preset generic-layered --registry-dir .archflow/registry --destination-dir presets

# explicit version
cargo run -- preset-install --preset generic-layered --preset-version 0.1.0 --registry-dir .archflow/registry --destination-dir presets
```

Install validation checks:

- preset id/version must exist in registry index
- compatibility range must include current Archflow version
- project/policy schema compatibility must match current runtime support
- destination preset directory must not already exist

Registry index format:

- registry root: `<registry-dir>`
- index file: `<registry-dir>/index.yaml`
- package files: `<registry-dir>/packages/<preset-id>/<version>/...`

---

## Sidecar Guard Checks: `archflow guard`

Guard checks provide sidecar-first policy enforcement with hook points for:

- `init`
- `plan`
- `ci`

Run guard explicitly:

```bash
# CI-oriented enforcement (warnings fail in strict mode)
cargo run -- guard --hook ci --strict

# Local checks aligned with plan hook
cargo run -- guard --hook plan
```

Diagnostics format is aligned with `audit` output:

- `rule_id`
- `severity` (`error` or `warn`)
- `target`
- remediation hint

Fallback behavior when guard rules are unavailable:

- if `guard.sidecar.yaml` is missing, guard emits `guard-rules-unavailable` as a warning
- fallback defaults are applied automatically to keep checks running
- if `guard.sidecar.yaml` exists but is invalid, guard still falls back and reports the warning

---

## Preset-Based Workflow Examples

For small workflow examples aimed at preset-based repositories, see:

- `examples/preset-repository-patterns/workflows/README.md`
- `examples/preset-repository-patterns/workflows/verify-preset-project.yml`
- `examples/preset-repository-patterns/workflows/plan-scaffold-prompt-preview.yml`

These files illustrate where `plan`, `scaffold`, `prompt`, and `verify` can fit
in normal repository automation without introducing a large CI framework.
