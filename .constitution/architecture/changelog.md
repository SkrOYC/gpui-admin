# Stage 2 Changelog — `.constitution/architecture/`

## v0.2.0 — 2026-08-21

Realign Evolution pass over prd v0.2.0 (rulings: `../reports/2026-08-21-interview-realign.md`).

### Added
- **R10 Object Store Port** container (independent port + first-party adapter): sole path to binary-object storage; object-first upload ordering inside R3's dispatch with best-effort cleanup and R8 orphan reporting; unconfigured degradation to reference-as-text fields. Third named egress boundary.
- **Per-Resource Provider Bindings:** R1 instantiated per binding; R4 resolution binding-aware; cross-binding Relations resolved as structurally-verified-only edges with visible markers; per-Gateway connectivity states.
- **Flows:** new `flow-object-upload-lifecycle`, `flow-multi-backend-binding`, `flow-draft-persistence`, `flow-form-rendering-and-validation`, `flow-schema-migration-reconcile` (the previously-named-but-unspecified regenerate-and-reconcile workflow).
- **Revised flows:** mutation-lifecycle (destructive-action confirmation CAP-308, Draft hooking, configurable Undo Window, Quick Edit riding the lifecycle), deep-link (declared scheme, typed Query payloads, Saved Query bookmarking), filter-sort-search (labeled loaded-window Search fallback replacing control-absence).
- Containers updated for: declaration-driven sidebar/breadcrumbs/command palette and OS-following theming in R5; input catalog, column masks, Quick Edit, m2m pickers, hierarchy cues, temporal rendering in R6; drafts/masks/bookmarks payload classes in R9; bulk forms and permission vocabulary in C1; namespaced Snapshots with Junction/tree flags in B2.
- Strategy trade-offs extended: third egress boundary, multi-sovereign routing, staged grid-editing ambition.
- Risks: RSK-02 mitigation rewritten around git-frozen-rev substrate tracking; structural-debt register extended (orphaned objects, weak-tier relations, junction inference limits).

## v0.1.1-reviewed — 2026-07-10

Reviewed for upstream delta (prd v0.1.1, licensing change); no changes required — recorded retroactively; this entry was omitted at the time.

## v0.1.0 — 2026-07-09

Initial architecture constitution, derived from `.constitution/prd/` v0.1.0.

- Created `strategy.md`: two-context pattern — offline derivation pipeline (Snapshot → owned declarations → build-time verification; Drift is unshippable) + single-process reactive desktop client (per-Resource Replicas, hexagonal Provider boundary, capability-gated Backend cooperation). Trade-offs named: thick stateful client, build-time cost, permanent dual pagination/freshness paths, honesty ceilings in optimistic UI, single-process envelope, LWW residual.
- Created `containers.md`: 4 build-time containers (Introspection Frontends, Snapshot Store, Scaffolder, Derivation & Verification), 1 shared contract boundary (Provider Contract + Conformance Suite), 9 run-time containers (Provider Gateway, Resource Replica, Mutation Coordinator, Resource Registry, Workspace Shell, View & Form Engine, Session & Access, Diagnostics & Logging, Local State Store), with labeled communication categories and C4 diagram.
- Created `resilience.md`: trust boundaries (Backend-only enforcement, two named egress points), normalized error taxonomy, read/mutation failure policies (generation discipline; mutations never auto-retried; rollback by subtraction), connectivity state machine with degraded mode, change-feed reconnect staleness rule, one-way truth flows, trace-correlated local observability + dev-mode diagnostics panel, code-first configuration posture.
- Created `risks.md`: RSK-01..10 with mitigations (state-machine complexity, substrate churn, 10M windowing proof, dual-frontend surface, build-budget, LWW residual, contract/suite atomicity rule, total-restoration rule for stale layouts, cross-window notification coalescing, diagnostics disclosure gating).
- Created `flows/` (15 files) mapping **all 30 P0 capabilities** plus the degraded-connectivity unhappy path: schema-capture-and-scaffold, drift-detection-at-build, windowed-browsing, filter-sort-search, mutation-lifecycle, mid-edit-drift-warning, relations, workspace-persistence, master-detail-follow, deep-link, provider-contract-and-capabilities, conformance-validation, unauthenticated-and-session, degraded-connectivity, adopter-onboarding.

Decision provenance: Stage 2 interview 2026-07-09 (single process / one OS window per Workspace / shared warm Replicas; logs + dev diagnostics panel; backend-unreachable degraded mode) over the PRD v0.1.0 and the session's pre-constitution design exploration.
