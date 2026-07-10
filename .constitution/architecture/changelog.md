# Stage 2 Changelog — `.constitution/architecture/`

## v0.1.0 — 2026-07-09

Initial architecture constitution, derived from `.constitution/prd/` v0.1.0.

- Created `strategy.md`: two-context pattern — offline derivation pipeline (Snapshot → owned declarations → build-time verification; Drift is unshippable) + single-process reactive desktop client (per-Resource Replicas, hexagonal Provider boundary, capability-gated Backend cooperation). Trade-offs named: thick stateful client, build-time cost, permanent dual pagination/freshness paths, honesty ceilings in optimistic UI, single-process envelope, LWW residual.
- Created `containers.md`: 4 build-time containers (Introspection Frontends, Snapshot Store, Scaffolder, Derivation & Verification), 1 shared contract boundary (Provider Contract + Conformance Suite), 9 run-time containers (Provider Gateway, Resource Replica, Mutation Coordinator, Resource Registry, Workspace Shell, View & Form Engine, Session & Access, Diagnostics & Logging, Local State Store), with labeled communication categories and C4 diagram.
- Created `resilience.md`: trust boundaries (Backend-only enforcement, two named egress points), normalized error taxonomy, read/mutation failure policies (generation discipline; mutations never auto-retried; rollback by subtraction), connectivity state machine with degraded mode, change-feed reconnect staleness rule, one-way truth flows, trace-correlated local observability + dev-mode diagnostics panel, code-first configuration posture.
- Created `risks.md`: RSK-01..10 with mitigations (state-machine complexity, substrate churn, 10M windowing proof, dual-frontend surface, build-budget, LWW residual, contract/suite atomicity rule, total-restoration rule for stale layouts, cross-window notification coalescing, diagnostics disclosure gating).
- Created `flows/` (15 files) mapping **all 27 P0 capabilities** plus the degraded-connectivity unhappy path: schema-capture-and-scaffold, drift-detection-at-build, windowed-browsing, filter-sort-search, mutation-lifecycle, mid-edit-drift-warning, relations, workspace-persistence, master-detail-follow, deep-link, provider-contract-and-capabilities, conformance-validation, unauthenticated-and-session, degraded-connectivity, adopter-onboarding.

Decision provenance: Stage 2 interview 2026-07-09 (single process / one OS window per Workspace / shared warm Replicas; logs + dev diagnostics panel; backend-unreachable degraded mode) over the PRD v0.1.0 and the session's pre-constitution design exploration.
