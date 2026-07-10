# gpui-admin

**A framework and companion toolchain for building native desktop administration
applications** over backends you do not necessarily control — written in Rust on
[GPUI](https://crates.io/crates/gpui) (Zed's UI framework) and the
[`gpui-component`](https://crates.io/crates/gpui-component) widget library.

gpui-admin is **Native-UX-led**: the goal is an admin that feels like professional
desktop software — multi-panel split / tabbed / floating workspaces, live
master-detail, instant startup, and fluid interaction over millions of records — not a
web app in a window.

Two design pillars:

- **Build-time correctness.** Resource declarations are verified against a captured,
  reviewable schema snapshot; drift is a *build failure*, never a runtime surprise.
- **Backend sovereignty.** The client is correct against an uncooperative backend and
  is never the security boundary; anything requiring backend cooperation is an opt-in
  capability. Authorization truth always belongs to the backend.

Target platforms are **Linux and macOS** first (Windows later). Open source from the
first release.

> **Status: pre-1.0, in active development.** The workspace is being built epic by
> epic; most crates are documented placeholders today.

## Workspace

| Crate | Role |
| --- | --- |
| [`gpui-admin`](crates/gpui-admin) | Facade / prelude — what adopters `cargo add`. |
| [`gpui-admin-core`](crates/gpui-admin-core) | Provider Gateway, Replica store, Mutation lifecycle. |
| [`gpui-admin-macros`](crates/gpui-admin-macros) | Derivation proc-macros. |
| [`gpui-admin-cli`](crates/gpui-admin-cli) | The `gpui-admin` CLI: introspection, snapshot, scaffold. |
| [`gpui-admin-ui`](crates/gpui-admin-ui) | Shell, views, forms, layout store. |
| [`gpui-admin-conformance`](crates/gpui-admin-conformance) | Executable Provider-contract conformance suite. |
| [`gpui-admin-provider-supabase`](crates/gpui-admin-provider-supabase) | First-party Supabase / PostgREST provider. |

## Compatibility policy (pre-1.0)

While the project is pre-1.0, **a minor version bump may contain breaking changes**
(the SemVer pre-1.0 allowance). The Provider contract and the builder surface are the
compatibility-critical APIs; any change to either is paired with a conformance-suite
change and a migration note. The GPUI substrate (`gpui`, `gpui-component`,
`gpui_http_client`) is exact-pinned and upgraded only in deliberate, reviewed steps.

## Development

This repository ships a [devenv](https://devenv.sh) environment that provides the
pinned Rust 1.95.0 toolchain and GPUI's native build dependencies:

```sh
devenv shell        # enter the environment
ci                  # fmt + clippy -D warnings + build + test
```

Without Nix, install the toolchain declared in the committed `rust-toolchain.toml` and
build with `cargo build --workspace`.

## The constitution

This project is governed by a four-stage constitution under
[`.constitution/`](.constitution): product requirements
([`prd/`](.constitution/prd)), architecture
([`architecture/`](.constitution/architecture)), the technical implementation spec
([`tech-spec/`](.constitution/tech-spec)), and the ordered execution plan
([`tasks/`](.constitution/tasks)).

## License

Licensed under the [MIT license](LICENSE-MIT). Copyright (c) 2026 gpui-admin
contributors.

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you shall be licensed as above, without any additional terms
or conditions.
