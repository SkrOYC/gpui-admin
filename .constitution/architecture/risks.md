# Logical Risks & Technical Debt

Factual, current, with owners-in-architecture and mitigations. Ordered by expected impact.

## RSK-01 — Replica/overlay state-machine complexity

The Replica (R2) + Mutation Coordinator (R3) pair is the hardest logic in the system: memoized views, generations, field-aware invalidation, three-state overlays, FIFO settlement, feed reconciliation. Subtle interleavings (settlement racing a feed event racing an invalidation) are the likeliest defect nest.
**Mitigation:** this pair is designed headless and deterministic — testable without UI, with simulated time and scripted Providers; the diagnostics panel (R8) exposes its internals live; the Conformance Suite's scripted Provider doubles as the test fixture source. Invariants (one-way truth, subtraction-only rollback, generation monotonicity) are stated in `resilience.md` and must become executable checks.

## RSK-02 — Foundation churn beneath the client

The run-time containers presuppose a reactive native UI substrate that is still pre-1.0 (per the pre-constitution verification record). Breaking substrate changes tax every UI container (R5/R6) at once.
**Mitigation:** the substrate family is tracked via git at lockfile-frozen revs with dual-location manifests (tech-spec ADR-011 supersedes ADR-004's registry-only policy) — upgrades are deliberate, reviewed acts: one PR, re-freezing all revs together, re-exercising the tracer-bullet seams. Publishes gate on upstream registry releases catching up, verified by a CI lane resolving from the registry alone. The windowed-list and dock integration seams are proven by the earliest milestone before deep investment; R2/R3/R1/R10/C1 are substrate-independent by design, confining churn to the presentation half.

## RSK-03 — Windowed-browsing ergonomics at the 10M envelope

NFC-01 commits to fluid scrolling over 10M-Record collections. The primitives exist (verified), but nobody has publicly demonstrated sparse random-access windowing at this envelope on this substrate.
**Mitigation:** first-milestone tracer bullet exercises exactly this seam with synthetic data at envelope scale; the honest degradation path (sequential frontier loading) is already an architectural first-class citizen (CAP-606), so failure here is a UX concession, not a redesign.

## RSK-04 — Dual introspection frontends double the toolchain surface

Two Schema Source classes in v1 (CAP-102) was a deliberate scope choice to force Snapshot neutrality; it also doubles parser/inference surface and its bug classes.
**Mitigation:** one shared Snapshot semantics fixture suite that both frontends must satisfy (the toolchain's analogue of the Provider Conformance Suite); the API-description frontend's weaker truth (nullability/relations often unstated) is materialized conservatively in Scaffolds — visible to the Adopter, never guessed silently.

## RSK-05 — Build-time budget vs. static specialization

Per-Resource static derivation (B4) directly stresses NFC-10 (<30 s incremental). At the envelope (~200 Resources), naive derivation could blow the budget and poison the Adopter experience the product sells.
**Mitigation:** derivation boundaries must be incremental-build-friendly (per-Resource units, no global regeneration on single-declaration change); budget measured from the tracer bullet onward as a tracked regression metric, not discovered late.

## RSK-06 — Last-write-wins residual (accepted, bounded)

Without the conditional-update Capability, concurrent Operator edits can overwrite (PRD-accepted trade-off). Bounded by: sparse patches (only dirty Fields travel), mid-edit drift warning (CAP-306), and the Capability upgrade path (CAP-307).
**Debt marker:** none to repay — this is a documented ceiling, revisited only if the low-contention assumption (NFC-42) stops holding for real adopters.

## RSK-07 — Contract/suite drift

If the Provider contract evolves faster than the Conformance Suite (C1), "conformance" silently stops meaning correctness, and third-party Providers rot invisibly — the exact failure the suite exists to prevent.
**Rule:** a contract change and its suite change are one atomic unit; a contract change without new/updated conformance checks is an invalid change. (Binds Stage 3/4 process.)

## RSK-08 — String-resolution boundaries at restore time

The two airlocks are startup-asserted, but persisted Workspace layouts (R9) can reference Resources or Panel kinds that no longer exist after an application upgrade.
**Decision (logical):** restoration is total, never fails: unresolvable panels restore as inert placeholder panels identifying what they referenced, preserving the rest of the layout. A restore must never crash or silently drop the Operator's arrangement.

## RSK-09 — Cross-window notification amplification

Shared Replicas serving N Workspace windows mean one change fans out to every observing panel; a busy change feed could multiply into UI-thrash across windows.
**Mitigation:** observation notifications are coalesced per frame at the R2 boundary (one data change, at most one repaint per panel per frame); the field-aware invalidation already minimizes which panels are notified at all.

## RSK-10 — Diagnostics surface as an accidental disclosure channel

The dev-mode panel (R8) renders live Backend data and internal state; on a shared screen this is a disclosure hazard, and if ever left enabled in a distributed build, a support burden.
**Mitigation:** developer mode is an explicit, visible, per-machine opt-in, off by default in every build; the panel is watermarked as diagnostic; logs redact secret-classified values at the R1 boundary by construction.

## Structural debt accepted at v0.2.0

- Two permanent pagination flows and two freshness flows (sovereignty cost — see `strategy.md`).
- Sequential-Backend UX ceilings (no random jumps, optional totals) are permanent honest behavior, not debt to repay.
- P1 identity work (CAP-702/703) is stubbed by design in v1: the none-mode default is first-class, and the error taxonomy already reserves the routes P1 will use — no rework anticipated, only addition.
- Orphaned objects can exist after crashes between object-upload and record settlement; bounded by best-effort cleanup and made observable through R8's orphan report sweep rather than prevented.
- Cross-binding Relations carry a permanently weaker verification tier than Snapshot-backed checks; documentation must keep the tiers loud so the marker is never mistaken for decoration.
- Junction detection infers candidate junctions from dual-FK shape; non-pure junctions (attribute-bearing relation rows) require the explicit has-many fallback — misclassification surfaces as a Scaffold review comment, never silent behavior.
