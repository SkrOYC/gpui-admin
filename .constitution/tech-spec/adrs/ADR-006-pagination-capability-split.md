# ADR-006: Pagination as a Declared Capability Split

**Status:** Accepted.

## Context

An opaque cursor can only extend a sequence frontier; a virtualized table with a proportional scrollbar invites random jumps a cursor Backend structurally cannot serve, and cursor Backends often cannot count. Pretending one windowing model fits both is the overclaim this ADR forbids.

## Decision

Pagination mode is declared by the Provider (Capability):

- **RandomAccess (offset-capable):** true windowed access — the table's visible-range callback demands `[i..j]`; misses fetch windows; proportional scrolling with `total`.
- **SequentialCursor:** frontier loading via load-more semantics; no random jumps; `total: Option<usize>` treated as absent-able everywhere.

Both map onto verified substrate hooks (`visible_rows_changed` / `has_more`+`load_more` on the table delegate). `GetListResult.total` is `Option` in the contract for all Providers.

## Consequences

- Sequential Backends are first-class and honest, not degraded simulations (CAP-606 posture).
- The UI permanently maintains two list-loading paths (named cost in architecture `strategy.md`).
- The Conformance Suite tests each mode only when declared, and verifies the *absence* of undeclared capabilities is handled correctly.
