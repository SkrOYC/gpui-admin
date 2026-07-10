# Functional Capabilities

Priorities: `P0` = critical path (first public release must contain it — "demo-complete" cut), `P1` = next phase, `P2` = optional/nice-to-have.

## Epic E1 — Schema-to-Admin Toolchain

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-101 | Capture the structure of an external Schema Source into a versioned, human-reviewable Schema Snapshot, entirely offline. | Schema truth must be visible, diffable in review, and usable in air-gapped builds. |
| P0 | CAP-102 | Support two distinct classes of Schema Source — database catalogs and machine-readable API descriptions — producing identical Snapshot semantics. | Two real source classes force the Snapshot to stay neutral; the toolchain must not privilege one Backend shape. |
| P0 | CAP-103 | Generate a one-time, Adopter-owned Scaffold per Resource, with inferred defaults (labels, input kinds, requiredness, Relations, Projections) written out as explicit, editable declarations. | Infer-then-own: aggressive inference with zero hidden magic; the Adopter reviews and owns every default. |
| P0 | CAP-104 | Fail the build when owned declarations contradict a regenerated Schema Snapshot, with the failure pointing at the exact stale declaration. | Drift surfaces before it ships, at the desk of the person who can fix it. |
| P2 | CAP-105 | Derive a Schema Snapshot by observing live data when no structured Schema Source exists. | Weakest-truth fallback; extends reach to Backends with no catalog or description. |

## Epic E2 — Browsing & Views

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-201 | List Views stay fluid regardless of collection size, presenting only what is visible and fetching on demand. | Desktop-grade browsing over millions of Records is the identity claim. |
| P0 | CAP-202 | Typed Filters and sorting per Resource, where referencing a nonexistent Field is impossible to express (caught at build). | Correctness pillar applied to the most-used browsing controls. |
| P0 | CAP-203 | Free-text Search across a Resource where the Backend supports it. | Search-everywhere is a baseline admin expectation, distinct from typed Filters. |
| P0 | CAP-204 | Show Views present a Record through its declared Projection. | Core read path of the four-View model. |
| P0 | CAP-205 | A malformed Record renders as a single contained error boundary; every other Record in the View remains usable. | One bad row must never take down an Operator's screen. |
| P2 | CAP-206 | Additional View archetypes beyond the core four (e.g., calendar, board). | Valuable, but outside the critical path; the four core Views define v1. |

## Epic E3 — Editing & Write Safety

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-301 | Create and Edit forms are derived from declarations, validating each Field at entry time (type fit and domain rules) while preserving what the Operator typed. | Errors contained at the field edge; no end-of-form surprise dumps. |
| P0 | CAP-302 | Pending Changes appear immediately, and their presentation is honest about what is unconfirmed — a pending creation is visibly provisional rather than faked into a final position. | Optimistic speed without lying about what only the Backend can decide. |
| P0 | CAP-303 | Every Pending Change is revocable during an Undo Window before dispatch; a revoked change never reaches the Backend. | Edit-without-fear: revocation by non-occurrence, not by compensating repair. |
| P0 | CAP-304 | A rejected mutation rolls back visibly, preserves the Operator's input, and offers retry or discard — even if the originating form is closed. | Failure must never silently discard work or silently keep a lie on screen. |
| P0 | CAP-305 | Validation failures reported by the Backend land on the exact Field that caused them. | The Backend's word is final; the Operator should see it where they can act. |
| P0 | CAP-306 | An Operator editing a Record is warned, without being blocked, when that Record changes on the Backend mid-edit. | Cheap, cooperation-free protection against surprised overwrites. |
| P1 | CAP-307 | Where the Backend supports it, updates can be made conditional so concurrent edits are never silently overwritten. | Opt-in Capability: stronger guarantee for cooperating Backends. |

## Epic E4 — Relations

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-401 | Declared Relations render as pick-one inputs (belongs-to) and as embedded related Lists (has-many). | Cross-Resource data is the substance of real admin work. |
| P0 | CAP-402 | A Relation referencing an unknown Resource cannot make it into a running application (verified at build or startup, with a clear message). | Correctness pillar applied to the cross-Resource graph. |
| P0 | CAP-403 | Related-data Lists behave identically to top-level Lists: windowed browsing, Filters, Pending Changes, and freshness all apply. | One mental model; no second-class related views. |

## Epic E5 — Workspace Shell

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-501 | Operators arrange Panels by splitting, tabbing, and floating them within one Workspace. | The multi-panel desktop experience is the product identity. |
| P0 | CAP-502 | Workspace layout persists locally and restores intact on relaunch. | Work is resumed, not restarted; the Workspace belongs to the Operator's machine. |
| P0 | CAP-503 | A detail Panel opened from a List Follows that List's selection live; closing the source simply freezes the follower. | The master-detail payoff of multi-panel; causally obvious, never spooky. |
| P0 | CAP-504 | Edit Panels never Follow, and re-opening a Record already being edited focuses the existing Panel instead of duplicating it. | Losing unsaved edits to a moving selection — or to a twin Panel — is made structurally impossible. |
| P0 | CAP-505 | A Deep Link addresses one primary Panel and opens a fresh Workspace; personal layout is never encoded in a shared link. | Shareable addressing without leaking or freezing anyone's personal Workspace. |
| P1 | CAP-506 | Deep Links are invokable from outside the application via operating-system link handling. | Completes the addressing story; polish, not critical path. |

## Epic E6 — Backend Integration

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-601 | All Backend communication passes through one pluggable Provider contract whose baseline assumes only elementary read and write operations. | Backend sovereignty: the core must be correct against an uncooperative Backend. |
| P0 | CAP-602 | Backend cooperation features — realtime change feeds, cursor paging, conditional updates, counting — are declared, opt-in Capabilities; their absence never degrades correctness, only richness. | Cooperation is rewarded, never required. |
| P0 | CAP-603 | A public Conformance Suite validates any Provider implementation against the contract. | Keeps the first-party Provider unprivileged and makes third-party Providers provable. |
| P0 | CAP-604 | At least one verified Provider against a real, publicly reachable Backend ships with the first release. | The demo must run against reality, not only mocks. |
| P1 | CAP-605 | Where the Backend offers a realtime change feed, open Panels stay fresh without manual refresh and recover safely after connection loss. | Freshness as a Capability; reconnection must not lie about missed changes. |
| P1 | CAP-606 | Collections reachable only through sequential paging remain fully browsable, with totals treated as optional. | Sovereignty applied to pagination physics; sequential Backends are first-class, just different. |

## Epic E7 — Identity & Access

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-701 | Applications run fully featured with no authentication configured. | Private-network and air-gapped deployments are first-class citizens. |
| P1 | CAP-702 | A pluggable authentication surface: sign-in, sign-out, identity display, and mid-session expiry routed to re-authentication without losing Operator work. | Session death must be an interruption, not a data loss. |
| P1 | CAP-703 | Permission-aware UI affordances (hide/disable) that are structurally distinct from authorization; a Backend denial visibly corrects an over-permissive affordance. | The UI may guess for convenience; the Backend's answer is the only enforcement. |

## Epic E8 — Collaboration Awareness

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P2 | CAP-801 | Transient collaboration signals (presence, notifications, broadcasts) can surface in Views but are never persisted into authoritative data. | Ambient awareness without polluting the source of truth. |

## Epic E9 — Actions & Extensibility

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P1 | CAP-901 | Operators select multiple Records and apply bulk actions, with selection surviving list refreshes. | Bulk correction is a staple admin workflow. |
| P1 | CAP-902 | Adopters declare custom Resource-level actions with confirmation and input-collection flows. | Every real admin has verbs beyond CRUD. |
| P1 | CAP-903 | Adopters override any single input widget or Projection entry without leaving the declaration model. | Contained escape hatch: divergence is one explicit block, not a fork. |

## Epic E10 — Adoption & Showcase

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-1001 | A public showcase application demonstrates every P0 capability against the verified Backend. | For an open-source, experience-led product, the demo *is* the argument. |
| P0 | CAP-1002 | An Adopter reaches a working admin over their own Schema Source, from fresh install, in under 30 minutes by following the getting-started path. | Time-to-first-success is the adoption funnel's only gate that matters. |
| P1 | CAP-1003 | A Contributor can author a new Provider and prove it correct using only public documentation and the Conformance Suite. | Ecosystem growth without core-team bottleneck. |
