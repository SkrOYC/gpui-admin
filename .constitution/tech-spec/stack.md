# Stack — Bill of Materials

## 0. Version

**v0.1.1** — see [`changelog.md`](./changelog.md).

All versions below verified against crates.io / upstream source on 2026-07-09; substrate family re-verified 2026-07-10 (see v0.1.1 correction).

## Toolchain

| Component | Pin | Justification |
| :--- | :--- | :--- |
| Rust | `1.95.0` via `rust-toolchain.toml` (channel, rustfmt, clippy, rust-analyzer) | Matches the substrate lineage (Zed pins 1.95.0). MSRV = pinned toolchain; no older-compiler support claimed. |
| Edition | 2024 | Current edition; async-fn-in-trait and RPITIT required by the Provider contract (ADR-003). |

## Substrate (exact-pinned family — ADR-004)

| Crate | Pin | Role (maps to architecture) |
| :--- | :--- | :--- |
| `gpui` | `=0.2.2` | Reactive runtime: entities, observation, tasks, windows (R2 observation surface, R5 windows). crates.io publication verified; pre-1.0 with breaking-change warning → exact pin. Platform backends are **feature flags on this crate**, not a sibling crate — its default features `["font-kit", "wayland", "x11", "windows-manifest"]` already select the Linux (wayland + x11) backends; macOS uses the `metal`/system-framework path. |
| `gpui-component` | `=0.5.1` | Widget layer: `DataTable`/`VirtualList` (R6 windowed lists), `InputState` family (R6 forms), `DockArea`/`PanelRegistry` (R5 shell), notifications. Depends on crates.io `gpui ^0.2.2` — pure registry chain verified. Does **not** re-export gpui: both are direct deps, versions kept paired. |
| `gpui_http_client` | `=0.2.2` | The `HttpClient` trait injected into Providers (ADR-003); `FakeHttpClient` in tests. Published name of upstream `http_client`, verified via gpui 0.2.2's dependency graph. |

> **Correction (v0.1.1, GA-A001):** an earlier `gpui_platform = "=0.2.2"` substrate entry was removed. That crate does not exist on crates.io (404) and is not a dependency of `gpui 0.2.2`; on Linux the platform backend is selected via `gpui`'s own `wayland`/`x11` features (on by default). Verified live against the crates.io registry during Epic A execution.

## Runtime & Provider dependencies

| Crate | Version | Role |
| :--- | :--- | :--- |
| `reqwest` | `0.13.4` | Backing for our thin `HttpClient` adapter (own crate module, dedicated tokio-runtime thread — the upstream `reqwest_client` pattern; no published equivalent exists, verified). |
| `serde` / `serde_json` | `1.0.228` / `1.0.150` | Record (de)serialization at the Gateway; layout persistence (R9). |
| `thiserror` | `2.0.18` | Error taxonomy types (`DataError`, `MutationError<R>`). |
| `tracing` / `tracing-subscriber` | `0.1.44` / `0.3.23` | R8 trace correlation (span per Operator action) + local structured logs. No network exporter is ever added (NFC-31). |
| `dirs` | `6.0.0` | Platform config directory for R9. |
| `schemars` | `1.2.1` | Generates the workspace-layout JSON Schema from the persisted types, keeping `data-models/workspace-layout.schema.json` honest. |

## Toolchain crates (CLI + derivation)

| Crate | Version | Role |
| :--- | :--- | :--- |
| `clap` | `4.6.1` | CLI command tree (`contracts/cli.md`), derive API. |
| `postgres` | `0.19.14` | B1 catalog frontend (synchronous driver — a CLI needs no async runtime). |
| `openapiv3` | `2.2.0` | B1 API-description frontend (OpenAPI 3.0/3.1 document model). |
| `syn` / `quote` / `proc-macro2` | `2.0.118` / `1.0.46` / `1.0.106` | B4 derivation proc-macros over committed `schema.rs` tokens. |
| `prettyplease` | `0.2.37` | B3 Scaffolder emits rustfmt-quality, human-owned builder code. |

## Testing

| Crate / facility | Version | Role |
| :--- | :--- | :--- |
| `#[gpui::test]` + `TestAppContext` | (with gpui) | Deterministic headless tests of R2/R3 state machines (verified upstream: deterministic dispatcher, `run_until_parked`). |
| `FakeHttpClient` (gpui_http_client `test-support`) | `=0.2.2` | Provider Gateway tests without network. |
| `insta` | `1.48.0` | Golden tests: Scaffolder output, Snapshot generation, both frontends against shared fixtures (RSK-04). |
| `anyhow` | `1.0.103` | Test/CLI binaries only — never in library public APIs (taxonomy types there). |

## Infrastructure & distribution

- **CI:** GitHub Actions — `ubuntu-latest` + `macos-latest` (NFC-40: P0 platforms only; Windows lane added at P1). Jobs: fmt-check, clippy `-D warnings`, tests, showcase build. Conformance suite runs against the compose backend on the Linux lane.
- **Showcase backend:** repo-contained `docker compose` — PostgreSQL 17 + PostgREST 13 with seeded schema (`data-models/showcase-backend.sql`) (ADR-008). Reproducible on any contributor machine; a hosted demo instance is a later, separate concern.
- **Release:** `release-plz` for versioning/publishing the crate family; `Cargo.lock` committed at workspace root.
- **License:** `MIT OR Apache-2.0` (Rust convention; substrate is Apache-2.0-compatible).

## Compatibility Policy (binding)

1. **Substrate family (`gpui`, `gpui-component`, `gpui_http_client`): exact pins (`=x.y.z`).** Upgrades are deliberate, reviewed acts: one PR, pairing all three, with the tracer-bullet seams re-exercised. Never floated, never auto-bumped (RSK-02).
2. **Commodity crates: caret ranges + committed `Cargo.lock`.** Renovate-style bumps allowed when CI is green.
3. **Public API SemVer:** pre-1.0, minor = breaking (documented plainly in README). The Provider contract and builder surface are the compatibility-critical APIs: any change to either requires the paired Conformance Suite change (RSK-07) and a migration note in the release notes.
4. **Snapshot format versioning:** `schema.rs` output carries a format marker; the derivation engine rejects Snapshots from an incompatible CLI generation with a "re-run capture" message — CLI and library versions may drift, but never silently.
5. **Layout persistence:** versioned JSON (`data-models/workspace-layout.md` migration rules); unknown versions/panels restore via the total-restoration rule (RSK-08), never crash.
