# Stage 1 Changelog — `.constitution/prd/`

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
