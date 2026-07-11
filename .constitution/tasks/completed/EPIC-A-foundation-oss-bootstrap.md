# EPIC A — Foundation & OSS Bootstrap

Stands up the workspace, CI, licensing, public repo, and crates.io reservations per `tech-spec/guidelines.md` and the Stage-4 interview decisions (public on personal account; default branch `master`; branch-per-epic PRs; "gpui-admin contributors" copyright).

#### GA-A001 Initialize cargo workspace and toolchain pins

- **Type:** Chore
- **Effort:** 2
- **Dependencies:** None
- **Category:** DX
- **Scope (In-Scope Files):**
  - `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.gitignore`
  - `crates/gpui-admin{,-core,-macros,-cli,-ui,-conformance,-provider-supabase}/` (skeletons via `cargo new`, per guidelines layout)
- **Scope (Out-of-Scope Files):**
  - `examples/`, `backends/`, `docs/` (later epics)
- **Verification Command:** `cargo build --workspace && cargo clippy --workspace --all-targets -- -D warnings`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if any pinned version from `tech-spec/stack.md` fails to resolve from the registry; do not substitute versions."
- **Description:** Create the workspace exactly as `tech-spec/guidelines.md` lays it out: member crates scaffolded with the project's CLI tooling, `[workspace.dependencies]` carrying the exact-pinned substrate family and verified commodity versions, `[workspace.lints]` (clippy all+pedantic, forbid unsafe, deny missing_docs on public crates), toolchain file pinning 1.95.0. Empty lib/bin stubs compile clean.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a fresh clone on the pinned toolchain
When the workspace is built with lints enabled
Then all seven crates compile with zero warnings
And the lockfile is committed and matches the pinned BOM

Given the substrate family entries in Cargo.toml
When dependency versions are inspected
Then gpui, gpui-component and gpui_http_client are exact-pinned ("=x.y.z")
```

##### GA-A001 Deviations & Justifications

- **Touched Files (outside the declared Scope):**
  - `devenv.nix`, `devenv.yaml`, `devenv.lock`, `.envrc` (committed in a preceding foundation commit)
  - `.constitution/tech-spec/stack.md`, `.constitution/tech-spec/adrs/ADR-004-exact-pin-substrate.md`, `.constitution/tech-spec/guidelines.md` (Developer Environment section + dependency-direction wording), `.constitution/tech-spec/changelog.md`
  - `.constitution/tasks/active/EPIC-A-foundation-oss-bootstrap.md` (this file — the Gherkin line above)
- **Justification:**
  - **devenv (operator request + build necessity):** the operator asked to use devenv for tooling, and this ticket's Gherkin ("all seven crates compile") requires a real compile of the GPUI substrate, which needs a reproducible native graphics/windowing/font stack (wayland, xkbcommon, xorg, vulkan-loader, fontconfig, freetype). The in-scope `rust-toolchain.toml` still pins Rust 1.95.0 for non-Nix contributors and CI; devenv provides that same toolchain plus the system libraries.
  - **Spec correction (Step 4 — Keep the Spec Folders Honest):** `stack.md` and ADR-004 listed a non-existent substrate crate `gpui_platform = "=0.2.2"`. Live crates.io verification on 2026-07-10 shows it returns 404 under every name/version and is absent from `gpui 0.2.2`'s dependency graph; GPUI selects its platform backend through `gpui` feature flags (`wayland`/`x11` default-on for Linux, `metal` for macOS). It was removed with **no version substituted** (honoring the STOP condition) and the tech-spec bumped to v0.1.1. The matching line in this ticket's own Gherkin was corrected to name the three real substrate crates.

#### GA-A002 Licenses, README, and repository metadata

- **Type:** Chore
- **Effort:** 1
- **Dependencies:** GA-A001
- **Category:** Docs
- **Scope (In-Scope Files):**
  - `LICENSE-MIT`, `README.md`
  - `crates/*/Cargo.toml` (license, authors, repository, description metadata)
- **Scope (Out-of-Scope Files):**
  - `docs/` (Epic K, deferred)
- **Verification Command:** `cargo package -p gpui-admin-core --list --allow-dirty`
- **Expected Success Output:** `exit 0` (license files included in package listing)
- **STOP Conditions:**
  - "STOP if any crate metadata would publish without both license files."
- **Description:** MIT license (operator decision 2026-07-10; supersedes the earlier `MIT OR Apache-2.0`) with copyright line "Copyright (c) 2026 gpui-admin contributors". README states the product identity (Native-UX-led, per `prd/vision.md`), the pre-1.0 breaking-change policy (Compatibility Policy §3), and links the constitution. Every crate's metadata carries license, repo URL, and an honest description.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the repository root
When license files are inspected
Then the MIT license text exists naming "gpui-admin contributors"
And every crate manifest declares license = "MIT"
```

##### GA-A002 Deviations & Justifications

- **Touched Files (outside the declared Scope):**
  - `crates/*/LICENSE-MIT` (symlinks to the root LICENSE-MIT file)
  - MIT-decision propagation across the constitution: `.constitution/prd/vision.md`, `.constitution/prd/changelog.md`, `.constitution/tech-spec/stack.md` (License line), `.constitution/tech-spec/guidelines.md` (repository layout), `.constitution/tech-spec/changelog.md`, `.constitution/tasks/changelog.md`
- **Justification:** the ticket's verification command (`cargo package -p gpui-admin-core --list`) expects the license files to appear in each crate's package listing, and each crate is published standalone to crates.io (GA-A004), where a self-contained license file is expected. Per-crate symlinks to the single root license text (`LICENSE-MIT`) keep licensing DRY while making every crate's package self-describing. `cargo package --list` follows the symlinks and includes them; the command exits 0 with no manifest warnings. The `license = "MIT"` SPDX field is inherited from `[workspace.package]` (defined in GA-A001) via `license.workspace = true`. (Licensing was changed to MIT-only on 2026-07-10 per operator decision; the earlier `LICENSE-APACHE` and its per-crate symlinks were removed.)

#### GA-A003 Continuous integration pipeline

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** GA-A001
- **Category:** DX
- **Scope (In-Scope Files):**
  - `.github/workflows/ci.yml`
- **Scope (Out-of-Scope Files):**
  - `.github/workflows/release-plz.yml` (Epic K, deferred)
- **Verification Command:** `gh run watch --exit-status` (after push to a test branch)
- **Expected Success Output:** `exit 0` (all jobs green on both platforms)
- **STOP Conditions:**
  - "STOP if the macOS runner cannot build the substrate; capture the error verbatim and report — do not disable the lane (NFC-40 forbids claiming unverified platforms, and dropping the lane silently would do the inverse)."
- **Description:** GitHub Actions on pushes to `master` and all PRs: format check, clippy `-D warnings`, workspace tests — matrix over `ubuntu-latest` and `macos-latest` (the two P0 platforms). Cache cargo artifacts. Conformance-against-compose job is added in Epic E when the fixture exists.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a pull request with a formatting violation
When CI runs
Then the fmt job fails and the PR is not mergeable

Given a clean commit on master
When CI runs
Then fmt, clippy and test jobs pass on both Linux and macOS
```

#### GA-A004 Reserve crates.io names with placeholder publishes

- **Type:** Chore
- **Effort:** 2
- **Dependencies:** GA-A002
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `crates/*/Cargo.toml`, `crates/*/README.md` (publish metadata + development-notice text)
- **Scope (Out-of-Scope Files):**
  - All `src/` implementation files (placeholders publish empty, documented stubs only)
- **Verification Command:** `cargo publish -p <crate> --dry-run` per crate, then live publishes
- **Expected Success Output:** all seven crates visible on crates.io at `0.0.1`
- **STOP Conditions:**
  - "STOP if the registry token is unavailable — request it from the operator; never source tokens elsewhere."
  - "STOP if any family name is already taken; report which and do not publish variants."
- **Description:** Publish `0.0.1` placeholders for the seven-crate family (interview decision: reserve now). Each README states "in active development" and links the repository. Publish order respects intra-family dependencies (leaf crates first).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the crates.io registry
When each family name is queried after publishing
Then all seven names resolve to version 0.0.1 owned by the operator's account
And each crate page links back to the repository
```

#### GA-A005 Publish repository and protect master

- **Type:** Chore
- **Effort:** 1
- **Dependencies:** GA-A002, GA-A003
- **Category:** DX
- **Scope (In-Scope Files):**
  - Repository settings via `gh` (visibility, default branch `master`, branch protection)
- **Scope (Out-of-Scope Files):**
  - All code (settings-only ticket)
- **Verification Command:** `gh repo view --json visibility,defaultBranchRef && gh api repos/{owner}/{repo}/branches/master/protection`
- **Expected Success Output:** visibility `PUBLIC`, default `master`, protection requiring CI + PR
- **STOP Conditions:**
  - "STOP before flipping visibility if any committed file contains credentials or non-neutral IP references; report findings first."
- **Description:** Make the repo public under the operator's personal account (interview decision), keep `master` as default branch (interview decision — no rename), enable protection: PRs required, CI checks required, no force-push. Epic branches follow `type/description` naming; merges are whole-epic squash PRs.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the published repository
When an unauthenticated visitor opens it
Then the code, README and .constitution are readable
And direct pushes to master are rejected by protection rules
```
