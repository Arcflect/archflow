# Presets

This document explains what a **preset** means in Archflow,
how presets relate to examples,
and how the current examples may evolve into reusable presets over time.

The goal is to clarify that presets are not rigid architecture templates.
They are reusable starting points for project structure, role conventions,
contract defaults, and implementation scaffolding.

---

## Overview

In Archflow, a **preset** is a reusable configuration package
for a common architectural style or ecosystem.

A preset may include:

- project defaults
- placement rules
- contract templates
- artifact conventions
- prompt defaults
- example structures

A preset helps users start faster without having to define every rule from scratch.

---

## Why presets matter

Archflow is designed to be flexible.

That flexibility is important, but it also means new users may ask:

- where should I start?
- which role names should I use?
- what directory structure should I adopt?
- how should contracts be shaped for this architecture?

Presets help answer those questions.

They provide a practical starting point while still allowing customization.

A preset is not meant to replace architectural thinking.
It is meant to reduce setup cost and improve consistency.

---

## Presets are not fixed architecture truth

Archflow should not assume that there is one correct architecture.

Different teams may prefer:

- simple layouts
- layered architecture
- clean architecture
- hexagonal architecture
- modular monolith structures
- ecosystem-specific conventions

Presets exist to support these styles,
not to declare one of them universally correct.

This is a core Archflow principle:

**presets support architectural intent, they do not dictate it**

---

## Relationship between examples and presets

The current `examples/` directory is the natural starting point
for future preset design.

Examples show:

- how input files may look
- how output structure may look
- how roles and contracts may be organized
- how Archflow can express different architectural styles

Presets build on the same ideas,
but move from “illustration” to “reusable configuration”.

You can think of the relationship like this:

- **example** = a teaching artifact
- **preset** = a reusable starting package

Examples are descriptive.
Presets are operational.

---

## Current examples and future preset direction

Archflow currently includes these examples:

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

These examples are useful as documentation today,
and they may become the foundation for future preset definitions.

### `minimal`

Current role:
- the smallest example for understanding the Archflow model

Future preset direction:
- a minimal preset for the smallest useful setup
- useful for experiments, demos, and onboarding
- useful when users want the least amount of structure

Likely preset characteristics:
- small role set
- minimal directory structure
- minimal contract defaults
- minimal prompt shape

---

### `generic-layered`

Current role:
- a language-agnostic layered architecture example

Future preset direction:
- a general-purpose layered preset
- useful for teams that want clear boundaries without ecosystem lock-in
- useful as a neutral default before choosing language-specific conventions

Likely preset characteristics:
- roles for domain, application, interfaces, infrastructure
- broad compatibility across languages
- generic file extensions or configurable output
- moderate default contract structure

---

### `rust-clean-hexagonal`

Current role:
- a Rust-oriented example using clean / hexagonal structure

Future preset direction:
- a Rust clean / hexagonal preset
- useful for Rust projects that care about strong architectural boundaries
- useful for workspace-oriented repository layouts

Likely preset characteristics:
- Rust-friendly role naming
- workspace-aware path conventions
- strong separation of domain, application, and adapters
- more explicit dependency boundary defaults

---

## What a preset may contain

A preset may eventually package some or all of the following:

### 1. Project defaults

Examples:
- architecture style
- language orientation
- workspace defaults
- default modules or starter module structure

### 2. Placement rules

Examples:
- `entity` -> `src/domain/entities/`
- `usecase` -> `src/application/usecases/`
- `controller` -> `src/interfaces/controllers/`

### 3. Contract templates

Examples:
- default responsibilities by role
- default forbidden behaviors by role
- default dependency boundaries by role
- default implementation size guidance

### 4. Prompt defaults

Examples:
- standard artifact prompt sections
- role-specific completion criteria
- prompt formatting variants

### 5. Example artifacts

Examples:
- starter artifact plans
- common artifact names by architecture style
- small sample module definitions

### 6. Optional verification defaults

Future examples:
- required contract fields
- role consistency checks
- structure validation defaults

---

## Preset lifecycle

A useful way to think about preset maturity is in stages.

### Stage 1: example

The structure exists as a documented example.

Purpose:
- explain the concept
- teach the architecture style
- show expected input and output

### Stage 2: draft preset

The structure becomes reusable with fewer manual edits.

Purpose:
- provide a copyable starting point
- establish stable naming and role conventions
- reduce setup cost

### Stage 3: supported preset

The preset becomes an officially supported Archflow starting package.

Purpose:
- give users a stable preset path
- reduce ambiguity in project bootstrap
- support consistent scaffold generation and future verification

This means examples can evolve into presets gradually.
They do not need to become presets all at once.

---

## How examples should evolve into presets

Not every example should automatically become a preset.

An example is a good preset candidate when it is:

- understandable
- repeatable
- broadly useful
- internally consistent
- stable enough to teach as a recommended starting point

A weak preset candidate is:

- too experimental
- too narrow
- too tied to one internal project
- too inconsistent in role naming or contract behavior

The right approach is to let examples prove themselves first.

---

## Suggested preset model

A future preset may look conceptually like this:

- preset name
- intended architecture style
- intended language or ecosystem
- included project defaults
- included placement rules
- included contract templates
- optional starter artifact plan
- optional prompt defaults

For example:

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

This means the current examples already hint at future preset names.

---

## Presets and customization

A preset should always be customizable.

Users should be able to:

- start from a preset
- change placement rules
- change role names
- refine contract templates
- add or remove modules
- override prompt defaults

This is important because Archflow is meant to preserve intent,
not force one fixed structure.

A good preset accelerates setup without removing flexibility.

---

## Presets and examples should coexist

Even after presets exist, examples should still remain.

Why?

Because they serve different purposes.

### Examples help users learn
Examples explain the model and show concrete input/output pairs.

### Presets help users start
Presets reduce bootstrap effort and provide reusable defaults.

A project may begin from a preset,
while the examples continue to serve as documentation and comparison material.

---

## How this connects to current repository structure

Today, the current examples live under:

- `examples/minimal/`
- `examples/generic-layered/`
- `examples/rust-clean-hexagonal/`

In the future, a preset system might reference those same structures
or extract reusable definitions from them.

For example, a future preset directory might look like:

- `presets/minimal/`
- `presets/generic-layered/`
- `presets/rust-clean-hexagonal/`

This does not need to exist yet.

For now, the important thing is to treat examples as the conceptual foundation
for future presets.

---

## Practical interpretation for the current phase

At the current stage of Archflow, the repository does not need a full preset system yet.

What it does need is:

- clear examples
- stable role naming
- stable concept definitions
- stable schema drafts
- clear documentation of how examples may become presets

That is exactly why this document exists.

It explains the direction without requiring preset implementation too early.

---

## Summary

A preset in Archflow is a reusable starting point for architecture-aware scaffolding.

It is not a rigid template.
It is not architecture dogma.
It is not a replacement for design.

It is a practical package of defaults that helps users start faster and more consistently.

The current examples:

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

are the natural foundation for future presets.

If you remember only one thing, remember this:

**examples teach the model, presets operationalize it**