# Layer Responsibilities: Batonel Module Placement Guide

> **Audience**: All contributors (human and AI).  
> **Purpose**: Make placement decisions predictable. When you are unsure where a new
> type, function, or file belongs, start here.

---

## Quick-Reference Table

| Layer | Main question it answers | Allowed to depend on |
|---|---|---|
| `cli/` | How does the user invoke this from a terminal? | `app`, `infra` (for rendering), `cli::runner` |
| `app/` | In what order do we carry out this workflow? | `domain`, `ports` |
| `domain/` | What are the rules and what decision do we make? | nothing outside `domain/` |
| `ports/` | What capability boundary do we need? | nothing — traits only |
| `infra/` | How do we actually read, write, or call the outside world? | `ports`, `domain` (data shapes only) |
| `config/` | How do we load and validate raw YAML/JSON config files? | `model/` data shapes, `serde`, `std::fs` |
| `model/` | What are the shared data shapes and validation rules? | nothing outside `model/` |
| `generator/` | How do we resolve paths and write scaffolded files? | `model/`, `config/`, `std::fs` |
| `commands/` | *(Transitional)* Legacy command executors. Shrink; do not grow. | anything — but this is the debt |

---

## Layer Responsibilities

### `cli/` — Terminal Adapter

**Owns:**
- CLI argument parsing (`clap` types, `#[command]`, `#[arg]`)
- Input → Input-DTO translation
- UseCase invocation via `cli::runner` helpers
- Rendering usecase output to the terminal

**Does not own:**
- Business rules of any kind
- File I/O (except bootstrapping `--config` path resolution)
- Direct calls to `infra` adapters when a usecase exists

**Where new code goes:**
- New `--flag` or subcommand → `cli/mod.rs`
- New rendering for a usecase output → add a handler arm in `cli/commands/mod.rs`
- Shared execution plumbing (error exit, banner) → `cli/runner.rs`

---

### `app/` — Application Orchestration

**Owns:**
- UseCase structs (`*UseCase::execute`)
- Input / Output DTO types (`*Input`, `*Output`)
- Workflow sequencing (call domain A, then domain B, then write via port C)
- Application-level error types (`AppError`, `ConfigLoadError`, …)

**Does not own:**
- Business rules (belongs in `domain/`)
- Raw file I/O (belongs in `infra/` behind a port)
- Terminal presentation (belongs in `cli/` or `infra/rendering/`)

**Where new code goes:**
- New user-facing workflow → new file in `app/usecase/`
- New application error variant → `app/error.rs`

---

### `domain/` — Business Logic Core

**Owns:**
- Core concepts and models (`ProjectContext`, `ArchitecturePlan`, `GenerationPlan`, …)
- Rule evaluation (`ArchitectureValidator`, `ArchitecturePlanner`, …)
- Invariant checks
- Preset resolution logic
- All logic that answers "what is the correct decision?"

**Does not own:**
- File reading or writing (`std::fs`)
- Shell / process execution
- HTTP / network calls
- Terminal formatting or ANSI color
- `clap`, `tokio`, `reqwest`, or any I/O framework

**Test rule:** Every `domain/` function must be testable without touching the
filesystem, network, or shell.

**Where new code goes:**
- New business rule or decision → extend the relevant `domain/<concept>/` module
- New cross-cutting domain concept → new subdirectory under `domain/`

---

### `ports/` — Capability Boundaries

**Owns:**
- Rust `trait` definitions for external capabilities
- `OutputPort`, `FilesystemPort`, `GitPort`, `TemplatePort`, `LlmPort`

**Does not own:**
- Any implementation (that lives in `infra/`)
- Business rules

**Design rule:** Keep each port focused on one capability. Avoid "god ports".

**Where new code goes:**
- New external capability needed by `app/` → new trait in `ports/<capability>.rs`

---

### `infra/` — Concrete Adapters

**Owns:**
- Implementations of `ports/` traits
- `ConsoleOutputAdapter`, `LocalFilesystemAdapter`, `PlanRendererAdapter`, etc.
- Third-party error → internal error conversion

**Does not own:**
- Business decisions
- Orchestration logic

**Where new code goes:**
- New implementation of an existing port → add to the matching `infra/` file
- New rendering format → `infra/rendering/`

---

### `config/` — Raw Configuration Loading *(Transitional)*

**Owns:**
- YAML/JSON deserialization structs (`ProjectConfig`, `PlacementRulesConfig`, …)
- Schema validation at parse time (`SUPPORTED_PROJECT_SCHEMA_VERSION`, duplicate checks)
- Override policy resolution (`config/override_policy.rs`)
- RBAC policy loading (`config/rbac.rs`)

**Does not own:**
- Business decisions about the loaded data (belongs in `domain/` or `app/`)
- Terminal output

**Boundary rule:** Config structs may be passed into `domain/` functions, but raw
config parsing **must not** happen inside domain or app. The calling use case is
responsible for loading config first, then handing the result to domain.

**Migration direction:** As modules mature, the "semantic" meaning of each config
field should migrate to domain types; only the raw deserialization shell should remain
in `config/`.

---

### `model/` — Shared Data Shapes and Validation *(Transitional)*

**Currently owns:**
- Lightweight value types: `Artifact`, `Contract`, `Placement`, `Project`
- Validation logic tightly coupled to these shapes: `contract_validation`,
  `prompt_validation`, `scaffold_validation`, `status_validation`
- The `verify` report model (`VerifyReport`, `CheckResult`, `VerifyStatus`)

**Ambiguity note:** `model/` sits between `config/` (raw) and `domain/` (decisions).
The value types here are candidates for migration into `domain/` sub-modules as the
relevant domain areas mature.

**Rule for new code:** Do not add new *business decision logic* to `model/`. If you
need to validate or evaluate something, ask: does it belong in a named `domain/<area>`
module instead?

---

### `generator/` — Scaffold File Generation *(Transitional)*

**Currently owns:**
- Artifact path resolution (`generator/resolver.rs`)
- File template rendering and writing (`generator/scaffold.rs`)

**Ambiguity note:** Path resolution is a domain concern (rules about where files go),
while file writing is an infra concern (actually touching the filesystem). These two
responsibilities should eventually separate into `domain/generation/` (rules) and
`infra/` (writes).

**Rule for new code:** Do not add new business rules here. Path resolution rules
belong in `domain/generation/`; file writing belongs behind a `FilesystemPort`.

---

### `commands/` — Legacy CLI Executors *(Transitional)*

**Status:** Being phased out. `commands/init.rs` and `commands/scaffold.rs` are
stubs. `commands/verify.rs` is a rendering helper only.

**Rule:** Do not add new orchestration here. Any new command must be backed by a
`app/usecase/` module. Legacy executors that remain (`audit`, `fix`, `prompt`,
`triage`, `guard`, `compliance_report`, `fix_rollout`, `policy_resolve`,
`preset_migrate`, `preset_registry`, `preset_verify`) should shrink over time.

---

## Placement Decision Guide

Use this flowchart when you are unsure where to put new code.

```
Is this about parsing or loading a YAML/JSON file?
  └─ YES → config/

Is this a business rule, validation, or planning decision?
  └─ YES → domain/<concept>/

Is this orchestrating multiple domain calls and ports together?
  └─ YES → app/usecase/

Is this defining what an external capability looks like (a trait)?
  └─ YES → ports/

Is this implementing how we actually read/write/call something?
  └─ YES → infra/

Is this about parsing CLI arguments, rendering output, or mapping
CLI input to a usecase?
  └─ YES → cli/

Is this a shared, stable, cross-cutting primitive with no better home?
  └─ YES → shared/ (create if it does not exist; keep it tiny)
```

---

## Common Change Types

### "I need to add a new CLI flag"
→ Add to `cli/mod.rs`. Map in `cli/commands/mod.rs`. Keep the handler arm thin.

### "I need a new validation rule"
→ Add to the relevant `domain/validation/` module. Write a unit test.

### "I need to load a new config file"
→ Add a deserialization struct in `config/`. Load it in the relevant `app/usecase/`.

### "I need to write files to disk"
→ Use `std::fs` in `generator/scaffold.rs` for now. Longer term, add a
`FilesystemPort` method and implement it in `infra/`.

### "I need a new rendering format"
→ Add a method to `infra/rendering/`. Call it from the command arm in
`cli/commands/mod.rs`.

### "I need to add a new command end-to-end"
1. Define Input/Output structs in `app/usecase/<name>.rs`.
2. Implement `<Name>UseCase::execute`.
3. Add the command variant in `cli/mod.rs`.
4. Wire the arm in `cli/commands/mod.rs` using `runner::run_usecase`.

---

## Known Ambiguous / Leaky Areas

| Area | Issue | Recommended direction |
|---|---|---|
| `model/` validation modules | Mix of data shapes and rule logic | Migrate decision logic to `domain/` sub-modules |
| `generator/resolver.rs` | Path resolution rules live outside `domain/` | Move rule logic to `domain/generation/` |
| `config/` loaded into `app/usecase/` directly | No port abstraction for config loading | Acceptable intermediate state; do not leak config types further |
| `commands/verify.rs` rendering helpers | Rendering in `commands/` rather than `infra/` | Move to `infra/rendering/` as a `VerifyRendererAdapter` |
| `commands/audit.rs` etc. | Full orchestration in `commands/` | Back with `app/usecase/` one at a time |

---

## Boundary Violation Checklist (for Code Review)

- [ ] Does `domain/` import `clap`, `std::fs`, `reqwest`, or any shell-execution crate?
- [ ] Does `domain/` call `std::process::exit`?
- [ ] Does `cli/` contain branching logic that makes a business decision?
- [ ] Does `infra/` define a business rule or domain invariant?
- [ ] Does `app/` call `println!` or `eprintln!` directly?
- [ ] Does `ports/` contain any `struct` implementation (not just `trait`)?
- [ ] Was new code placed in `commands/`, `model/`, or `generator/` without a documented reason?

If any of these is **yes**, the PR should explain why or offer a remediation plan.
