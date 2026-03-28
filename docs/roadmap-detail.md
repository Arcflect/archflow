# Roadmap Detail

This document expands the high-level roadmap into a more practical working plan.

Its purpose is to make each phase easier to execute by clarifying:

- the goal of the phase
- the main work items
- the expected outputs
- the definition of done
- what should not be overbuilt too early

This document complements `ROADMAP.md`.
`ROADMAP.md` stays concise.
This file adds more operational detail.

---

## Overview

Archflow is being developed in phases.

The overall flow is:

- Phase 0: repository bootstrap
- Phase 1: core design model
- Phase 2: minimal CLI
- Phase 3: AI handoff layer
- Phase 4: verification
- Phase 5: presets and ecosystem fit

The purpose of this phased approach is to keep the project focused.

Archflow should not start by trying to solve everything at once.
It should establish concepts first, then operationalize them gradually.

---

## Phase 0: Repository bootstrap

### Goal

Establish a clear open source foundation and make the project understandable before implementation grows.

### Why this phase matters

If the repository is unclear, future implementation will become harder to navigate.
Phase 0 creates the minimum structure needed for contributors and future users to understand what Archflow is trying to do.

### Main work items

- README
- CONTRIBUTING
- CODE_OF_CONDUCT
- LICENSE
- SECURITY
- issue forms
- labels
- roadmap
- examples directory bootstrap
- basic documentation structure

### Expected outputs

- root repository documentation is in place
- examples directory exists and is understandable
- core project positioning is documented
- contribution entry points are clear
- community health files exist

### Definition of done

Phase 0 is done when:

- a new visitor can understand what Archflow is
- a contributor can find how to participate
- examples exist for the main conceptual directions
- the repository structure no longer feels empty or ambiguous
- the initial roadmap and supporting docs exist

### What not to overbuild

Do not overbuild:

- plugin integration
- code-aware analysis
- full CLI behavior
- preset engine
- advanced CI

The focus here is repository clarity, not feature completeness.

---

## Phase 1: Core design model

### Goal

Define the minimum stable conceptual model of Archflow.

### Why this phase matters

Without a stable concept model, implementation will drift.
Phase 1 gives Archflow its vocabulary and internal architecture.

### Main work items

- define core concepts
- define glossary
- define schema drafts
- define schema guide
- define architecture flow
- define preset direction
- align examples with the concept model

### Core concepts to stabilize

The main concepts to stabilize are:

- project
- module
- role
- artifact
- placement rule
- contract
- prompt
- scaffold
- verify
- preset

### Expected outputs

- concept documents exist
- glossary exists
- schema drafts exist
- schema guide exists
- architecture flow document exists
- preset concept is documented
- examples are aligned with the terminology

### Definition of done

Phase 1 is done when:

- the core concepts no longer conflict with each other
- the repository has a stable shared vocabulary
- input and output file types are documented
- contributors can explain the model consistently
- future implementation work has a clear conceptual base

### What not to overbuild

Do not overbuild:

- strict formal validation
- deep schema enforcement
- complete preset machinery
- detailed runtime behavior
- all possible role types

The goal is conceptual stability, not maximal completeness.

---

## Phase 2: Minimal CLI

### Goal

Provide the first usable command-line flow.

### Why this phase matters

At this stage, Archflow should move from concept documentation to an actual operational tool.

The first CLI should be small but real.

### Main work items

- `archflow init`
- `archflow plan`
- `archflow scaffold`

### Recommended implementation order

1. parse project definition
2. parse placement rules
3. parse artifact plan
4. resolve paths
5. generate basic scaffold structure
6. optionally generate initial sidecar files

### Expected outputs

- CLI crate exists
- configuration loading works
- path resolution works
- scaffold generation works for documented examples
- example-based manual validation is possible

### Definition of done

Phase 2 is done when:

- users can initialize or prepare a project structure
- users can generate planned output from structured input
- examples can be mapped to real CLI behavior
- the core CLI flow is demonstrable end-to-end

### What not to overbuild

Do not overbuild:

- perfect UX
- many subcommands
- editor integration
- advanced validation
- every configuration edge case

The first CLI should prove the flow, not solve every future use case.

---

## Phase 3: AI handoff layer

### Goal

Make each artifact directly usable by lightweight coding models.

### Why this phase matters

This is where Archflow becomes clearly different from a generic scaffold tool.

The goal is not only to create files.
It is to create artifact-level implementation handoff.

### Main work items

- `archflow prompt`
- prompt generation from contract data
- role-based prompt defaults
- prompt output modes
- artifact-level completion criteria

### Expected outputs

- prompts can be generated from contracts
- prompt structure is consistent across examples
- lightweight AI-oriented usage becomes demonstrable
- prompt derivation is clearly tied to the contract model

### Definition of done

Phase 3 is done when:

- one artifact can be handed to an AI model with explicit constraints
- prompt generation is stable for the main example roles
- prompts are derived from contract information, not handwritten ad hoc
- the value of Archflow for AI-assisted development is visible

### What not to overbuild

Do not overbuild:

- model-specific integrations
- vendor-specific APIs
- agent protocol support
- overly complex prompt personalization
- automatic implementation generation inside Archflow itself

The focus is handoff quality, not model orchestration.

---

## Phase 4: Verification

### Goal

Check whether project structure and artifact definitions remain consistent over time.

### Why this phase matters

Without verification, contracts and prompts may drift away from actual structure.
Phase 4 protects the architectural memory of the project.

### Main work items

- `archflow verify`
- required contract checks
- placement consistency checks
- status checks
- scaffold consistency checks
- future CI example

### Verification scope for the first version

Start with checks such as:

- required files exist
- required fields exist
- role names align across files
- artifact paths match placement rules
- contract and prompt files are present for expected artifacts

### Expected outputs

- local verification command works
- verification output is understandable
- examples can be checked using the same rules
- CI usage becomes possible

### Definition of done

Phase 4 is done when:

- users can detect structural drift
- users can detect missing contracts or prompt files
- role/path mismatches are surfaced clearly
- verification can be demonstrated locally and in a basic CI example

### What not to overbuild

Do not overbuild:

- full static code analysis
- compiler integration
- deep dependency graph inspection
- advanced policy DSL
- heavy runtime coupling to one language

The first verify phase should focus on structure and contract consistency.

---

## Phase 5: Presets and ecosystem fit

### Goal

Make Archflow easier to adopt in real projects and more reusable across styles.

### Why this phase matters

Once the core flow exists, users will want faster starting points.
Presets and ecosystem-aware defaults reduce friction.

### Main work items

- define preset packaging approach
- formalize current example-to-preset evolution
- create Rust preset
- create generic preset
- add example repository patterns
- add GitHub workflow examples
- improve onboarding for new projects

### Expected outputs

- at least one reusable preset exists
- example structures map clearly to preset concepts
- project bootstrap becomes faster
- ecosystem-specific conventions become easier to apply

### Definition of done

Phase 5 is done when:

- users can start from a preset instead of defining everything manually
- examples and presets have a clear relationship
- at least one language-specific and one language-agnostic path exist
- Archflow feels easier to adopt in realistic projects

### What not to overbuild

Do not overbuild:

- too many presets too early
- deep per-framework specialization
- overly rigid preset locking
- automatic migration of all existing repositories
- complete plugin ecosystem

The focus is useful starting points, not maximum coverage.

---

## Cross-phase principles

Some principles apply across all phases.

### Keep concepts ahead of implementation

Implementation should follow stable concepts, not invent them on the fly.

### Prefer explicitness over magic

Archflow should be understandable by reading files, not only by running code.

### Keep the artifact as the main execution unit

The artifact is the center of planning, contracts, prompts, and future verification.

### Preserve architecture outside source code

Important architectural intent should not exist only in production code.

### Avoid premature ecosystem lock-in

Archflow can begin with Rust-friendly examples without becoming Rust-only.

---

## Suggested practical milestone order

A practical internal milestone sequence may look like this:

1. bootstrap repository and examples
2. stabilize concepts and glossary
3. stabilize schema drafts
4. document architecture flow and presets
5. implement minimal config loading
6. implement path resolution
7. implement scaffold generation
8. implement prompt generation
9. implement verification
10. evolve examples into reusable presets

This sequence keeps the work grounded and incremental.

---

## How to use this document

Use this file when deciding:

- what to work on next
- whether a phase is actually complete
- whether a feature is too early
- how to scope contributions
- how to explain project maturity to contributors

If `ROADMAP.md` says where the project is going,
this file explains how each stage should behave in practice.

---

## Summary

The roadmap should help Archflow stay focused.

The main idea is:

- first make the repository understandable
- then stabilize the model
- then make it operational
- then make it AI-useful
- then make it verifiable
- then make it reusable through presets

If you remember only one thing, remember this:

**Archflow should grow in layers: clarity first, then structure, then execution, then reuse.**