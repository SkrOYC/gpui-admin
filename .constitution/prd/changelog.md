# Stage 1 Changelog — `.constitution/prd/`

## v0.2.0 — 2026-08-21

Realign Evolution pass (interview 2026-08-21; rulings in `../reports/2026-08-21-interview-realign.md`). Goal: depth/breadth parity with the react-admin class of tools, per operator direction.

### Added
- **Archetype block** restructured to the pipeline's Primary/Secondary/Confidence/Rationale form (Primary: Library/SDK; Secondary: CLI/Tooling + System/Native).
- **Governing principle** stated in vision: a library, not a product — all configurability belongs to the Adopter-developer via declarations and code.
- **Glossary:** six canonical terms added (Object Store, Saved Query, Draft, Provider Binding, Junction, Quick Edit); Deep Link definition extended to carry typed Queries under an Adopter-declared scheme.
- **Capabilities:** CAP-211 (type-faithful temporal rendering, P0), CAP-308 (destructive-action confirmation, P0), CAP-309 (local Draft persistence, P0), CAP-404 (many-to-many over detected Junctions, P0), CAP-607 (per-Resource Provider Binding across multiple Backends, P0), CAP-608 (Object Store port with S3-compatible first-party implementation, P0), CAP-207/208/209/210 (column personalization, shareable Saved Queries, prev/next navigation, presentation-edge locale formatting, P1), CAP-310 (Quick Edit, P1), CAP-405 (cross-provider Relations, structurally verified only, P1), CAP-507/508/509 (sidebar+breadcrumbs, command palette, OS-following theming, P1), CAP-311 (full grid editing behind feasibility spike, P2), CAP-212 (tree hierarchy cues, P2). Amended CAP-101/103/203/301/303/601/702/703/901 for bulk forms, declarative validation rules, search fallback, Undo Window configurability, session kit, and permission vocabulary; smaller amendments to CAP-202 (multi-column sort expressibility), CAP-305 (backend precedence over client rules), CAP-502 (placeholder-panel restoration clause), CAP-505 (declared scheme + typed Query payload), CAP-1001 (plural Backends).
- **Constraints:** NFC-05 warm memory envelope (≤1 GB goal at 10M-record browsing envelope, calibrated by post-spike measurement); NFC-31 reworded to include the configured Object Store and to fix distribution/update/error-reporting as Adopter concerns.
- **Out-of-scope records:** soft-delete, audit-log, record-revisions, record-locking (rejected-for-now), screen-readers (deferred w/ trigger), ops-update-error-reporting, server-autosave (rejected-permanently).
- **Domain model:** Object Store external system, Junction and Provider Binding concepts, multi-sovereign and object-truth boundary statements.

### Changed
- Open decisions registered in `../reports/2026-08-21-open-decisions.md` (OD-01 audit/revisions reconsideration; OD-02 concurrency strengthening revisit) — downstream stages must not answer these silently.

## v0.1.1 — 2026-07-10

- Operator preference update: project licensing changed from `MIT OR Apache-2.0` to `MIT` only (`vision.md` Operator Preferences appendix). Propagated downstream to `tech-spec/` (v0.1.1) and applied during Epic A (GA-A002) execution: the `LICENSE-APACHE` file and per-crate Apache symlinks were removed, and every crate now declares `license = "MIT"`.

## v0.1.0 — 2026-07-09

Initial product constitution for the greenfield project.

- Created `vision.md`: Native-UX-led identity (professional desktop admin), supported by build-time correctness and backend sovereignty; OSS-from-day-one archetype (framework + offline toolchain → compiled native desktop applications); five JTBD; Operator Preferences appendix.
- Created `glossary.md`: 24 canonical terms with prohibited synonyms.
- Created `actors.md`: Adopter (primary), Operator, Contributor (secondary), Backend Owner (contextual non-cooperating actor).
- Created `capabilities.md`: 10 epics, 43 capabilities (30 at P0) graded to the "demo-complete P0" cut (typed CRUD, windowed browsing, forms, optimistic editing with Undo Window, Relations, Filters/Search, dock Workspace with Follow and persistence, Provider contract + Conformance Suite + one verified Provider, showcase app, 30-minute getting-started). P1: authentication, realtime freshness, sequential paging, bulk/custom actions, permission affordances, Windows platform. P2: presence, extra View archetypes, observed-data Snapshots.
- Created `constraints.md`: measurable NFCs — 60 fps windowed scrolling at 10M Records, <500 ms cold start, one-frame optimistic visibility, <30 s incremental rebuild, air-gapped builds, fault containment, input preservation, workspace restoration, no-telemetry default, Linux+macOS P0 / Windows P1, stated scale envelope, keyboard operability.
- Created `domain-model.md`: context and domain diagrams; one-way truth flows (build-time and run-time), personal Workspace boundary, declarative cooperation.
- Created `out-of-scope/`: eight rejection records (no-code builder, hosted service, web deployment, mobile, offline-first, full internationalization, built-in data export, client-side security).

Decision provenance: session interview 2026-07-09 (primary persona = OSS Adopter with neutral wording; vision spine = Native-UX-led; P0 cut = demo-complete; platforms = Linux+macOS P0/Windows P1) over the v4 pre-constitution architecture exploration.
