# Role

## Overview

In Archflow, a **Role** is the architectural classification assigned to an artifact.

A role describes what kind of implementation unit an artifact is,
such as an `entity`, `usecase`, `controller`, `repository_port`, or `gateway`.

Roles are one of the central concepts in Archflow because they connect
architectural intent to structure, contracts, prompts, and future verification.

---

## Purpose

The purpose of a role is to provide a stable architectural label
that Archflow can use across multiple layers of the system.

A role answers questions such as:

- What kind of artifact is this?
- Where should this artifact live?
- Which default contract rules should apply?
- What kind of prompt should be generated for it?
- How should this artifact be interpreted in the architecture?

Without roles, Archflow would have artifact names,
but no consistent way to attach structural or behavioral meaning to them.

---

## Responsibilities

A role is responsible for defining:

- the architectural type of an artifact
- the link between artifacts and placement rules
- the link between artifacts and contract templates
- the link between artifacts and prompt defaults
- a reusable vocabulary for architectural interpretation

A role is not responsible for defining one specific artifact.
That belongs to the artifact definition.

A role is also not responsible for defining exact behavior for every instance.
That belongs to contracts and contract templates.

---

## Core fields

A role is usually represented as a string value inside other files.

Typical usage appears in:

- `artifacts.plan.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- generated contract files
- generated prompt files

Examples of role values include:

- `entity`
- `usecase`
- `service`
- `repository_port`
- `repository_interface`
- `controller`
- `handler`
- `gateway`
- `repository_impl`

---

## Example

```yaml
artifacts:
  - name: create_user
    module: user
    role: usecase

  - name: user
    module: user
    role: entity
```

In this example:

- `create_user` is interpreted as a usecase
- `user` is interpreted as an entity

That single role value affects structure, contracts, and prompts.

---

## Relationship to other concepts

A role connects several parts of the Archflow model.

The relationship is:

- the project defines the overall architectural frame
- the module defines a meaningful area within the project
- the artifact defines a concrete implementation unit
- the role defines what kind of unit that artifact is
- the placement rule defines where that role should live
- the contract template defines default behavior for that role
- the prompt generation can use the role to shape implementation handoff

This makes role one of the main connecting concepts in Archflow.

---

## Why role is central in Archflow

Role is central because it allows Archflow to reuse architectural meaning.

Instead of defining everything from scratch for every artifact,
Archflow can use role as a shared layer of interpretation.

For example, if an artifact has role `entity`,
Archflow can infer things such as:

- it probably belongs in a domain-oriented location
- it should not depend on infrastructure concerns
- its contract should emphasize invariants
- its prompt should stay focused on domain behavior

This reduces duplication and keeps the model more coherent.

---

## Design principles

A role should be:

- stable
- understandable
- reusable across artifacts
- meaningful in the architecture
- aligned with placement and contract behavior

A good role name should make architectural interpretation easier,
not harder.

---

## What a role should not do

A role should not:

- replace the artifact name
- replace the module name
- encode a full business feature
- become too specific to one single artifact
- become so vague that it loses structural meaning

For example, a role such as `very_special_user_creation_logic_handler`
would be too specific.
That is closer to an artifact name than a reusable role.

---

## Role vs artifact

A role is not the same as an artifact.

- a **role** is a reusable architectural category
- an **artifact** is a concrete implementation unit

For example:

- role: `usecase`
- artifact: `create_user`

This distinction is important because many artifacts may share the same role.

---

## Role vs module

A role is not the same as a module.

- a **module** groups artifacts by functional area
- a **role** classifies artifacts by architectural type

For example:

- module: `user`
- role: `usecase`
- artifact: `create_user`

This means the same role may appear in many modules.

---

## Role vs contract template

A role is not the same as a contract template.

- a **role** identifies an architectural type
- a **contract template** provides reusable defaults for that type

For example:

- role: `entity`
- contract template for `entity`:
  - protect domain invariants
  - avoid infrastructure dependencies

The role identifies.
The template elaborates.

---

## Examples of roles

Common example roles include:

- `entity`
- `usecase`
- `service`
- `repository_port`
- `repository_interface`
- `controller`
- `handler`
- `gateway`
- `repository_impl`

Not every project needs all of these.

Different examples or presets may use different subsets of roles.

For example:

- `minimal` may use only `entity` and `usecase`
- `generic-layered` may use `entity`, `service`, `controller`, `gateway`
- `rust-clean-hexagonal` may use `entity`, `usecase`, `repository_port`, `repository_impl`, `http_handler`

---

## Role naming guidance

Role names should be:

- concise
- reusable
- architecture-oriented
- consistent across files

Good role names tend to describe a kind of artifact,
not one specific artifact instance.

Examples of good role names:

- `entity`
- `usecase`
- `controller`
- `gateway`

Examples of weak role names:

- `user_create_file`
- `special_logic_part`
- `main_thing`
- `handler2`

Weak role names make placement and contract behavior harder to reason about.

---

## Why it matters

Many parts of Archflow depend on role consistency.

If role naming drifts, then all of the following become weaker:

- path resolution
- contract template application
- prompt consistency
- example clarity
- future verification

A stable role system makes Archflow much easier to scale.

---

## Future directions

In the future, roles may also support:

- role aliases
- preset-specific role maps
- role inheritance or specialization
- role validation rules
- role-specific verification checks
- ecosystem-specific role conventions

Even with those extensions, the basic purpose stays the same:

a role gives an artifact reusable architectural meaning.