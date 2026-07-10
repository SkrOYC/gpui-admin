# Stage 3 Changelog — `.constitution/tech-spec/`

## v0.1.2 — 2026-07-10

Patch: licensing changed to MIT-only, tracking the operator's updated preference (`prd/` v0.1.1). Updated the License line in `stack.md` (now `MIT`) and the repository-layout license files in `guidelines.md` (`LICENSE-MIT` only). The `LICENSE-APACHE` file and per-crate Apache symlinks are removed during Epic A (GA-A002). No other stack entries changed.

## v0.1.1 — 2026-07-10

Patch: substrate BOM correction discovered during Epic A (GA-A001) execution.

- Removed the non-existent `gpui_platform = "=0.2.2"` from the substrate family in `stack.md`, `adrs/ADR-004-exact-pin-substrate.md`, and the Compatibility Policy. Live crates.io verification (2026-07-10) shows the crate returns 404 under every name/version and is absent from `gpui 0.2.2`'s dependency graph. GPUI selects its platform backend via **feature flags on `gpui`** (`wayland`/`x11` default-on for Linux, `metal` for macOS), so no sibling crate — and no version substitution — is required. The substrate family is now three crates: `gpui`, `gpui-component`, `gpui_http_client`.
- No other pins changed; all remaining BOM entries re-confirmed resolvable on 2026-07-10.

## v0.1.0 — 2026-07-09

Initial implementation constitution, derived from `.constitution/prd/` v0.1.0 and `.constitution/architecture/` v0.1.0. All version pins verified against crates.io/upstream source on this date.

- Created `stack.md`: Rust 1.95.0 (matches substrate lineage); exact-pinned substrate family (`gpui =0.2.2`, `gpui_platform =0.2.2`, `gpui-component =0.5.1`, `gpui_http_client =0.2.2` — published name verified via gpui's registry dependency graph); runtime/toolchain/testing BOM with verified versions; GitHub Actions (Linux+macOS), compose showcase backend, release-plz, `MIT OR Apache-2.0`; five-point Compatibility Policy.
- Created `guidelines.md`: cargo workspace layout (facade + core/macros/cli/ui/conformance/provider-supabase + showcase + backends), enforced dependency directions, lint/format/docs/unsafe rules, glossary naming discipline, testing expectations (deterministic state-machine suites, shared frontend fixture corpus, conformance-first, perf trend gates), tracing span contract, migration rules.
- Created `adrs/` (10): committed Snapshot module; typed builders canonical (three-persona interview decision); async-fn Provider + injected HttpClient (+ owned reqwest adapter); exact-pin substrate policy; Replica LWW/overlay/staged-undo/immutable records; pagination capability split; standalone `gpui-admin` binary; compose showcase backend; sparse patch + previous-state (+ ConditionalUpdate capability); versioned-JSON layout persistence.
- Created `data-models/`: `schema-snapshot.rs` (+ companion) — normative B2 format with field metadata, server-owned classification, relations, format marker; `workspace-layout.schema.json` (+ companion) — R9 envelope, JSON Schema 2020-12, total-restoration rules; `showcase-backend.sql` — PostgreSQL 17 DDL doubling as fixture, conformance target, and demo dataset.
- Created `contracts/`: `provider.rs` — Resource/FieldIdent traits, typed query model (closed operator set + first-class Search + extension escape hatch), Window/pagination types, `DataError`/`MutationError` taxonomy, `DataProvider` (async fn in trait), `Capabilities`, `SyncProvider`, `AuthProvider` + `NoAuth`; `resource_builder.rs` — the canonical authoring surface with normative Scaffolder-output example; `cli.md` — full command tree with exit codes and behavioral guarantees.

Decision provenance: Stage 3 interview 2026-07-09 (typed-builders-canonical via three-persona analysis; all other physical choices from the session's verified decision ledger and unobjected defaults).
