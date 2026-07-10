# ADR-009: Sparse Update Patch + Previous-State in the Mutation Contract

**Status:** Accepted.

## Context

Update payload shape decides three things at once: PATCH-vs-PUT Backend compatibility, concurrent-edit blast radius, and whether conditional updates are ever possible without contract changes. Full-record-only clobbers concurrent edits and locks out conditions; patch-only locks out full-PUT Backends.

## Decision

`update(id, patch, previous)`:

- **`patch`** is sparse — the dirty-Field bitset (typed field identity) + values. Form dirty-tracking produces it for free.
- **`previous`** is the loaded baseline record. PATCH-capable Providers send the patch; full-PUT Providers reconstruct `previous + patch` internally; conditional-update-capable Providers derive their precondition (version/timestamp/etag) from `previous` — the **ConditionalUpdate Capability** needs no contract change, only a declaration, and surfaces failures as `MutationError::Conflict`.

(This is react-admin's `data`+`previousData` shape, made typed.)

## Consequences

- One mutation contract serves every Backend update dialect; concurrent-edit blast radius shrinks to actually-edited Fields.
- Providers carry the reconstruction burden (where it belongs — only they know their dialect); the Conformance Suite checks both dialect behaviors.
- `previous` requires the Replica to retain the loaded baseline for open forms — already true for dirty-tracking.
