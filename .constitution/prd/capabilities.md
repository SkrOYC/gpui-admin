# Functional Capabilities

Priorities: `P0` = critical path (first public release must contain it — "demo-complete" cut), `P1` = next phase, `P2` = optional/nice-to-have.

## Epic E1 — Schema-to-Admin Toolchain

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-101 | Capture the structure of an external Schema Source into a versioned, human-reviewable Schema Snapshot, entirely offline, with type fidelity sufficient for honest presentation (exact-numeric scalars, distinct temporal kinds, nullability) and structural pattern recognition (Junction candidates, self-referencing Relations). | Schema truth must be visible, diffable in review, usable air-gapped, and rich enough that downstream rendering never guesses what the source actually declares. |
| P0 | CAP-102 | Support two distinct classes of Schema Source — database catalogs and machine-readable API descriptions — producing identical Snapshot semantics. | Two real source classes force the Snapshot to stay neutral; the toolchain must not privilege one Backend shape. |
| P0 | CAP-103 | Generate a one-time, Adopter-owned Scaffold per Resource, with inferred defaults (labels, input kinds, requiredness, declarative validation rules derivable from truth, Relations including Many-to-Many convenience declarations over detected Junctions, Projections) written out as explicit, editable declarations. | Infer-then-own: aggressive inference with zero hidden magic; the Adopter reviews and owns every default. |
| P0 | CAP-104 | Fail the build when owned declarations contradict a regenerated Schema Snapshot, with the failure pointing at the exact stale declaration. | Drift surfaces before it ships, at the desk of the person who can fix it. |
| P2 | CAP-105 | Derive a Schema Snapshot by observing live data when no structured Schema Source exists. | Weakest-truth fallback; extends reach to Backends with no catalog or description. |

## Epic E2 — Browsing & Views

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-201 | List Views stay fluid regardless of collection size, presenting only what is visible and fetching on demand. | Desktop-grade browsing over millions of Records is the identity claim. |
| P0 | CAP-202 | Typed Filters and sorting per Resource (multi-column ordering expressible), where referencing a nonexistent Field is impossible to express (caught at build). | Correctness pillar applied to the most-used browsing controls. |
| P0 | CAP-203 | Free-text Search across a Resource where the Backend supports it; where it does not, a visibly labeled client-side fallback searches loaded rows ("searching loaded rows") rather than hiding the affordance or pretending completeness. | Search-everywhere is a baseline admin expectation; honest degradation matches the framework's truth-telling posture. |
| P0 | CAP-204 | Show Views present a Record through its declared Projection. | Core read path of the four-View model. |
| P0 | CAP-205 | A malformed Record renders as a single contained error boundary; every other Record in the View remains usable. | One bad row must never take down an Operator's screen. |
| P0 | CAP-211 | Temporal Fields render type-faithfully: absolute instants display in the Operator's machine zone by default with Adopter-declared per-Projection overrides; naive timestamps and date-only values render verbatim, never converted, with their undeclared-zone nature marked at entry time. | Time is where silent lying is easiest; the framework renders what the source actually declares, no fabricated conversions. |
| P1 | CAP-207 | Operators show/hide List columns and override per-list sort, persisted locally as a mask over the declared Projection; vanished Fields degrade gracefully on restore. | Daily-operator comfort over wide Resources without touching build-time ownership of Projections. |
| P1 | CAP-208 | Named Saved Queries over a Resource are bookmarkable locally and shareable as Deep Links carrying the typed Query; received links open the query directly and can be bookmarked. | Team workflows ("use this filter") plus personal reuse, both built on content-addressable addressing. |
| P1 | CAP-209 | Show/Edit Panels offer previous/next Record navigation following the source List's ordering at open time; disabled at frontiers; forward-only past the loaded frontier on sequential-paging Backends; blocked while the form carries unsaved input. | The reflexive operator gesture after opening a record; cheap because query membership is already client-known. |
| P1 | CAP-210 | Number and date rendering follows a configurable presentation-edge policy (system-locale default, canonical alternative one declaration away), chosen by the Adopter-developer; string sort order always remains Backend-owned. | Native-feeling output without crossing into translation territory; collation honesty preserved. |
| P2 | CAP-206 | Additional View archetypes beyond the core four (e.g., calendar, board). | Valuable, but outside the critical path; the four core Views define v1. |
| P2 | CAP-212 | Resources whose Snapshot shows a self-referencing Relation render hierarchy cues (indentation, root-to-leaf breadcrumb path) via depth-limited eager fetching. | Hierarchy stops being invisible at bounded cost; fuller tree interaction stays under CAP-206. |

## Epic E3 — Editing & Write Safety

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-301 | Create and Edit forms are derived from declarations, validating each Field at entry time (type fit plus a closed set of declarative rules — requiredness, enum membership, bounds, length, pattern — with Scaffolder-emitted defaults for the derivable ones, and Adopter closures for domain logic) while preserving what the Operator typed. | Errors contained at the field edge; scaffolds validate out-of-the-box; backend validation always has final precedence. |
| P0 | CAP-302 | Pending Changes appear immediately, and their presentation is honest about what is unconfirmed — a pending creation is visibly provisional rather than faked into a final position. | Optimistic speed without lying about what only the Backend can decide. |
| P0 | CAP-303 | Every Pending Change is revocable during an Undo Window before dispatch — window duration defaulting framework-wide and overridable per Resource by the Adopter — and a revoked change never reaches the Backend. | Edit-without-fear: revocation by non-occurrence, not by compensating repair; regret windows differ by operation, so duration is developer-owned. |
| P0 | CAP-304 | A rejected mutation rolls back visibly, preserves the Operator's input, and offers retry or discard — even if the originating form is closed. | Failure must never silently discard work or silently keep a lie on screen. |
| P0 | CAP-305 | Validation failures reported by the Backend land on the exact Field that caused them, overriding any client-side rule verdict. | The Backend's word is final; the Operator should see it where they can act. |
| P0 | CAP-306 | An Operator editing a Record is warned, without being blocked, when that Record changes on the Backend mid-edit. | Cheap, cooperation-free protection against surprised overwrites. |
| P0 | CAP-308 | Destructive actions — record deletion above all — require explicit confirmation before dispatch. | Deletion responsibility sits with the Operator under an unambiguous "are you sure?" gate; the Undo Window complements but never replaces this confirmation. |
| P0 | CAP-309 | Unsaved form input persists locally as a Draft (debounced), restores verbatim after relaunch or crash alongside any mid-edit drift warning, and clears only on confirmed save or explicit discard; Drafts are never dispatched to the Backend. | No typed work is ever silently lost — offline, across crashes, across sessions — without spending the mutation lifecycle's guarantees. |
| P1 | CAP-307 | Where the Backend supports it, updates can be made conditional so concurrent edits are never silently overwritten. | Opt-in Capability: stronger guarantee for cooperating Backends. |
| P0 | CAP-312 | Forms render every supported Field kind through a declaration-derived input catalog — text, multiline text, exact-decimal numbers (never binary-float approximations of exact-numeric truth), switches, dates, date-times, times, passwords, validated email/URL variants, enum selects, relation selects, rich text/markdown, file and image inputs backed by the configured Object Store — with widget inference materialized by the Scaffolder as explicit owned declarations and per-field divergence via the override escape hatch. | Forms are the daily surface of an admin; this is the breadth commitment, and exact numerics exist because money silently corrupted through floating point is a correctness failure, not a UX nit. |
| P1 | CAP-310 | Quick Edit: simple widgets — switches, enum-selects, numbers incl. exact-decimal numbers, and text — are editable directly in List rows through the same Pending Change lifecycle (Undo Window, FIFO dispatch, rollback), with row-level error display; Quick Edit never opens or duplicates an Edit Panel. | Flipping one status should not require opening a form; reuses machinery that already exists. |
| P2 | CAP-311 | Full multi-cell grid editing (range selection, batch staging, clipboard flows), pursued only if its feasibility spike demonstrates editor-state survival under virtualization and sound batch-undo semantics. | DataGrip-class destination; staged deliberately so ambition never destabilizes the proven state machines. |

## Epic E4 — Relations

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-401 | Declared Relations render as pick-one inputs (belongs-to) and as embedded related Lists (has-many). | Cross-Resource data is the substance of real admin work. |
| P0 | CAP-402 | A Relation referencing an unknown Resource cannot make it into a running application (verified at build or startup, with a clear message). | Correctness pillar applied to the cross-Resource graph. |
| P0 | CAP-403 | Related-data Lists behave identically to top-level Lists: windowed browsing, Filters, Pending Changes, and freshness all apply. | One mental model; no second-class related views. |
| P0 | CAP-404 | Many-to-many Relations declare over detected Junction Resources (`many_to_many<J, Target>(via_self, via_target)` — junction FK fields, type-checked at registration), compiling to filtered queries through the Junction; pick-one inputs gain multi-select behavior where declared; non-pure junctions fall back to ordinary has-many editing. | Junction-row UX is what operators hate; sugar compiles to existing query machinery, so runtime cost is near zero. |
| P1 | CAP-405 | A Relation may target a Resource bound to a different Provider, hand-declared and verified structurally only (target registered, field types compatible) — never against Snapshot truth — and visibly marked unverified in build output. | Advanced setup for those who know what they are doing: cross-system administration without fabricating verification the source cannot provide. |

## Epic E5 — Workspace Shell

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-501 | Operators arrange Panels by splitting, tabbing, and floating them within one Workspace. | The multi-panel desktop experience is the product identity. |
| P0 | CAP-502 | Workspace layout persists locally and restores intact on relaunch; unresolvable Panels restore as inert placeholders identifying what they referenced. | Work is resumed, not restarted; restoration never crashes or drops the arrangement. |
| P0 | CAP-503 | A detail Panel opened from a List Follows that List's selection live; closing the source simply freezes the follower. | The master-detail payoff of multi-panel; causally obvious, never spooky. |
| P0 | CAP-504 | Edit Panels never Follow, and re-opening a Record already being edited focuses the existing Panel instead of duplicating it. | Losing unsaved edits to a moving selection — or to a twin Panel — is made structurally impossible. |
| P0 | CAP-505 | A Deep Link addresses one primary Panel — optionally carrying its typed Query — under the application's own declared URL scheme, and opens a fresh Workspace; personal layout is never encoded in a shared link. | Shareable addressing without leaking anyone's Workspace; scheme naming belongs to the Adopter's brand, not the framework. |
| P1 | CAP-506 | Deep Links are invokable from outside the application via operating-system link handling for the declared scheme. | Completes the addressing story; polish, not critical path. |
| P1 | CAP-507 | A declaration-driven navigation sidebar (Resources grouped by Adopter-declared sections) plus panel breadcrumbs orient the Operator across large Resource counts. | Flat sidebars collapse at envelope scale; grouping derives from owned declarations, not hidden config. |
| P1 | CAP-508 | A global command palette (keyboard-first) jumps to any Resource, Record, or action. | The desktop-native answer to scale; doubles as the power-user discoverability surface aligned with NFC-50. |
| P1 | CAP-509 | Light and dark appearance follow the operating system's setting, built on substrate theme tokens, with Adopter-declared accent color. | Table-stakes desktop polish; deep custom theming deliberately deferred until adopter feedback exists. |

## Epic E6 — Backend Integration

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-601 | All Backend communication passes through pluggable Provider contracts whose baseline assumes only elementary read and write operations, in singular and bulk forms. | Backend sovereignty: the core must be correct against an uncooperative Backend. |
| P0 | CAP-602 | Backend cooperation features — realtime change feeds, cursor paging, conditional updates, counting — are declared, opt-in Capabilities; their absence never degrades correctness, only richness. | Cooperation is rewarded, never required. |
| P0 | CAP-603 | A public Conformance Suite validates any Provider implementation against the contract. | Keeps first-party Providers unprivileged and makes third-party Providers provable. |
| P0 | CAP-604 | At least one verified Provider against a real, publicly reachable Backend ships with the first release. | The demo must run against reality, not only mocks. |
| P0 | CAP-607 | Each Resource binds to one of several declared Providers at build time (default binding covers single-Backend apps), each Provider bringing its own namespaced Schema Snapshot. | Real administration spans systems — internal database plus third-party APIs — and the seam is far cheaper to reserve now than to retrofit after the Registry assumes a singleton. |
| P0 | CAP-608 | File and Image fields store binary objects through a configurable Object Store addressed by its own contract (first-party implementation against the dominant de-facto object-storage standard); applications without a configured Object Store degrade those fields to reference-as-text inputs, never break. | Binary content is infrastructure beside, not inside, the data Provider; upload ordering and cleanup semantics are defined so committed Records never carry dangling references. |
| P1 | CAP-605 | Where the Backend offers a realtime change feed, open Panels stay fresh without manual refresh and recover safely after connection loss. | Freshness as a Capability; reconnection must not lie about missed changes. |
| P1 | CAP-606 | Collections reachable only through sequential paging remain fully browsable, with totals treated as optional. | Sovereignty applied to pagination physics; sequential Backends are first-class, just different. |

## Epic E7 — Identity & Access

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-701 | Applications run fully featured with no authentication configured. | Private-network and air-gapped deployments are first-class citizens. |
| P1 | CAP-702 | A first-class session kit: declaration-derived login View, sign-out, identity display in shell chrome, credentials held in platform secure storage, and mid-session expiry routed as an interruption that preserves Operator work. | Session death must be an interruption, not a data loss — promised since v0.1.0, now given concrete UX and storage mechanics. |
| P1 | CAP-703 | A minimal declarative permission vocabulary (per-Resource CRUD booleans, per-Field read/write masks) carried in advisory hints drives uniform hide/disable affordances; permission-aware UI is structurally distinct from authorization, and a Backend denial visibly corrects an over-permissive affordance. | Uniform behavior across Providers, scaffoldable and introspectable — while enforcement truth stays exclusively with the Backend. |

## Epic E8 — Collaboration Awareness

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P2 | CAP-801 | Transient collaboration signals (presence, notifications, broadcasts) can surface in Views but are never persisted into authoritative data. | Ambient awareness without polluting the source of truth. |

## Epic E9 — Actions & Extensibility

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P1 | CAP-901 | Operators select multiple Records and apply bulk actions, with selection surviving list refreshes; bulk operations ride contract-level bulk methods whose Providers loop internally when the Backend offers none. | Bulk correction is a staple admin workflow; the data layer must not make it N network calls disguised as one action. |
| P1 | CAP-902 | Adopters declare custom Resource-level actions with confirmation and input-collection flows. | Every real admin has verbs beyond CRUD. |
| P1 | CAP-903 | Adopters override any single input widget or Projection entry without leaving the declaration model. | Contained escape hatch: divergence is one explicit block, not a fork. |

## Epic E10 — Adoption & Showcase

| Priority | ID | Capability | Rationale |
| :--- | :--- | :--- | :--- |
| P0 | CAP-1001 | A public showcase application demonstrates every P0 capability against the verified Backend(s). | For an open-source, experience-led product, the demo *is* the argument. |
| P0 | CAP-1002 | An Adopter reaches a working admin over their own Schema Source, from fresh install, in under 30 minutes by following the getting-started path. | Time-to-first-success is the adoption funnel's only gate that matters. |
| P1 | CAP-1003 | A Contributor can author a new Provider and prove it correct using only public documentation and the Conformance Suite. | Ecosystem growth without core-team bottleneck. |
