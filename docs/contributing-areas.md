# Contributing Areas

This document explains where contributors can help Archflow today.

Its purpose is to make contribution opportunities easier to understand by clarifying:

- which areas already exist
- which areas still need definition
- which areas are good for design contributions
- which areas are good for implementation contributions
- which areas are intentionally not ready yet

Archflow is still evolving in layers.
That means not every contribution area has the same level of maturity.

This document helps contributors choose the right place to start.

---

## Overview

Archflow currently has several major contribution areas:

- repository and documentation
- examples
- concept model
- schema design
- CLI implementation
- AI handoff design
- verification design
- presets and ecosystem fit

Some of these areas are ready for immediate contribution.
Others are still better suited for discussion and design refinement.

---

## 1. Repository and documentation

### Why this area matters

Archflow is still defining its public shape.
Clear documentation makes the project understandable before the implementation grows.

### Good contribution types

- improve wording in existing docs
- clarify README sections
- improve document navigation
- refine contributor guidance
- fix inconsistencies in terminology
- improve cross-links between docs

### Good first issues in this area

- broken or missing links
- inconsistent naming across docs
- missing references between examples and concepts
- small readability improvements
- missing explanation of a concept already used elsewhere

### Best for

- first-time contributors
- writers
- editors
- contributors who want to learn the project before writing code

---

## 2. Examples

### Why this area matters

Examples are one of the clearest ways to explain Archflow.

They show how architectural intent becomes:

- placement rules
- artifact plans
- contracts
- prompts
- expected outputs

### Good contribution types

- improve existing examples
- add missing contract fields where appropriate
- improve expected output consistency
- add explanation to example README files
- propose new example structures
- align examples with evolving concept definitions

### Good first issues in this area

- make example naming more consistent
- align one example with current schema terminology
- improve one example README
- add missing prompt or contract file in an existing example
- simplify an example that is too noisy

### Best for

- contributors who like concrete artifacts
- contributors who want to work without touching runtime code first
- people interested in architecture examples

---

## 3. Concept model

### Why this area matters

Archflow depends on a stable conceptual model.

If concepts such as `artifact`, `contract`, or `preset` drift in meaning,
implementation will become inconsistent.

### Good contribution types

- clarify concept boundaries
- refine concept definitions
- identify overlap between concepts
- suggest missing concepts
- improve explanatory docs for how concepts connect

### Good discussion topics in this area

- what should count as an artifact
- how strict contracts should be
- whether prompts should be generated or partially editable
- how presets differ from examples
- how role naming should evolve

### Best for

- contributors interested in design and architecture thinking
- contributors who want to shape the model before implementation hardens
- maintainers and frequent contributors

---

## 4. Schema design

### Why this area matters

Schemas are the bridge between documentation and implementation.

They make the model more precise and help future CLI behavior stay consistent.

### Good contribution types

- improve schema readability
- tighten field definitions
- propose optional or required field changes
- align schemas with examples
- suggest validation-oriented improvements
- reduce ambiguity across schema drafts

### Good contribution topics

- field naming consistency
- optional vs required boundaries
- future compatibility with JSON Schema
- path override behavior
- contract and prompt schema alignment

### Best for

- contributors who like structured formats
- contributors who want to shape future validation behavior
- contributors comfortable with system modeling

---

## 5. CLI implementation

### Why this area matters

The CLI is where Archflow becomes operational.

This area turns concepts and schemas into a working tool.

### Current focus areas

- config loading
- file parsing
- path resolution
- scaffold generation
- prompt generation
- verification entry points

### Good contribution types

- implement config parsing
- implement role-to-path resolution
- implement scaffold directory creation
- implement sidecar file generation
- add basic CLI tests
- improve error handling in early commands

### Best for

- Rust contributors
- contributors interested in practical tooling
- contributors comfortable turning specs into code

### Important note

CLI work should follow the concept and schema model,
not invent new concepts during implementation unless discussed first.

---

## 6. AI handoff design

### Why this area matters

AI handoff is one of the main reasons Archflow exists.

The goal is to make artifacts implementable by humans or lightweight models
with clear constraints.

### Good contribution types

- refine prompt structure
- improve completion criteria design
- define prompt modes such as compact vs detailed
- improve prompt consistency across examples
- identify which contract fields should always flow into prompts

### Good discussion topics

- what makes a prompt usable by a smaller model
- how much context is too much
- whether prompts should be fully generated or partly templated
- how role-specific prompt defaults should work

### Best for

- contributors interested in AI-assisted development
- contributors who use coding assistants in practice
- contributors interested in prompt quality and constraints

---

## 7. Verification design

### Why this area matters

Without verification, contracts and scaffold structure may drift over time.

Verification protects architectural intent after initial generation.

### Good contribution types

- define verify scope
- suggest first verification rules
- identify required consistency checks
- propose example verification output
- design status and file presence checks

### Good first verification targets

- required file presence
- required contract fields
- role consistency across files
- path consistency with placement rules
- contract/prompt presence for planned artifacts

### Best for

- contributors who like rule systems
- contributors interested in consistency and tooling quality
- contributors who want to help before deep code parsing exists

---

## 8. Presets and ecosystem fit

### Why this area matters

Presets will make Archflow easier to adopt in real projects.

They help turn examples and conventions into reusable starting points.

### Good contribution types

- refine existing example-to-preset direction
- suggest role sets for future presets
- identify ecosystem-specific needs
- propose default project bootstrap structures
- compare preset boundaries across architectures

### Good discussion topics

- when an example should become a preset
- how opinionated presets should be
- how to keep presets flexible
- how many presets should be supported early on

### Best for

- contributors interested in architecture patterns
- contributors with ecosystem experience
- contributors who want to improve adoption paths

---

## Areas that are especially good right now

At the current stage of Archflow, the most contribution-friendly areas are:

### High readiness
- documentation improvements
- example refinement
- glossary and terminology cleanup
- schema clarification
- concept alignment

### Medium readiness
- early CLI parsing and scaffold work
- prompt generation structure
- verification rule drafting

### Lower readiness for direct implementation
- full preset engine
- editor integration
- deep code-aware checks
- advanced plugin ecosystem
- vendor-specific AI integrations

These lower-readiness areas are still useful discussion topics,
but they are not the best place to begin implementation.

---

## Good first contribution paths

If you want a practical way to start, choose one of these paths.

### Path 1: documentation-first
- read the core docs
- improve one concept page
- improve cross-linking
- fix inconsistent naming

### Path 2: examples-first
- choose one example
- check it against schemas
- improve consistency
- improve its README

### Path 3: schema-first
- review one schema draft
- identify ambiguous fields
- propose clarification
- align examples with the improved schema

### Path 4: implementation-first
- implement one parsing step
- implement one resolution step
- implement one generation step
- keep behavior aligned with docs and examples

---

## What contributors should avoid for now

To keep the project coherent, contributors should avoid pushing too early into these areas without alignment:

- adding many new concepts at once
- creating architecture-specific behavior not grounded in the current model
- coupling Archflow too tightly to one language or framework
- making prompts the primary source of truth
- implementing advanced features before the minimal flow is stable

This does not mean these ideas are bad.
It means timing matters.

---

## How to choose the right area

A simple way to choose is:

- if you want low-risk contribution, start with docs or examples
- if you want to shape the model, work on concepts or schemas
- if you want to build the tool, work on CLI internals
- if you care about AI workflows, work on prompts and handoff design
- if you care about long-term consistency, work on verification design

All of these are valid ways to contribute.

---

## Summary

Archflow has multiple contribution areas, but not all are equally mature yet.

The best areas to contribute right now are:

- documentation
- examples
- concept clarification
- schema refinement

The next layer after that is:

- minimal CLI
- prompt generation
- verification drafting

If you remember only one thing, remember this:

**the best contribution is the one that strengthens clarity without outrunning the current model**