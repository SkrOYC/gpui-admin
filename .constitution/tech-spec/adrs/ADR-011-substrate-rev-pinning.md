# ADR-011: Substrate Family Tracked via Git at Frozen Revs (Dual-Location Manifests)

**Status:** Accepted (2026-08-21 realign). **Supersedes ADR-004's registry-only/no-git-deps policy.**

## Context

ADR-004 pinned the substrate family (`gpui`, `gpui-component`, `gpui_http_client`) to exact crates.io versions on the grounds that registry-only builds are hermetic and reproducible, with "no git dependencies anywhere."

Live verification during the 2026-08-21 realign changed the facts on the ground:

1. **gpui-component's own `main` builds against zed-industries/zed via git** (`gpui`, `gpui_platform`, `gpui_macros` are git dependencies of its workspace). Mixing a registry-resolved `gpui 0.2.2` with a git-resolved `gpui-component` produces duplicate-source type conflicts that do not compile: family-wide git is the only coherent configuration.
2. **The features this product plans against exist only on unreleased main.** `DataTable`, cell selection, Tab navigation, and `DoubleClickedCell` landed 2026-06-19; no published version contains them.
3. **The Cargo Book sanctions the hybrid**: dual-location dependencies (`version` + `git`) resolve from git locally while publishes reference the registry version. crates.io rejects git-only dependencies (except dev-dependencies), so pure-git manifests would break publishing entirely.

Operator ruling: track git instead of crates — matching upstream's own practice ("just like gpui directly is recommended at this stage").

## Decision

1. **Family-wide git tracking at lockfile-frozen revs**, dual-location declared: every substrate dependency specifies both a fallback registry `version` and its `git` location. The family comprises `gpui`, `gpui-platform`, `gpui_macros`, `gpui-component`, and `gpui-base` (plus any sibling the upstream restructure adds).
2. **Rev-freeze discipline replaces version-pairing:** revs move only via a deliberate bump PR that refreezes the whole family together and re-exercises the two integration seams (windowed table binding, dock persistence) plus the substrate-seam test suite.
3. **Publish gate:** `cargo publish` runs only when upstream registry releases satisfy the declared fallback versions, after a CI lane verifies the workspace resolves and compiles from the registry alone (what downstream consumers will get).
4. **Hermeticity preserved:** NFC-11 remains satisfied via the committed `Cargo.lock` (rev-pinned) plus `cargo vendor` for air-gapped builds.
5. **Open verdict carried into the re-pin ticket:** whether `gpui_http_client` (registry mirror) or zed-main's `http_client`/`reqwest_client` is the correct injected-trait source under frozen main revs (ADR-003 annotation).

## Consequences

- DataTable/cell-selection/Tab-nav capabilities arrive immediately; specs naming them become true as-written.
- Churn surface widens to zed-main, bounded by rev-freezing (RSK-02 mitigation updated accordingly).
- The first meaningful publish waits for upstream releases satisfying our fallback versions; until then crates.io placeholders remain placeholders.
- Vendor tooling (`cargo vendor`, lockfile audits) becomes load-bearing infrastructure; CI must fail on any manifest adding an unfrozen git reference outside the family policy.
