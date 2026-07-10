# Stage 4 Changelog — `.constitution/tasks/`

## v0.1.1 — 2026-07-10

Epic A execution progress (the epic remains **active** — see the still-open remote tail below).

- **Ticket-document corrections (Step 4, keep-specs-honest):** in `active/EPIC-A-foundation-oss-bootstrap.md`, corrected GA-A001's Gherkin to name the three real substrate crates (`gpui`, `gpui-component`, `gpui_http_client`) after the non-existent `gpui_platform` was removed upstream (tech-spec v0.1.1), and updated GA-A002's description/verification to MIT-only (operator decision, prd v0.1.1). Appended `Deviations & Justifications` blocks to GA-A001 (devenv committed as a build necessity + the substrate spec correction) and GA-A002 (per-crate `LICENSE-MIT` symlinks for self-contained crate packaging).
- **Local + registry deliverables landed:** GA-A001 (7-crate workspace, exact-pinned substrate, `Cargo.lock` committed, clean clippy `-D warnings`), GA-A002 (MIT license + README + per-crate metadata), and GA-A004 (all seven family names published to crates.io as `0.0.1` placeholders, each README linking the repository) are complete and independently reviewed. GA-A003's `.github/workflows/ci.yml` is committed and `actionlint`-clean.
- **Still open (deferred to the push/PR session by operator instruction):** GA-A003's acceptance is remote — `gh run watch` green on `ubuntu-latest` + `macos-latest` — and requires the branch to be pushed; GA-A005 (repo public + `master` branch protection) requires the push plus repo-admin access. Because those two tickets are not yet satisfiable, the epic is **not** archived and `critical-path.md` story points are **unchanged** (121 pts / 6 epics). Formal close (archive + critical-path recompute) happens once CI is verified green and protection is enabled.

## v0.1.0 — 2026-07-09

Initial execution constitution, derived from `.constitution/tech-spec/` v0.1.0 (with `architecture/` and `prd/` v0.1.0 upstream).

- Created `critical-path.md`: 121 active story points across 36 tickets in 6 epics; dependency spine; build-order graph; Phase-1 phasing with epics G–K outlined-and-deferred (interview decision: Phase 1 active, rest re-planned via evolution passes).
- Created `active/` epics:
  - **EPIC-A foundation-oss-bootstrap** (5 tickets, 9 pts): workspace + pins, licenses/README ("gpui-admin contributors"), CI (Linux+macOS), crates.io placeholder publishes (interview decision), public repo on personal account with protected `master` (interview decisions — `master` retained, branch-per-epic PRs).
  - **EPIC-B provider-contract-conformance** (6 tickets, 18 pts): contract types, error taxonomy, provider/auth traits + NoAuth, HTTP adapter, conformance skeleton, mock provider passing both pagination modes.
  - **EPIC-C tracer-bullet-vertical** (5 tickets, 21 pts): hand-written resource + minimal replica, windowed-table seam, form/mutation/undo seam, dock persistence seam, Spike GA-C005 (10M scroll, RSK-03).
  - **EPIC-D replica-mutation-core** (7 tickets, 26 pts): record store, query layer (generations/ranges), overlay lifecycle (staged/undo/FIFO/settlement), field-aware invalidation, freshness/observation guards, feed reconciliation + reconnect, adversarial interleaving suite (RSK-01).
  - **EPIC-E toolchain-mvp** (6 tickets, 20 pts): snapshot emitter, CLI skeleton, compose fixture, both introspection frontends, byte-equivalence corpus (RSK-04).
  - **EPIC-F derivation-authoring** (7 tickets, 27 pts): builder surface (ADR-002), derivation macros (exit criterion: regenerate the tracer identically), drift diagnostics (NFC-12), Scaffolder, `check`, Spike GA-F006 (build budget, RSK-05), boot assertion sweep.
- Created `.constitution/spikes/SPK-C005.md` and `SPK-F006.md` placeholders from the master template.
- Created `completed/` (empty).

Decision provenance: Stage 4 interview 2026-07-09 (Phase-1 phasing; GA- IDs; placeholder crates.io publishes now; public repo on personal account; branch-per-epic on retained `master`; "gpui-admin contributors" copyright).
