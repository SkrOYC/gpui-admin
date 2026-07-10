# Out of Scope: No-Code / Visual Admin Builder

**Status:** Rejected (structural).
**Since:** v0.1.0.

## Context

Adjacent products let non-developers assemble admin screens through drag-and-drop configuration stored as runtime data.

## Decision

gpui-admin is a **code-first framework**. The Adopter is a developer; declarations are owned source code, reviewed and versioned like any other code.

## Reasoning

The product's correctness pillar depends on declarations being verified at build time against Schema Snapshot truth. A runtime-assembled configuration surface reintroduces exactly the class of failure the product exists to eliminate: screens that break in front of Operators because nothing verified them earlier. The two models are structurally incompatible; choosing both would deliver neither.

## Reopening Conditions

Only if a future major version finds a way to verify visually-assembled configurations with the same rigor as owned declarations — considered unlikely and not pursued.
