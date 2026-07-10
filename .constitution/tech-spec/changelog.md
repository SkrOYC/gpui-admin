# Stage 3 Changelog — `.constitution/tech-spec/`

## v0.1.0 — 2026-07-09

Initial implementation constitution, derived from `.constitution/prd/` v0.1.0 and `.constitution/architecture/` v0.1.0. All version pins verified against crates.io/upstream source on this date.

- Created `stack.md`: Rust 1.95.0 (matches substrate lineage); exact-pinned substrate family (`gpui =0.2.2`, `gpui_platform =0.2.2`, `gpui-component =0.5.1`, `gpui_http_client =0.2.2` — published name verified via gpui's registry dependency graph); runtime/toolchain/testing BOM with verified versions; GitHub Actions (Linux+macOS), compose showcase backend, release-plz, `MIT OR Apache-2.0`; five-point Compatibility Policy.
- Created `guidelines.md`: cargo workspace layout (facade + core/macros/cli/ui/conformance/provider-supabase + showcase + backends), enforced dependency directions, lint/format/docs/unsafe rules, glossary naming discipline, testing expectations (deterministic state-machine suites, shared frontend fixture corpus, conformance-first, perf trend gates), tracing span contract, migration rules.
- Created `adrs/` (10): committed Snapshot module; typed builders canonical (three-persona interview decision); async-fn Provider + injected HttpClient (+ owned reqwest adapter); exact-pin substrate policy; Replica LWW/overlay/staged-undo/immutable records; pagination capability split; standalone `gpui-admin` binary; compose showcase backend; sparse patch + previous-state (+ ConditionalUpdate capability); versioned-JSON layout persistence.
- Created `data-models/`: `schema-snapshot.rs` (+ companion) — normative B2 format with field metadata, server-owned classification, relations, format marker; `workspace-layout.schema.json` (+ companion) — R9 envelope, JSON Schema 2020-12, total-restoration rules; `showcase-backend.sql` — PostgreSQL 17 DDL doubling as fixture, conformance target, and demo dataset.
- Created `contracts/`: `provider.rs` — Resource/FieldIdent traits, typed query model (closed operator set + first-class Search + extension escape hatch), Window/pagination types, `DataError`/`MutationError` taxonomy, `DataProvider` (async fn in trait), `Capabilities`, `SyncProvider`, `AuthProvider` + `NoAuth`; `resource_builder.rs` — the canonical authoring surface with normative Scaffolder-output example; `cli.md` — full command tree with exit codes and behavioral guarantees.

Decision provenance: Stage 3 interview 2026-07-09 (typed-builders-canonical via three-persona analysis; all other physical choices from the session's verified decision ledger and unobjected defaults).
