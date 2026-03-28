# 0007 Archflow remains sidecar-first before code-aware analysis

- Status: accepted
- Date: 2026-03-28

## Context

Archflow is being designed as an architecture-to-execution bridge.

At this stage, its main assets are:

- project definitions
- placement rules
- artifact plans
- contracts
- prompts
- scaffold structure
- examples
- schema drafts

A natural future direction is code-aware analysis, such as:

- source file inspection
- import analysis
- dependency graph analysis
- AST-aware validation
- language-specific structural checks

These directions may become useful later, especially for stronger verification.

However, introducing code-aware analysis too early would change the center of gravity of the project.

It would create pressure toward:

- language-specific behavior
- parser-first design
- implementation-derived architecture interpretation
- heavier runtime complexity
- less useful pre-code workflows

This would be risky because Archflow’s current value comes from making architecture explicit before implementation is complete.

The project therefore needs a clear sequencing decision.

## Decision

Archflow remains **sidecar-first before code-aware analysis**.

This means the primary architectural records remain:

- project definition files
- placement rules
- artifact plans
- contract files
- prompt files

Source code may become an additional signal later,
but it is not the primary source of truth in early Archflow.

Code-aware analysis may be added in the future,
but only after the sidecar-centered model is stable enough.

In practical terms, the ordering is:

1. define architecture through structured files
2. generate scaffold, contracts, and prompts
3. verify structure and contract consistency
4. only later expand into optional code-aware analysis where useful

## Consequences

What becomes easier:
- preserving language-agnostic design
- supporting pre-implementation workflows
- keeping contracts and prompts central
- making examples meaningful even without full code
- keeping early verification focused and clear
- avoiding premature parser complexity

What becomes harder:
- validating implementation details directly from source code in early phases
- catching drift that only appears inside code
- providing advanced language-specific architectural enforcement immediately

This is an intentional tradeoff.

Archflow is designed to start from explicit architecture,
not reverse-engineer architecture from code.

## Why this fits Archflow

This decision aligns with earlier project decisions.

It supports the idea that:

- Archflow is an architecture-to-execution bridge
- contracts are the source of truth for artifact boundaries
- prompts are derived from contracts
- sidecar files are first-class
- verification starts with structure and contract consistency
- examples should stabilize before more operational expansion

This decision keeps the project coherent.

If Archflow became code-aware too early,
it could drift toward becoming a language-specific architecture analysis tool,
which is not its primary early purpose.

## What “sidecar-first” means in practice

Sidecar-first means that important architectural intent should live in files such as:

- `project.arch.yaml`
- `placement.rules.yaml`
- `artifacts.plan.yaml`
- `*.contract.yaml`
- `*.prompt.md`

It also means:

- code is not required for Archflow to be useful
- placeholders can still be meaningful
- the model remains usable in design-first workflows
- AI handoff can happen before full implementation exists

This is important because Archflow is meant to work in the phase between design and completed code.

## What this decision does not mean

This decision does **not** mean:

- Archflow will never inspect code
- code-aware validation is a bad idea
- language-specific integrations are forbidden
- source code should be ignored forever

It only means that code-aware analysis should remain **downstream** from the sidecar model.

Code-aware features should extend Archflow later.
They should not redefine its foundation too early.

## Alternatives considered

### Make code-aware analysis a primary early feature

Not chosen because it would push the project toward language-specific complexity before the core model is stable.

### Treat source code and contracts as equal sources of truth

Not chosen because this creates ambiguity in early phases and weakens the clarity of architectural ownership.

### Avoid code-aware analysis entirely

Not chosen because future code-aware checks may provide real value,
especially for stronger verification and ecosystem fit.

## Future direction

In the future, Archflow may support optional code-aware analysis such as:

- import-pattern checks
- lightweight dependency checks
- role-aware file inspections
- optional AST-backed verification
- ecosystem-specific structural validation

But these should remain additive layers.

The core flow remains:

**project -> placement rules -> artifact plan -> contract -> prompt -> scaffold -> verify**

Code-aware analysis, if added, should appear after this core flow is stable.

## Notes

This decision should guide future discussions about:

- verification scope
- language-specific adapters
- parser integration
- AST-backed validation
- ecosystem-specific enforcement

If future code-aware features are added,
they should be evaluated against this question:

Does this feature extend the sidecar-first model,
or does it try to replace it too early?