# ADR-007: Standalone `gpui-admin` Binary (Not a Cargo Subcommand)

**Status:** Accepted.

## Context

The toolchain (B1/B3) needs a CLI entry point. A cargo subcommand (`cargo admin …`) requires publishing a binary named `cargo-admin`, squatting the generic `admin` name in every Rust user's cargo namespace; `cargo gpui-admin …` avoids the squat but reads clumsily.

## Decision

A standalone binary: `[[bin]] name = "gpui-admin"` in the `gpui-admin-cli` crate, installed via `cargo install gpui-admin-cli` (or distributed binaries later). Command tree in `contracts/cli.md`.

## Consequences

- Clean invocation (`gpui-admin introspect postgres …`), no cargo-namespace pollution, and the CLI can grow non-cargo concerns (e.g., compose helpers for the showcase) without violating subcommand expectations.
- Cost: not discoverable via `cargo --list`; mitigated by README/getting-started prominence (the 30-minute path starts with this binary, CAP-1002).
