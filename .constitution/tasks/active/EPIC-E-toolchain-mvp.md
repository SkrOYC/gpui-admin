# EPIC E — Toolchain MVP

The offline build-time pipeline's capture half (B1/B2 containers): CLI skeleton, both introspection frontends emitting one neutral Snapshot, the compose fixture that feeds them, and the shared-corpus equivalence tests (RSK-04). Scaffolder and drift-check land in Epic F where the authoring surface exists.

#### GA-E001 Snapshot emitter

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-A001
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-cli/src/snapshot/{model.rs,emit.rs}` (neutral in-memory schema model → formatted `schema.rs` per `tech-spec/data-models/schema-snapshot.rs`)
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-cli/src/introspect/` (frontends come next)
- **Verification Command:** `cargo insta test -p gpui-admin-cli snapshot`
- **Expected Success Output:** `exit 0` (golden outputs match)
- **STOP Conditions:**
  - "STOP if emission needs source-dialect concepts in the neutral model; the model's neutrality is the CAP-102 guarantee."
- **Description:** The single emitter both frontends feed: neutral model (types, nullability, server-owned, unique, enums, relations) → generation banner + format marker + Record structs + field enums + metadata tables, formatted for human review. Golden-tested from hand-built model fixtures.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a neutral model fixture with nullable, server-owned and relation fields
When the emitter runs
Then the output matches the golden snapshot byte-for-byte
And carries the generation banner and format marker
```

#### GA-E002 CLI skeleton and exit-code discipline

- **Type:** Feature
- **Effort:** 2
- **Dependencies:** GA-A001
- **Category:** DX
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-cli/src/main.rs`, `crates/gpui-admin-cli/src/cmd/`
- **Scope (Out-of-Scope Files):**
  - Frontend implementations (stubs return "not yet implemented" with exit 1)
- **Verification Command:** `cargo test -p gpui-admin-cli cli::tree`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if any flag or exit code diverges from `tech-spec/contracts/cli.md`; the command tree is contract."
- **Description:** The full command tree (`introspect postgres|openapi`, `scaffold`, `check`, `showcase up|down|seed`) with the contract's flags, help texts, exit codes 0–4, stderr/stdout separation, and credential-never-persisted handling.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the built binary
When invoked with --help at each tree level
Then commands, flags and defaults match the CLI contract

Given scaffold pointed at an existing owned file without --force
When it runs
Then it refuses with exit code 4 and touches nothing
```

#### GA-E003 Showcase backend compose fixture

- **Type:** Chore
- **Effort:** 2
- **Dependencies:** GA-E002
- **Category:** DX
- **Scope (In-Scope Files):**
  - `backends/showcase/compose.yaml`, `backends/showcase/init/` (applies `tech-spec/data-models/showcase-backend.sql`)
  - `crates/gpui-admin-cli/src/cmd/showcase.rs`
  - `.github/workflows/ci.yml` (add Linux-lane service startup for later conformance jobs)
- **Scope (Out-of-Scope Files):**
  - `examples/showcase/` app (Epic K, deferred)
- **Verification Command:** `cargo run -p gpui-admin-cli -- showcase up && curl -sf localhost:3000/users?limit=1`
- **Expected Success Output:** `exit 0` (data API serving seeded rows)
- **STOP Conditions:**
  - "STOP if the seed DDL fails to apply cleanly; fix must go to the tech-spec data model, not an ad-hoc local patch."
- **Description:** ADR-008's reproducible environment: containers for the database + data-API layer, seeded from the normative DDL, wired to the `showcase` CLI verbs; CI Linux lane can start it as a service for Epic F/J jobs.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a clean machine with container tooling
When showcase up runs
Then the data API serves the three seeded resources
And showcase down leaves no running containers
```

#### GA-E004 Database catalog frontend

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-E001, GA-E003
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-cli/src/introspect/postgres/`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-cli/src/introspect/openapi/` (GA-E005)
- **Verification Command:** `cargo test -p gpui-admin-cli introspect::postgres` (integration against the compose fixture)
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if a catalog concept has no neutral-model representation; extend the model via a Stage 3 data-model pass, never with a dialect leak."
- **Description:** Read the catalog (tables, columns, types, nullability, defaults→server-owned classification, unique constraints, enums, FK relations) for the selected schema/globs into the neutral model → emitter. Connection credentials from flags/env only, never persisted (CLI contract).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the running showcase fixture
When introspect postgres runs against it
Then the emitted Snapshot classifies id/created_at/updated_at as server-owned
And captures both enum types and all three FK relations
```

#### GA-E005 API-description frontend

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-E001
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-cli/src/introspect/openapi/`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-cli/src/introspect/postgres/` (done)
- **Verification Command:** `cargo test -p gpui-admin-cli introspect::openapi`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if weak-truth cases (missing nullability/relations) tempt silent guessing; conservatism must be visible in the model (Snapshot invariant 5)."
- **Description:** Parse 3.0/3.1 documents (file or URL) into the same neutral model: schemas→Records, formats→types, `required`→nullability (conservative when unstated), reference-shaped fields→relations where expressible, weak-truth markers for Scaffolder visibility. Fixture set includes a document generated by the showcase data-API layer.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a document omitting nullability for a field
When introspection runs
Then the neutral model marks the field with a conservative, reviewable weak-truth flag

Given the showcase-generated API description
When introspection runs
Then all three resources emit with types matching the catalog frontend's reading
```

#### GA-E006 Shared fixture corpus and frontend equivalence

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-E004, GA-E005
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-cli/tests/equivalence.rs`, `crates/gpui-admin-cli/tests/fixtures/`
- **Scope (Out-of-Scope Files):**
  - Frontend internals (test-only ticket)
- **Verification Command:** `cargo test -p gpui-admin-cli --test equivalence`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if equivalence requires weakening the byte-identical guarantee; divergence is an RSK-04 defect, not a tolerance."
- **Description:** The toolchain's conformance analogue: paired fixtures (catalog dump ↔ equivalent API description) through both frontends must produce byte-identical Snapshots; weak-truth divergences appear only as the documented conservative markers.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given each paired fixture in the corpus
When both frontends process their half
Then the two emitted Snapshots are byte-identical
Or differ only in documented weak-truth markers listed by the test report
```
