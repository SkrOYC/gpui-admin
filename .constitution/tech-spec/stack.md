# Stack — Bill of Materials

## 0. Version

**v0.2.0** — see [`changelog.md`](./changelog.md).

All versions below verified against crates.io / upstream source on 2026-07-09; substrate sourcing re-decided 2026-08-21 (ADR-011: git-frozen revs). New entries marked *(pin at planning)* are committed at ticket time, not invented here.

## Toolchain

| Component | Pin | Justification |
| :--- | :--- | :--- |
| Rust | `1.95.0` via `rust-toolchain.toml` (channel, rustfmt, clippy, rust-analyzer) | Matches the substrate lineage (Zed pins 1.95.0). MSRV = pinned toolchain; no older-compiler support claimed. |
| Edition | 2024 | Current edition; async-fn-in-trait and RPITIT required by the Provider contract (ADR-003). |

## Substrate (git-frozen family — ADR-011, supersedes ADR-004)

Every family member is tracked **via git at a lockfile-frozen rev** with a dual-location manifest (`version` fallback + `git` source; local builds resolve from git, publishes from the registry when releases satisfy the declared versions). The family moves only via a deliberate bump PR that refreezes all revs together and re-exercises the two integration seams.

| Crate | Source | Fallback version | Role (maps to architecture) |
| :--- | :--- | :--- | :--- |
| `gpui` | zed-industries/zed @ frozen rev | `0.2.x` | Reactive runtime: entities, observation, tasks, windows (R2 observation surface, R5 windows). Platform backends via its feature flags (`wayland`/`x11` Linux default-on, macOS metal). |
| `gpui-platform` | zed-industries/zed @ frozen rev | n/a (unpublished) | Platform backend crates consumed transitively by upstream's own build; never referenced directly. |
| `gpui_macros` | zed-industries/zed @ frozen rev | n/a | Transitively required by gpui main. |
| `gpui-component` | longbridge/gpui-component @ frozen rev | `0.5.x` | Widget layer: `DataTable`/cell selection/Tab nav (R6 lists, Quick Edit), `InputState` family (R6 forms), `DockArea`/`PanelRegistry` + `invalid_panel` (R5 shell, RSK-08 restoration). Brings `gpui-base`. |
| `gpui_http_client` OR zed-main `http_client` | verdict due in the substrate re-pin ticket | `=0.2.2` | The injected `HttpClient` trait for Providers (ADR-003); `FakeHttpClient` in tests. Source decided during re-pin; conformance suite follows whichever wins (RSK-07). |

> **History:** the registry-only exact-pin policy lived in ADR-004 until 2026-08-21. Its `gpui_platform = "=0.2.2"` entry had already been removed in v0.1.1 (crate unpublished; platform selection is gpui feature flags). ADR-011 supersedes the policy itself after live verification showed upstream builds against zed-git and the features we plan against exist only on unreleased main.

## Runtime & Provider dependencies

| Crate | Version | Role |
| :--- | :--- | :--- |
| `reqwest` | `0.13.4` | Backing for our thin `HttpClient` adapter (own crate module, dedicated tokio-runtime thread — the upstream `reqwest_client` pattern; no published equivalent exists, verified). |
| `serde` / `serde_json` | `1.0.228` / `1.0.150` | Record (de)serialization at the Gateway; layout persistence (R9). |
| `thiserror` | `2.0.18` | Error taxonomy types (`DataError`, `MutationError<R>`). |
| `tracing` / `tracing-subscriber` | `0.1.44` / `0.3.23` | R8 trace correlation (span per Operator action) + local structured logs. No network exporter is ever added (NFC-31). |
| `dirs` | `6.0.0` | Platform config directory for R9. |
| `schemars` | `1.2.1` | Generates the workspace-layout JSON Schema from the persisted types, keeping `data-models/workspace-layout.schema.json` honest. |
| ICU4X family (`icu_decimal`, `icu_datetime`, …) | *(pin at planning)* | Presentation-edge number/date formatting (CAP-210, P1): developer-configurable locale policy; never used for collation/sorting (backend-owned ordering). |
| `keyring` | *(pin at planning)* | Platform secure storage behind the P1 session kit (CAP-702); concrete mechanism for NFC-32. |

## Toolchain crates (CLI + derivation)

| Crate | Version | Role |
| :--- | :--- | :--- |
| `clap` | `4.6.1` | CLI command tree (`contracts/cli.md`), derive API. |
| `postgres` | `0.19.14` | B1 catalog frontend (synchronous driver — a CLI needs no async runtime). |
| `openapiv3` | `2.2.0` | B1 API-description frontend (OpenAPI 3.0/3.1 document model). |
| `syn` / `quote` / `proc-macro2` | `2.0.118` / `1.0.46` / `1.0.106` | B4 derivation proc-macros over committed `schema.rs` tokens. |
| `prettyplease` | `0.2.37` | B3 Scaffolder emits rustfmt-quality, human-owned builder code. |
| `rust_decimal` | *(pin at planning)* | Exact-decimal scalar behind `FieldKind::Decimal` (CAP-312): money never routed through binary floats — Snapshot types, FilterValue, and form parsing share it. |

## Testing

| Crate / facility | Version | Role |
| :--- | :--- | :--- |
| `#[gpui::test]` + `TestAppContext` | (with gpui) | Deterministic headless tests of R2/R3 state machines (verified upstream: deterministic dispatcher, `run_until_parked`). |
| `FakeHttpClient` (gpui_http_client `test-support`) | `=0.2.2` | Provider Gateway tests without network. |
| `insta` | `1.48.0` | Golden tests: Scaffolder output, Snapshot generation, both frontends against shared fixtures (RSK-04). |
| `anyhow` | `1.0.103` | Test/CLI binaries only — never in library public APIs (taxonomy types there). |

## Infrastructure & distribution

- **Object Store (P0, CAP-608):** first-party implementation against the S3-compatible API (operator preference appendix), exercised in CI against the compose MinIO; contract at `contracts/object_store.rs`.

- **CI:** GitHub Actions — `ubuntu-latest` + `macos-latest` (NFC-40: P0 platforms only; Windows lane added at P1). Jobs: fmt-check, clippy `-D warnings`, tests, showcase build. Conformance suite runs against the compose backend on the Linux lane.
- **Showcase backend:** repo-contained `docker compose` — PostgreSQL 17 + PostgREST 13 + **MinIO** (S3-compatible Object Store for CAP-608) with seeded schema (`data-models/showcase-backend.sql`) (ADR-008 as amended). Reproducible on any contributor machine; a hosted demo instance is a later, separate concern.
- **Release:** `release-plz` for versioning/publishing the crate family; `Cargo.lock` committed at workspace root.
- **License:** `MIT` (operator decision, 2026-07-10; supersedes the earlier `MIT OR Apache-2.0`). MIT is compatible with the Apache-2.0-licensed substrate.

## Compatibility Policy (binding)

1. **Substrate family (`gpui`, `gpui-platform`, `gpui_macros`, `gpui-component`, `gpui-base`): git-tracked at lockfile-frozen revs, dual-location manifests (ADR-011).** Upgrades are deliberate, reviewed acts: one PR refreezing all revs together and re-exercising the tracer-bullet seams. Publishes gate on upstream registry releases satisfying declared fallback versions, after a CI lane verifies registry-only resolution. Never floated, never auto-bumped (RSK-02).
2. **Commodity crates: caret ranges + committed `Cargo.lock`.** Renovate-style bumps allowed when CI is green.
3. **Public API SemVer:** pre-1.0, minor = breaking (documented plainly in README). The Provider contract and builder surface are the compatibility-critical APIs: any change to either requires the paired Conformance Suite change (RSK-07) and a migration note in the release notes.
4. **Snapshot format versioning:** `schema.rs` output carries a format marker; the derivation engine rejects Snapshots from an incompatible CLI generation with a "re-run capture" message — CLI and library versions may drift, but never silently.
5. **Layout persistence:** versioned JSON (`data-models/workspace-layout.md` migration rules); unknown versions/panels restore via the total-restoration rule (RSK-08), never crash.
