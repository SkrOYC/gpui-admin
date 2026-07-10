# ADR-005: Replica Reconciliation — Bare LWW, Two-Layer Overlay, Staged Undo, Immutable Records

**Status:** Accepted.

## Context

The Replica (R2) must reconcile Backend truth, change-feed events, and optimistic local mutations without requiring Backend cooperation (Backend Sovereignty). Alternatives considered: version/sequence guards and echo tokens (both demand cooperation), denormalized per-query stores (cross-query inconsistency), fold-everything optimistic membership (client re-implements server ordering — a lie on divergence).

## Decision

- **Record store:** `HashMap<Id, Result<Arc<Record>, DriftError>>` — records are immutable `Arc`s; installs are pointer swaps; memoized query snapshots hold `Arc`s (cheap rebuilds). Per-record `Result` is the drift containment boundary (CAP-205).
- **Reconciliation:** bare last-write-wins installs; at-least-once, unordered feed delivery absorbed by idempotent upserts. Post-confirm stale-echo flicker and arrival-order regressions are accepted, self-healing (documented in architecture `resilience.md`).
- **Overlay:** pure-server snapshot + read-time pending layer. Field patches by id; deletes hide by id; creates render in the provisional band (membership under simple typed predicates is client-evaluated; *ordering* is never faked).
- **Lifecycle:** `Staged (undo window, undispatched) → InFlight → Settled(Confirmed | Failed)` with per-Record FIFO dispatch (R3). Undo = drop staged overlay; nothing was sent.
- **Staleness safety:** per-query generation counters; invalidation bumps the generation and drops in-flight tasks (substrate tasks are cancel-on-drop — verified upstream); stale-generation responses discarded.

## Consequences

- Correct against a fully uncooperative Backend; every stronger guarantee (conditional updates, feed resume) layers on as a Capability without reworking this core.
- The state machine is the project's complexity center (RSK-01): it is `gpui-admin-core` code with zero UI dependencies, tested deterministically.
- Memory: warm replicas hold `Arc`'d records until app exit; bounded by the schema-bounded replica count and query-entry TTL eviction.
