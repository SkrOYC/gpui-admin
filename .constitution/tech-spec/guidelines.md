# Guidelines — Structure & Standards

## Project Structure

Cargo workspace. Crate names = reserved crates.io family (verified unclaimed 2026-07-09). Mapping to logical containers in parentheses.

```
gpui-admin/
├── Cargo.toml                      # workspace root: members, [workspace.dependencies],
│                                   # [workspace.lints], shared metadata
├── Cargo.lock                      # committed
├── rust-toolchain.toml             # 1.95.0 + rustfmt/clippy/rust-analyzer
├── LICENSE-MIT
├── README.md                       # identity, quickstart, pre-1.0 churn policy
├── crates/
│   ├── gpui-admin/                 # facade: prelude re-exports of core+ui+macros;
│   │                               # what Adopters `cargo add`
│   ├── gpui-admin-core/            # R1 Provider Gateway, R2 Replica, R3 Mutation
│   │   ├── src/
│   │   │   ├── provider/           #   contract traits, error taxonomy, capabilities,
│   │   │   │                       #   reqwest HttpClient adapter (own tokio thread)
│   │   │   ├── replica/            #   record store, query entries, overlay, invalidation
│   │   │   ├── mutation/           #   staged/undo/FIFO/settlement (R3)
│   │   │   ├── registry/           #   R4 airlocks, warmth, boot assertions
│   │   │   ├── resource/           #   typed builder authoring surface (contracts/resource_builder.rs)
│   │   │   ├── session/            #   R7 auth surface, none-mode
│   │   │   └── diag/               #   R8 introspection traits + tracing helpers
│   │   └── tests/                  #   headless #[gpui::test] state-machine suites
│   ├── gpui-admin-macros/          # B4 derivation proc-macros (consume committed schema.rs)
│   ├── gpui-admin-cli/             # B1+B3; [[bin]] name = "gpui-admin" (ADR-007)
│   │   └── src/
│   │       ├── introspect/         #   postgres/ (catalog), openapi/ (document) frontends
│   │       ├── snapshot/           #   neutral schema.rs emission (data-models/schema-snapshot.rs)
│   │       └── scaffold/           #   builder-code emission via prettyplease
│   ├── gpui-admin-ui/              # R5 Shell (dock, follow, feedback), R6 Views & Forms,
│   │                               # R8 diagnostics panel, R9 layout store
│   ├── gpui-admin-conformance/     # C1 harness: obligations, scripted backend, fixtures
│   └── gpui-admin-provider-supabase/ # PostgREST transport (Range-header windows P0,
│                                   # Realtime feed P1); consumes only public contracts
├── examples/
│   └── showcase/                   # CAP-1001 app: composition of everything above
├── backends/
│   └── showcase/                   # compose.yaml (PostgreSQL 17 + PostgREST 13),
│                                   # seed from data-models/showcase-backend.sql
├── docs/                           # getting-started (CAP-1002 30-min path), provider-authoring
└── .github/workflows/              # ci.yml (fmt, clippy, test, conformance), release-plz.yml
```

**Dependency direction (enforced by crate graph):** `cli` and `conformance` never depend on `ui`; `core` depends on `gpui` but never on `gpui-component`; `provider-supabase` (and the `conformance` harness) depend on `core`'s public contracts plus `gpui_http_client` solely to name the injected `HttpClient` trait they must implement/exercise (ADR-003, `contracts/provider.rs`) — no other substrate privilege, keeping the first-party Provider unprivileged (CAP-603/604); `macros` is a dependency of the Adopter's app crate, not of `core`.

## Developer Environment

A reproducible [devenv](https://devenv.sh) (Nix) shell provides the pinned Rust 1.95.0 toolchain and GPUI's native build dependencies — `wayland`, `libxkbcommon`, the xorg libs, `vulkan-loader`, `fontconfig`, and `freetype` on Linux; system frameworks on macOS. `rust-toolchain.toml` is the source of truth for non-Nix contributors — their local `rustup` selects `1.95.0` automatically; CI pins the same `1.95.0` explicitly in `ci.yml`, kept in lockstep with this file and `devenv.nix`. Enter with `devenv shell`; the `ci` script runs the full local gate (fmt, clippy `-D warnings`, build, test). CI runners are not under devenv and install the equivalent system libraries via `apt` on the Linux lane.

## Coding Standards

- **Formatting:** rustfmt defaults, checked in CI. No custom rustfmt.toml unless a rule earns an ADR.
- **Linting:** workspace-level `[workspace.lints]`: clippy `all` + `pedantic` (curated allows), `-D warnings` in CI. Library crates additionally deny `clippy::unwrap_used`, `clippy::expect_used` (structured errors only); binaries/tests exempt.
- **Unsafe:** `#![forbid(unsafe_code)]` in every crate. No exceptions at v0.x.
- **Public API docs:** `#![deny(missing_docs)]` in `gpui-admin`, `gpui-admin-core`, `gpui-admin-conformance` — the surfaces Adopters and Contributors read.
- **Errors:** the taxonomy in `contracts/provider.rs` is the only error currency crossing crate boundaries; `anyhow` confined to binaries.
- **Naming discipline:** code identifiers follow `.constitution/prd/glossary.md` (Vocabulary Drift Rule): `Provider`, `Replica`, `PendingChange`, `Snapshot`, `Scaffold`, `Workspace`, `Panel` — never the prohibited synonyms.

### Testing expectations

- R2/R3 state machines: deterministic `#[gpui::test]` suites with scripted Providers and simulated time; the interleaving cases from RSK-01 (settlement × feed × invalidation) each get named tests.
- Both introspection frontends run against **one shared fixture corpus** with `insta` golden Snapshots (RSK-04); Scaffolder output is golden-tested the same way.
- The Conformance Suite is executable documentation: every Provider-contract obligation exists as a suite check before or with the code implementing it (RSK-07 atomicity).
- Perf gates from constraints run as benches on the tracer-bullet path: windowed scroll fixture at envelope scale (NFC-01) and incremental-build timing (NFC-10) tracked in CI as trend metrics, hard-failed on gross regression.

### Observability hooks

- Every Operator-initiated action opens a `tracing` span (`action.*`) whose id propagates through Gateway requests (`provider.*`), overlay transitions (`mutation.*`), and install/invalidate events (`replica.*`) — the R8 correlation contract. Span/field names are stable API for the diagnostics panel.
- No `println!`/`eprintln!` in library code; `tracing` macros only. Secret-classified values are redacted at the Gateway boundary by construction (RSK-10).

### Migration & compatibility rules

- Provider contract or builder-surface change → paired Conformance/golden-test change + release-note migration entry (Compatibility Policy §3).
- Snapshot format change → format-marker bump + CLI/derivation cross-check error message (Policy §4).
- Layout persistence change → version bump + migration in the R9 loader; restoration never fails (Policy §5, RSK-08).
