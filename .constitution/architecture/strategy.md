# Architecture Strategy

## 0. Version

**v0.2.0** — see [`changelog.md`](./changelog.md).

## Architectural Pattern

**A two-context system:**

1. **Build-Time Context — an offline derivation pipeline.** External schema truth is captured into a committed, reviewable Schema Snapshot; Adopter-owned Resource declarations are verified against it and derived into statically specialized program artifacts at build time. Drift between declarations and truth is a build failure, never a runtime state.
2. **Run-Time Context — a single-process, modular, reactive desktop client.** One process hosts many Workspace windows (one OS window per Workspace). Per-Resource **Replica** containers hold authoritative client-side state — normalized record truth, memoized query views, and an honest optimistic overlay — shared by every window. All Backend communication passes through one **Provider Gateway** port (hexagonal boundary), with Backend cooperation modeled as declared, opt-in Capabilities.

There is no server-side component owned by this system. The Backend is external and sovereign; the only trusted enforcement lives there.

## Why This Pattern Fits

Traceability to `.constitution/prd/`:

- **Native-UX identity (vision, NFC-01..04).** A single-process client with in-memory per-Resource Replicas makes every interaction a local read: windowed Lists render from memoized query views (O(1) at draw), Pending Changes appear within a frame because they are applied locally first, and multi-window Workspaces share one warm truth with zero synchronization protocol — the same process memory serves all windows.
- **Build-time correctness (JTBD 2, CAP-104, NFC-12).** Correctness claims require a moment where declarations and truth *must* meet. Placing that meeting at build time — via the committed Snapshot and the Derivation & Verification Engine — makes drift structurally unshippable and keeps the check offline and air-gap-compatible (NFC-11).
- **Backend sovereignty (CAP-601/602, actor: Backend Owner).** The hexagonal Provider Gateway is the *only* container allowed to know how a Backend speaks. The core's baseline assumes elementary reads/writes; realtime feeds, cursor paging, conditional updates, and counting are Capability declarations that unlock richer flows without ever being load-bearing. The Conformance Suite exists so no Provider — including the first-party one — is privileged.
- **Edit-without-fear (CAP-301..306, NFC-21).** A dedicated Mutation Coordinator owns the staged → in-flight → settled lifecycle: the Undo Window is revocation-by-non-occurrence (staged changes were never sent), failures roll back by removing an overlay rather than by compensating writes, and Operator input is preserved on every rejection path.
- **Resumable Workspaces (CAP-501/502, NFC-22).** Workspace layout is a local storage boundary; restoration rebuilds Panels through the same string-resolution boundary used by Deep Links — one mechanism, two entry points.

## Trade-offs Accepted

- **A thick, stateful client.** The Replica/overlay/invalidation machinery is the system's hardest state-machine complexity, accepted to buy instant UX and cross-window consistency. Mitigated by making it the most deterministic, headless-testable, and introspectable part of the system (Diagnostics container).
- **Build-time cost and re-scaffold friction.** Static derivation regresses build/iteration time (bounded by NFC-10: <30 s incremental) and schema migrations demand a regenerate-and-reconcile step, with the build verifier as the guide. Accepted: the correctness pillar depends on it.
- **Dual paths where Backend physics differ.** Capability-gating means the architecture permanently maintains two pagination flows (random-access vs sequential frontier) and two freshness flows (change-feed vs staleness-driven refetch). Accepted as the honest cost of sovereignty; the alternative is privileging one Backend shape.
- **Honesty over illusion in optimistic UI.** Pending creations are presented provisionally instead of faked into final positions; totals may be absent on sequential Backends; a brief post-confirmation flicker under concurrent change is tolerated as self-healing. These are deliberate UX ceilings, not defects.
- **Single-process ceiling.** No horizontal or multi-process story; the scale envelope (NFC-42: ~200 Resources, tens of Operators against a Backend) is a stated design assumption, not an open promise.
- **Last-write-wins residual.** Without the conditional-update Capability, concurrent edits can overwrite (PRD accepts this; CAP-306 warns, CAP-307 upgrades it where the Backend cooperates).
- **A third egress boundary.** The Object Store adds one more configured network surface beside the Backend — accepted because binary content is unavoidable in real admins and an independent port keeps record truth vendor-neutral. Unconfigured apps keep two-boundary discipline.
- **Multi-sovereign routing.** Per-Resource Provider Bindings multiply integration surface (one Gateway instance per binding) and create structurally-verified-only cross-binding Relations — accepted as the honest price of spanning systems.
- **Staged grid-editing ambition.** Quick Edit ships P1 on existing machinery; full multi-cell editing waits on measured feasibility rather than optimism.
