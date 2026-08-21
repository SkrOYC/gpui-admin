# ADR-004: Exact-Pin Policy for the Substrate Family

**Status:** Superseded by [ADR-011](./ADR-011-substrate-rev-pinning.md) (2026-08-21 realign). Retained for decision provenance; the correction note below remains historically accurate.

## Context

`gpui` is pre-1.0 with an explicit "breaking changes between versions" warning; `gpui-component` does not re-export `gpui`, so the two must be version-compatible as *sibling* dependencies of ours. A pure crates.io chain is verified available (`gpui-component 0.5.1` → `gpui ^0.2.2`).

## Decision

Exact pins for the family — `gpui = "=0.2.2"`, `gpui-component = "=0.5.1"`, `gpui_http_client = "=0.2.2"` — upgraded only as a single deliberate PR that bumps all three together and re-exercises the two integration seams (windowed table binding, dock persistence). No git dependencies anywhere: hermetic, registry-only builds (NFC-11 discipline applied to dependencies).

> **Correction (v0.1.1, GA-A001):** the family originally also listed `gpui_platform = "=0.2.2"`. That crate does not exist on crates.io and is not a dependency of `gpui 0.2.2`; GPUI selects its platform backend through **feature flags on the `gpui` crate itself** (`wayland`/`x11` are on by default; macOS uses `metal`). It was removed after live registry verification during Epic A execution; no version was substituted.

## Consequences

- Reproducible builds and a controlled churn surface (RSK-02); adopters of our published crates inherit a coherent pair.
- Cost: we lag upstream features/fixes between deliberate upgrades; security-relevant upstream fixes trigger an out-of-band upgrade PR under the same pairing rule.
- Commodity crates stay caret-ranged with the committed lockfile (Compatibility Policy §2) — the strict policy is scoped to where breakage actually lives.
