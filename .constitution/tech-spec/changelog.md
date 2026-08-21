# Stage 3 Changelog — `.constitution/tech-spec/`

## v0.2.0 — 2026-08-21

Realign Evolution pass over prd v0.2.0 / architecture v0.2.0 (rulings: `../reports/2026-08-21-interview-realign.md`).

### Added
- **ADR-011** superseding ADR-004: substrate family tracked via git at lockfile-frozen revs, dual-location manifests, deliberate refreeze PRs, publish gate + registry-resolution CI lane; NFC-11 hermeticity preserved via committed lockfile + `cargo vendor`.
- **Contract `object_store.rs`:** ObjectStore port (put/get/delete), ObjectRef/ObjectError, object-first dispatch ordering with best-effort cleanup semantics (PB-2).
- **Contract `deep_link.md`:** adopter-declared scheme grammar; readable path+query serialization of typed Queries via snapshot-canonical field names; Decimal values serialized exactly.
- **Builder surface:** `provider()` binding key, `undo_window()`, `many_to_many()`, cross-provider relation acceptance with UNVERIFIED markers, `FormBuilder` section/columns layout primitives, closed declarative `FieldRule` set alongside closures, `quick_edit()` for simple widgets, expanded Widget catalog (DecimalNumber, DateTime, Time, Password, Email, Url, RichText, Markdown, File/ImageUpload).
- **Snapshot format 2:** `FieldKind` scalar taxonomy (Decimal exact; TimestampNaive/TimestampAware distinction), `self_ref` relation flags, Junction candidates, per-binding namespace banner.
- **BOM:** MinIO in showcase compose (ADR-008 amendment); ICU4X formatting family, `keyring`, and `rust_decimal` added as *(pin at planning)* entries; bulk methods + typed PermissionHints vocabulary in `provider.rs`.

### Changed
- **Compatibility Policy §3 acknowledgments:** the bulk methods are additive pre-release (no external consumers yet; loop-fallback documented in-contract); the `create()`/`edit()` builder signatures changed from bare projections to FormBuilder closures — both land before any registry publish, so no migration path is owed beyond release notes; each carries a paired conformance-suite obligation (RSK-07) to be implemented with Epic B.
- ADR-003 annotated: HttpClient source verdict deferred to the substrate re-pin ticket.
- Compatibility Policy §1 rewritten around rev-freezing; dependency-direction rule updated (git-family policy; NFC-11 note); error-currency rule extended to the object-store taxonomy.
- workspace-layout invariants updated: column masks, Saved Query bookmarks, Drafts are stored payload classes (operator input, not wholesale Record caches).

## v0.1.1 — 2026-07-10

Epic A execution updates (single tech-spec bump for the whole Epic A PR).

- **Substrate BOM correction (GA-A001):** removed the non-existent `gpui_platform = "=0.2.2"` from the substrate family in `stack.md`, `adrs/ADR-004-exact-pin-substrate.md`, and the Compatibility Policy. Live crates.io verification (2026-07-10) shows the crate returns 404 under every name/version and is absent from `gpui 0.2.2`'s dependency graph. GPUI selects its platform backend via **feature flags on `gpui`** (`wayland`/`x11` default-on for Linux, `metal` for macOS), so no sibling crate — and no version substitution — is required. The substrate family is now three crates: `gpui`, `gpui-component`, `gpui_http_client`. All other pins re-confirmed resolvable on 2026-07-10.
- **Licensing → MIT only (GA-A002):** changed the License line in `stack.md` (now `MIT`) and the repository-layout license files in `guidelines.md` (`LICENSE-MIT` only) from `MIT OR Apache-2.0`, tracking the operator's updated preference (`prd/` v0.1.1). The `LICENSE-APACHE` file and per-crate Apache symlinks were removed.
- **Developer environment + dependency-direction wording (GA-A001, refined during PR review):** added a "Developer Environment" section to `guidelines.md` recording the devenv (Nix) shell — pinned Rust 1.95.0 toolchain plus GPUI's native build dependencies. Clarified that `rust-toolchain.toml` is the source of truth for non-Nix contributors while CI mirrors the `1.95.0` pin explicitly (kept in lockstep, not auto-read), correcting the earlier "source of truth for CI" phrasing. Also noted in the dependency-direction rule that `provider-supabase`/`conformance` legitimately depend on `gpui_http_client` solely to name the injected `HttpClient` trait (ADR-003), which is not a boundary breach.

## v0.1.0 — 2026-07-09

Initial implementation constitution, derived from `.constitution/prd/` v0.1.0 and `.constitution/architecture/` v0.1.0. All version pins verified against crates.io/upstream source on this date.

- Created `stack.md`: Rust 1.95.0 (matches substrate lineage); exact-pinned substrate family (`gpui =0.2.2`, `gpui_platform =0.2.2`, `gpui-component =0.5.1`, `gpui_http_client =0.2.2` — published name verified via gpui's registry dependency graph); runtime/toolchain/testing BOM with verified versions; GitHub Actions (Linux+macOS), compose showcase backend, release-plz, `MIT OR Apache-2.0`; five-point Compatibility Policy.
- Created `guidelines.md`: cargo workspace layout (facade + core/macros/cli/ui/conformance/provider-supabase + showcase + backends), enforced dependency directions, lint/format/docs/unsafe rules, glossary naming discipline, testing expectations (deterministic state-machine suites, shared frontend fixture corpus, conformance-first, perf trend gates), tracing span contract, migration rules.
- Created `adrs/` (10): committed Snapshot module; typed builders canonical (three-persona interview decision); async-fn Provider + injected HttpClient (+ owned reqwest adapter); exact-pin substrate policy; Replica LWW/overlay/staged-undo/immutable records; pagination capability split; standalone `gpui-admin` binary; compose showcase backend; sparse patch + previous-state (+ ConditionalUpdate capability); versioned-JSON layout persistence.
- Created `data-models/`: `schema-snapshot.rs` (+ companion) — normative B2 format with field metadata, server-owned classification, relations, format marker; `workspace-layout.schema.json` (+ companion) — R9 envelope, JSON Schema 2020-12, total-restoration rules; `showcase-backend.sql` — PostgreSQL 17 DDL doubling as fixture, conformance target, and demo dataset.
- Created `contracts/`: `provider.rs` — Resource/FieldIdent traits, typed query model (closed operator set + first-class Search + extension escape hatch), Window/pagination types, `DataError`/`MutationError` taxonomy, `DataProvider` (async fn in trait), `Capabilities`, `SyncProvider`, `AuthProvider` + `NoAuth`; `resource_builder.rs` — the canonical authoring surface with normative Scaffolder-output example; `cli.md` — full command tree with exit codes and behavioral guarantees.

Decision provenance: Stage 3 interview 2026-07-09 (typed-builders-canonical via three-persona analysis; all other physical choices from the session's verified decision ledger and unobjected defaults).
