# EPIC D — Replica & Mutation Core

Deepens the tracer's minimal slice into the full R2/R3 state machines (ADR-005, architecture flows: windowed-browsing, mutation-lifecycle, mid-edit-drift, degraded-connectivity). Everything headless, deterministic, UI-free. This epic is the RSK-01 complexity center — its final ticket is the adversarial interleaving suite.

#### GA-D001 Record store with immutable records and drift containment

- **Type:** Feature
- **Effort:** 2
- **Dependencies:** GA-C001
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/replica/store.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-ui/` (no UI in this epic)
- **Verification Command:** `cargo test -p gpui-admin-core replica::store`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if any consumer needs mutable access to a stored record; records are immutable by contract (ADR-005)."
- **Description:** `Id → Result<Arc<Record>, DriftError>`: installs are pointer swaps (LWW), deserialization failures stored as contained per-record faults (CAP-205), never propagated.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a store holding a record observed by two query snapshots
When a new version installs
Then both snapshots see the new Arc on next read without record cloning

Given a feed payload that fails to conform to the Record shape
When it installs
Then the store holds a per-record fault and sibling records are unaffected
```

#### GA-D002 Query layer: windows, generations, memoized views

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-D001
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/replica/{entry.rs,range_set.rs}`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-core/src/replica/overlay.rs` (GA-D003)
- **Verification Command:** `cargo test -p gpui-admin-core replica::entry`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if in-flight cancellation cannot ride task-drop semantics; that guarantee is load-bearing (resilience.md generation discipline)."
- **Description:** Full `QueryEntry`: requested-vs-loaded `RangeSet` (in-flight dedup), generation counter (invalidation bumps + drops in-flight tasks; stale-generation responses discarded), `total: Option`, cursor continuation for sequential mode, memoized joined view rebuilt only on relevant change, per-frame notification coalescing (RSK-09).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given an entry with a window fetch in flight
When the same range is demanded again
Then no duplicate fetch is issued

Given an entry invalidated while a fetch is in flight
When the stale response arrives after new data installed
Then the stale response is discarded by generation check
```

#### GA-D003 Overlay lifecycle: staged, undo, FIFO, settlement

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-D002
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/replica/overlay.rs`
  - `crates/gpui-admin-core/src/mutation/{mod.rs,fifo.rs,settle.rs}`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-ui/src/form/` (validation routing lands with Epic H, deferred)
- **Verification Command:** `cargo test -p gpui-admin-core mutation::`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if rollback ever requires a compensating write; rollback is subtraction-only by contract (ADR-005)."
- **Description:** Full three-state lifecycle over the tracer's minimal version: staged overlays with undo windows (revoke = never dispatched), per-record FIFO dispatch, sparse patch + previous at dispatch, settlement routing — confirm graduates (membership-affecting confirms mark queries stale), field-validation and failures roll back by overlay removal with preserved input surfaced through a feedback event type. Pending creates live in the provisional band structure with client-evaluated simple-predicate membership; failed creates stay in the band in error state.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given two staged updates to the same record
When both undo windows elapse
Then dispatches occur strictly in order, the second awaiting the first's settlement

Given a dispatched create rejected by the provider
When settlement processes
Then the overlay rolls back, the band entry enters error state
And the preserved payload is available for retry or discard
```

#### GA-D004 Field-aware invalidation

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-D002
- **Category:** Perf
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/replica/invalidation.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-core/src/replica/freshness.rs` (GA-D005)
- **Verification Command:** `cargo test -p gpui-admin-core replica::invalidation`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if correctness ever depends on the fine-grained path; the coarse policy must remain a correct fallback (flow-windowed-browsing note)."
- **Description:** Diff a changed record's fields (FieldSet AND) against each entry's filter/sort field-set: filter/sort touch or create/delete → mark stale + refetch (generation bump); display-only → in-place view patch, no refetch. Payloads without old-row data degrade to coarse per-resource invalidation.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a query filtered by status and a feed event changing only a display field
When invalidation runs
Then the view patches in place and no refetch is issued

Given the same query and an event changing status
When invalidation runs
Then the entry is marked stale and refetches
```

#### GA-D005 Freshness, observation guards and eviction

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-D004
- **Category:** Perf
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/replica/freshness.rs` (wall-clock staleness, SWR, RAII observation guards, unobserved eviction sweep)
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-core/src/registry/` (warmth handles already minimal from tracer; full registry in Epic G, deferred)
- **Verification Command:** `cargo test -p gpui-admin-core replica::freshness`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if observation counting requires substrate observer introspection; guards are framework-owned bookkeeping by design (see `.constitution/architecture/containers.md`, R2 observation-guard note)."
- **Description:** The 2×2 policy: fresh→serve; stale+observed→serve-then-background-refetch; stale+unobserved→evict on the low-frequency sweep. Observation via RAII guard objects (Drop decrements). Wall-clock `fetched_at` with revalidation on activation (suspend-safety).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a stale entry with one live observation guard
When it is read
Then the stale view serves immediately and one background refetch is issued

Given a stale entry whose last guard dropped
When the eviction sweep runs
Then the entry is removed and the record store is untouched
```

#### GA-D006 Change-feed reconciliation and reconnect staleness

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-D004
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/src/replica/feed.rs`
  - `crates/gpui-admin-conformance/src/mock.rs` (scripted feed scenarios only)
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-provider-supabase/` (real feed transport is Epic J, deferred)
- **Verification Command:** `cargo test -p gpui-admin-core replica::feed`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if any path assumes ordered or exactly-once delivery; the contract is at-least-once, unordered (resilience.md)."
- **Description:** LWW installs of feed upserts/deletes routed through invalidation; duplicate and reordered delivery absorbed idempotently; feed interruption → global staleness pass over warm replicas (observed queries refetch) — same rule reused for unreachable→healthy recovery.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a feed delivering the same upsert twice and out of order with a delete
When reconciliation processes all events
Then the final store state is identical regardless of arrival order

Given a feed disconnect and reconnect
When recovery runs
Then every warm replica's observed queries are marked stale and refetch
```

#### GA-D007 Adversarial interleaving suite

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-D003, GA-D005, GA-D006
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-core/tests/interleavings.rs`
- **Scope (Out-of-Scope Files):**
  - All `src/` (test-only ticket; fixes it forces become follow-up patches within the epic branch)
- **Verification Command:** `cargo test -p gpui-admin-core --test interleavings`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if an invariant violation reveals a contract gap rather than a bug; trigger a Stage 3 pass before patching around it."
- **Description:** The RSK-01 named cases as deterministic tests with simulated time: settlement racing a feed event racing an invalidation; undo racing window-elapse; stale-generation response after refetch; feed echo landing post-confirm (must self-heal); eviction racing observation re-registration. Executable invariants: one-way truth, subtraction-only rollback, generation monotonicity, notification coalescing bounds.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given every named interleaving scenario
When the suite runs under the deterministic executor
Then all scenarios pass with invariants asserted

Given the post-confirm stale-echo scenario
When the stale feed event lands after confirmation
Then the transient regression self-heals on the next event as documented
```
