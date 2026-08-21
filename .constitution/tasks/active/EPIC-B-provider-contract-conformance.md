# EPIC B — Provider Contract, Conformance & Mock

Materializes `tech-spec/contracts/provider.rs` (+ `object_store.rs`) into `gpui-admin-core`, with the Conformance Suite (C1) and a fully capable mock Provider. Everything here is UI-free and headless-testable. Contract-suite atomicity (RSK-07) starts binding at this epic.

#### GA-B000 Substrate re-pin to frozen git revs

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** None (first ticket of the post-Epic-A era)
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `Cargo.toml`, `Cargo.lock` (workspace root)
  - `rust-toolchain.toml` (only if the frozen revs demand a toolchain move)
  - `.github/workflows/ci.yml` (registry-resolution lane)
- **Scope (Out-of-Scope Files):**
  - All `crates/*/src/` production logic (no behavior changes)
- **Verification Command:** `cargo build --workspace && cargo test --workspace`
- **Expected Success Output:** `exit 0` on both P0 platforms
- **STOP Conditions:**
  - "STOP if the frozen revs cannot satisfy either integration seam (windowed table, dock persistence); record findings and trigger an ADR-011 revision instead of patching."
- **Description:** Execute ADR-011: point the substrate family (`gpui`, `gpui-platform`, `gpui_macros`, `gpui-component`, `gpui-base`) at lockfile-frozen git revs with dual-location manifests (fallback registry versions declared). Deliver the `gpui_http_client` vs zed-main `http_client` verdict (ADR-003 annotation) and amend contracts if the trait source migrates. Re-exercise both tracer-bullet seams via the Epic A-era smoke paths. Add the CI registry-resolution lane (publish gate prerequisite).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the workspace manifests declaring dual-location substrate dependencies
When cargo builds with the committed lockfile
Then every family member resolves from its frozen rev and the build is green

Given the registry-resolution CI lane
When it runs against the same sources ignoring git locations
Then the outcome (pass/fail + missing-release list) is recorded in the PR description

Given the HttpClient source verdict
When the re-pin PR merges
Then ADR-003 carries the decision and the conformance suite targets that source
```

#### GA-B001 Core contract types

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-B000
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/provider/{mod.rs,types.rs,query.rs}`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-ui/` (no UI in this epic)
- **Verification Command:** `cargo test -p gpui-admin-core provider::`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if implementing requires widening any signature in `tech-spec/contracts/provider.rs`; trigger a Stage 3 pass instead."
- **Description:** Implement the contract's foundation exactly as specified: `Resource`/`FieldIdent` traits, `FieldSet` (fixed-width mask; intersection is one bitwise AND), `UpdatePatch` (dirty-set + values), the typed `Query` model (closed `FilterOp` set, first-class `search`, extension escape hatch), `Window`/`OpaqueCursor`, `GetListParams`/`GetListResult` with `total: Option`.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a hand-written test Resource with a 7-variant field enum
When a FieldSet of {filter fields} is intersected with a dirty set
Then the result is computed via a single mask operation and matches set semantics

Given an UpdatePatch with two dirty Fields
When it is serialized through a test dialect
Then only the two dirty Fields appear in the payload
```

#### GA-B002 Error taxonomy

- **Type:** Feature
- **Effort:** 2
- **Dependencies:** GA-B001
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/provider/error.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-core/src/replica/` (consumes taxonomy later)
- **Verification Command:** `cargo test -p gpui-admin-core error::`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if a needed error case has no taxonomy variant; trigger a Stage 3 pass rather than adding ad-hoc variants."
- **Description:** `DataError` and `MutationError<R>` per contract: unauthenticated/forbidden/not-found/unsupported-query/transport{retryable}/provider, plus typed per-Field `Validation` and `Conflict`. These are the only error types crossing crate boundaries (guidelines rule).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a MutationError::Validation carrying two field entries
When it is matched by consumer code
Then each entry yields the typed Field identity and its message

Given a transport failure marked retryable
When classified by a retry-policy helper
Then reads are eligible for retry and mutations are not
```

#### GA-B003 DataProvider, Capabilities, AuthProvider and NoAuth

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-B002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/provider/{traits.rs,capabilities.rs,auth.rs,object_store.rs}`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-provider-supabase/` (Epic J, deferred)
- **Verification Command:** `cargo test -p gpui-admin-core provider:: && cargo doc -p gpui-admin-core --no-deps`
- **Expected Success Output:** `exit 0` (docs build proves missing_docs compliance)
- **STOP Conditions:**
  - "STOP if async-fn-in-trait forces a dyn-compat workaround anywhere; providers are generic-only by contract (ADR-003)."
- **Description:** The `DataProvider` trait including the bulk forms (`create_many`/`update_many`/`delete_many` with loop-fallback semantics and the singular-only ConditionalUpdate rule), the independent `ObjectStore` port trait per `contracts/object_store.rs` (put/get/delete, UploadHandle semantics; implementation adapters arrive in Epic J), `Capabilities`/`PaginationMode` declarations, `SyncProvider` signature (stub semantics; exercised in Epic D), the typed `PermissionHints` vocabulary (`ResourcePermissions` + `FieldPermissionMask`; advisory only), `AuthProvider` + shipping `NoAuth` (every method succeeds with anonymous identity — CAP-701).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the NoAuth implementation
When check_session is awaited
Then it resolves to an anonymous Identity without any I/O

Given a provider declaring SequentialCursor without counting
When Capabilities are inspected by consumer code
Then pagination mode and counting are independently readable

Given a mock backend lacking native bulk support
When update_many is invoked over three ids
Then exactly three singular updates are dispatched in order
```

#### GA-B004 HTTP client adapter

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-B000
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/provider/http.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-core/src/provider/traits.rs` (adapter is independent of the contract)
- **Verification Command:** `cargo test -p gpui-admin-core http::`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if clean shutdown cannot be guaranteed without substrate changes; document the ordering hazard (ADR-003 notes it) and report."
- **Description:** The thin adapter implementing the injected HTTP trait over the pinned HTTP stack, running on a dedicated runtime thread (ADR-003). Redaction of secret-classified header values at this boundary (RSK-10). Tested against an in-process TCP fixture: request/response lifecycle, timeout mapping to the retryable transport error, clean shutdown.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the adapter and an in-process test server
When a request exceeds its configured timeout
Then the caller receives a transport error marked retryable
And the runtime thread survives for subsequent requests

Given a request carrying an authorization header
When the request is traced/logged
Then the header value is redacted
```

#### GA-B005 Conformance suite skeleton

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-B003
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-conformance/src/` (obligation registry, runner, report format, scripted-backend fixtures)
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-ui/`, `crates/gpui-admin-cli/`
- **Verification Command:** `cargo test -p gpui-admin-conformance`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if an obligation cannot be expressed against the public contract alone; the suite must never reach into non-public APIs."
- **Description:** The executable meaning of the Provider contracts (flow-conformance-validation): core obligations (read semantics, window correctness per declared pagination mode, sparse-patch/previous-state handling in both dialects, bulk semantics incl. loop-fallback and partial-failure reporting, error-taxonomy fidelity, honest capability declaration, permission-vocabulary shape) as a runnable checklist over any `DataProvider`, with a precise failure report (obligation, observed vs required).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a deliberately broken provider that returns wrong window offsets
When the suite runs
Then it fails naming the windowing obligation with observed vs required rows

Given a provider declaring capabilities it does not honor
When the capability obligations run
Then the suite fails identifying the dishonest declaration
```

#### GA-B006 Mock provider passing conformance

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-B005
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-conformance/src/mock.rs` (in-memory provider with configurable Capabilities, scripted latency/failures, scripted change-feed)
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-provider-supabase/` (deferred)
- **Verification Command:** `cargo test -p gpui-admin-conformance mock_passes_conformance`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if passing requires weakening an obligation; obligations only change with a Stage 3 contract pass (RSK-07)."
- **Description:** Full in-memory `DataProvider` with every Capability toggleable, deterministic scripted latency/failure/feed-event injection — plus bulk behaviors exercising loop-fallback and partial-failure reporting, and permission-vocabulary shapes for affordance tests — the test double for every later epic (tracer bullet, replica suites, interleaving tests). Passes the whole suite in both pagination modes.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the mock configured as RandomAccess with counting
When the full conformance suite runs
Then every obligation passes

Given the mock reconfigured as SequentialCursor without counting
When the full conformance suite runs
Then every obligation passes with totals reported as absent
```
