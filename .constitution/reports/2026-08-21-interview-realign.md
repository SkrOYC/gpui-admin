# Interview Record — Realign (2026-08-21)

**Driver:** Operator judged the constitution (all layers at v0.1.x) insufficient in depth and breadth for a react-admin-level alternative. Full-sweep calibration agreed (~25–30 questions). Reference classes grounded against live sources: react-admin `docs/` on master (gh api, marmelab/react-admin @ 26.9k stars) and Google AppSheet's documented view/input surface. Substrate facts verified against `longbridge/gpui-component` (v0.5.1 tag and main) and zed-industries manifests via gh api. Codebase state at interview: Epic A closed (commit `a43092d`), crates are documentation-only stubs awaiting Epics B–F; working tree had an uncommitted `.envrc` change which the operator directed deleted during the session.

Each ruling records the decision and the operator's stated reasoning. Downstream stages must honor these without re-asking.

---

## Governing principle (operator-stated, applies everywhere)

> **gpui-admin is a library, not a product. All configurability belongs to the Adopter-developer through declarations and code.**

Stated while ruling Q18; applied as tiebreaker throughout. It confirms/refines prior rulings (theming accent = adopter-declared; undo window = developer-configured constant, never a user setting; formatting = developer-facing configuration point) and ruled Q22 outright.

## Theme A — Data lifecycle & write safety

- **A-Q1 Soft delete: OUT** → new `out-of-scope/soft-delete.md`. Operator: "a user owns the responsibility of deletes under clear 'are you sure?' confirmation." **Derived requirement (operator's own words, currently mandated nowhere):** new P0 capability — destructive actions require explicit confirmation before dispatch; its relationship to the Undo Window (confirm ≠ undo; window applies after confirmation) must be documented.
- **A-Q2 Audit log & record revisions: BOTH OUT** → `out-of-scope/audit-log.md`, `out-of-scope/record-revisions.md`. Operator: "for now; will be judged in the future" → carried as **OD-01**.
- **A-Q3 Concurrent editing: KEEP current design** (mid-edit drift warning CAP-306 + ConditionalUpdate capability CAP-307). Locking evaluated and rejected for now. Operator: "will revisit later when things stabilize" → carried as **OD-02**.
- **A-Q4 Bulk operations: IN at contract level, NOW** (pre-Epic-B). Add `create_many`, `update_many(ids, patch)`, `delete_many(ids)` to `contracts/provider.rs`; Providers without native support loop internally (sovereignty preserved); paired conformance obligations (RSK-07 atomicity); GA-B001/B005/B006 scope growth.

## Theme B — Views & browsing

- **B-Q5 Column personalization =(b):** Operator show/hide + per-list sort override persisted locally in R9, as a *mask* over the declared Projection (declaration = default, operator = decoration; precedence rule explicit). Drifted fields restore via the inert-placeholder philosophy of RSK-08.
- **B-Q6 Saved queries =(c):** Shareable saved queries via Deep Links, subsuming local bookmarks (R9 keeps a bookmark list of created/received queries; sharing rides Deep Links). Consequence accepted: the Deep Link payload format must be specified before Phase 1 closes (see PB-3).
- **B-Q7 Grid editing =(a) STAGED:** P1 ships single-field quick-edit (Switch, EnumSelect, Number, Text) through the *existing* R3 staged lifecycle with row-banner errors, explicitly NOT an Edit Panel (no CAP-504 focus-not-duplicate interaction). Full multi-cell grid editing is P2 behind a feasibility spike (editor-state survival under virtualization + batch-undo semantics). Evidence recorded: gpui-component has zero editing machinery even on `main`; `uniform_list` rebuilds rows every frame, so any editor state must live outside elements keyed by `(record_id, field)`; cell selection/Tab nav exist on main only (unreleased).
- **B-Q8 Prev/next navigation =(b):** Show/Edit panels navigate the source List's query ordering *as of open time*; disabled at frontiers; honest SequentialCursor degradation (forward-only past the loaded frontier); **navigation blocked while the Edit panel is dirty** (CAP-504 interaction rule, proposed by interviewer, accepted by choice of option).

## Substrate sourcing (major structural ruling)

- **Ruling: family-wide git tracking via dual-location manifests, lockfile-frozen revs.** Supersedes ADR-004's registry-only/no-git-deps policy. Operator rationale: "just like gpui directly is recommended at this stage."
- Verified evidence: gpui-component `main` itself builds against `gpui`, `gpui_platform`, `gpui_macros` from zed-industries/zed **via git**, adds a new `gpui-base` crate, workspace version 0.5.2-dev; mixing sources (crates.io gpui 0.2.2 + component-from-git) yields duplicate-type non-compilation, so family-wide is the coherent shape. Cargo Book sanctions the dual-location pattern (`version` + `git`; publishes use the registry version — crates.io rejects git-only dependencies except dev-deps).
- Additional verified facts folded into rulings: `DataTable` landed on main 2026-06-19 and **no release contains it** (stack.md's DataTable claim was false at the 0.5.1 pin — resolved by this re-sourcing rather than doc edits); v0.5.1's table delegate hooks (`visible_rows_changed`, `load_more`) do match ADR-006's citations; the 0.5.1 dock ships `invalid_panel.rs` (direct substrate support for RSK-08 placeholder restoration).
- Derived deltas: new ADR superseding ADR-004 (rev-pinning discipline: freeze → deliberate bump PR → paired seam re-verification); Compatibility Policy §1 rewritten; NFC-11 annotated (hermeticity preserved via committed lockfile + `cargo vendor`); RSK-02 mitigation rewritten (churn surface = zed-main, bounded by rev-freeze); publish gate (publish only when a satisfying registry release exists; CI lane verifying registry-resolution compatibility); **new pre-Epic-C ticket**: re-pin to frozen revs, re-exercise windowed-table + dock seams, deliver verdict on `gpui_http_client` vs zed-main's `http_client`/`reqwest_client` (possible ADR-003 amendment).

## Theme C — Forms & inputs

- **C-Q9 Widget catalog: full-parity direction, split landing:**
  - P0 catalog: Text, TextArea, **decimal-safe Number** (a Decimal scalar enters the Snapshot type system now — `FilterValue`'s F64-only silently corrupts money), Switch, Date, **DateTime + Time**, Password, Email/Url validated variants, EnumSelect, RelationSelect, **RichText/Markdown** (client-side only), display-side Image/Url renderers.
  - **ObjectStore as an independent port** (not a data-Provider capability): `put/get/delete` signatures reserved in the contract pass pre-Epic-B; **S3-compatible implementation committed as P0** (operator: "it will be S3 as our first priority"); MinIO joins the showcase compose stack (ADR-008 amendment); runtime capability-gating retained (unconfigured apps degrade to URL-text fields).
  - Constitutional ripples: NFC-31 rewording ("configured Backend **and configured object storage**"), containers.md names ObjectStore as a third egress boundary, credentials route through NFC-32.
- **PB-2 Upload ordering =(b): object-first inside the staged lifecycle.** Upload happens at dispatch before the record write; record rejection or Undo-Window revocation triggers best-effort object deletion, logged to diagnostics on failure. Documented scoped exception: cleanup deletes are compensating actions on the *object store* (infrastructure hygiene), not replica writes — the subtraction-only rollback invariant (ADR-005) governs client truth and remains intact.
- **C-Q10 Form layout =(b): lightweight declaration-level primitives** — `section("…")` / `columns(n)` groupings inside `create()`/`edit()` projections; Scaffolder emits sensible defaults; freeform composition excluded (consistent with no-code-builder rejection).
- **C-Q11 Autosave =(i): local draft persistence, first-class P0.** Debounced dirty-form state to R9 keyed by `(resource, record_id)`; restored verbatim on relaunch *alongside* the mid-edit drift warning; cleared only on confirmed save/discarded; crash exits covered. Server-dispatch autosave **permanently rejected** (blur-triggered dispatches destroy Undo Window semantics, flood FIFO, multiply conflicts) → document in out-of-scope. New drafts data-model artifact; mutation-lifecycle flow updated; named as the concrete mechanism fulfilling NFC-21.
- **C-Q12 Validation vocabulary =(b): declarative core rules + closures as escape hatch.** Closed set: required, enum-membership, min/max, length caps, pattern. Derivable rules Scaffolder-emitted from Snapshot metadata; documented precedence: backend validation always overrides client rules (CAP-305 posture); GA-F004 golden tests cover emitted rules.

## Theme D — Appearance, accessibility, envelope

- **D-Q13 Theming =(b): light + dark following OS appearance** on substrate theme tokens; Adopter accent-color override only. Deep theming API deliberately deferred.
- **D-Q14 Screen readers =(a): explicit out-of-scope** with documented trigger condition (GPUI accessibility maturity). Keyboard operability (NFC-50) unchanged.
- **D-Q15 Memory ceiling =(b): new NFC-05** — warm working-set ceiling at the browsing envelope, Planguage form, initial Goal ≈1 GB RSS with a 10M-record Resource under active browsing on reference hardware, **target calibrated by a post-SPK-C005 measurement ticket** (evidence before commitment).
- **D-Q16 Navigation chrome =(b):** declaration-driven grouped sidebar + panel breadcrumbs + global command palette (jump-to-resource/record/action, keyboard-first, NFC-50 aligned) at P1 polish tier.

## Theme E — Meaning-correctness

- **E-Q17 Timezones =(b): type-faithful rendering.** Absolute instants (`timestamptz`) render in machine zone by default with Adopter per-projection override; naive timestamps and date-only values render **verbatim, never converted**, their undeclared-zone nature marked in forms. Snapshot format must distinguish `timestamp` / `timestamptz` / `date` / `time`.
- **E-Q18 Formatting =(b) + governing principle:** presentation-edge number/date formatting as a developer-facing configuration point (system-locale documented default; canonical-fixed one declaration away); string collation stays backend-owned (ADR-005: ordering is never client-faked).
- **E-Q19 Auth surface =(b): first-class session kit** — login View archetype (declaration-derived), sign-out, identity display in shell chrome, expiry interruption preserving input, credentials via **platform keychain** (concrete NFC-32 mechanism). Protocols stay Provider-authored; first-party ships form-credential auth; OIDC/OAuth are Contributor territory.
- **E-Q20 Permission vocabulary =(b): minimal declarative shape** inside `PermissionHints` — per-Resource CRUD booleans + per-Field read/write masks; framework derives hide/disable affordances uniformly; hints remain strictly advisory (NFC-30 untouched).

## Theme F — Toolchain breadth & envelope

- **F-Q21 Multi-backend =(b): per-Resource Provider binding** declared at build time (optional provider key on `ResourceDecl`); Registry routes to the correct Gateway; each Provider carries its own namespaced Schema Snapshot; single-provider apps use the default binding; mechanism ships P0, multi-backend demonstrations come later. Operator confirmed the intent: "a single table could come from a different source."
- **F-Q22 Update checks & error reporting =(a): nothing, ever.** Distribution/update/error-reporting are wholly Adopter concerns; `tracing` subscribers are the only seam (any exporter attachable by the Adopter).
- **F-Q23 Undo Window =(b): framework default (~5 s) + per-Resource builder override** `undo_window(Duration)`; Scaffolder materializes the default explicitly (infer-then-own visibility).
- **F-Q24 Search without backend capability =(b): labeled client-side fallback** — case-insensitive substring across loaded windows, visibly labeled "searching loaded rows"; honest partial coverage, zero backend demands.

## Phase B resolutions

- **PB-1 Cross-provider relations =(b): allowed, hand-declared, structurally verified only.** Builder permits relations whose target binds to a different Provider; derivation verifies target registration + field-type compatibility but NOT against Snapshot truth; the relation carries a visible "unverified" marker (weak-truth precedent from the OpenAPI frontend). Embedded related lists run through the target's Gateway with existing contained-failure/degraded handling. Operator framing, recorded: *"an advanced setup for those who know what they are doing."*
- **PB-2:** see C-Q9/PB-2 above (object-first staged lifecycle).
- **PB-3 Deep Link format =(a): Adopter-declared custom scheme.** Readable path + query params (typed queries serialize losslessly via `FieldIdent::name()`); no base64 blobs; reviewable/pasteable/diffable. OS-handler registration is the Adopter's packaging concern (CAP-506, P1).
- **Q25 Many-to-many =(b): detected sugar over junction resources.** Snapshot introspection flags dual-FK tables as candidate junctions; builder gains `many_to_many(via: Junction)` deriving filtered-query machinery (semantically a typed `has_many` with join predicate — no new Replica state machinery); pick-one inputs gain declared multi-select behavior; Scaffolder emits both the junction Resource and the convenience declarations; escape hatch for non-pure junctions (attribute-bearing rows).
- **Q26 Trees =(b): recognition + minimal display at P2.** Self-referencing FKs flagged in Snapshot; List/Show render indentation + root→leaf breadcrumb via depth-limited eager fetching (sovereignty-clean bounded recursion). No drag-reparent, no lazy expansion; CAP-206 umbrella remains the home for fuller tree views.

## Process findings (to repair during the passes)

1. `prd/vision.md` archetype block is prose, not the structured Primary/Secondary/Confidence/Rationale form Stage 1's contract requires.
2. `architecture/changelog.md` lacks the "reviewed upstream delta; no changes required" entry for prd v0.1.1.
3. `tech-spec/stack.md` claimed `DataTable` exists in the pinned 0.5.1 — false at pin time; resolved by the substrate re-sourcing ruling (specs become true as-written against frozen main revs).
4. `.envrc` deleted at operator direction (uncommitted deletion pending in working tree).

## Open decisions

See [`2026-08-21-open-decisions.md`](./2026-08-21-open-decisions.md) — OD-01 (audit/revisions reconsideration), OD-02 (concurrency strengthening revisit).
