# EPIC C — Tracer-Bullet Vertical

The de-risking epic (architecture strategy + RSK-02/03): one hand-written Resource end to end — windowed list, typed form, optimistic mutation with undo, one dock panel with persistence — against the mock Provider, with **no derivation macros**. Its code becomes the normative target the macros (Epic F) must emit. Both substrate integration seams are proven here before deep investment.

#### GA-C001 Hand-written tracer Resource and minimal replica slice

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-B006
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `examples/showcase/src/tracer/{schema.rs,resource.rs}` (hand-written mirror of `tech-spec/data-models/schema-snapshot.rs` exemplar)
  - `crates/gpui-admin-core/src/replica/{mod.rs,entry.rs}` (minimal: one query entry, window fetch, record install, observation)
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-core/src/replica/{overlay.rs,invalidation.rs,freshness.rs}` (Epic D)
  - `crates/gpui-admin-macros/` (must stay empty — the point of the tracer)
- **Verification Command:** `cargo test -p gpui-admin-core replica::minimal`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if the minimal slice needs an API the contract lacks; trigger a Stage 3 pass."
- **Description:** Hand-implement the `Resource`/`FieldIdent` traits for the tracer `User` (exactly the Snapshot exemplar shape) and the smallest honest replica: query entry keyed by (filter, sort), windowed fetch through the mock, record store install, change notification. Headless deterministic tests.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a warm replica and a query entry with rows 0..50 loaded
When rows 200..250 are demanded
Then exactly one window fetch is issued for the missing range
And observers are notified once the rows install
```

#### GA-C002 Windowed table binding seam

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-C001
- **Category:** Perf
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-ui/src/list/{mod.rs,delegate.rs}` (table delegate ↔ query entry binding, skeleton rows, per-row drift boundary)
  - `examples/showcase/src/tracer/list_panel.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-ui/src/form/` (GA-C003)
- **Verification Command:** `cargo run -p showcase --features tracer` (manual seam check) and `cargo test -p gpui-admin-ui list::`
- **Expected Success Output:** `exit 0`; scrolling the tracer list renders placeholders then rows, UI thread never blocks
- **STOP Conditions:**
  - "STOP if the table delegate cannot express visible-range demand-loading against the replica; record findings in SPK-C005 and halt the epic — this invalidates the random-access plan (RSK-03 fallback applies)."
- **Description:** Bind the substrate's virtualized table to the query entry: `rows_count` from `total`, per-index cell render from the memoized view, visible-range callback → window demands, skeleton cells for unloaded rows, contained error row for a drifted record (CAP-201/205 seam).
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a tracer list over 100_000 mock rows
When the view scrolls to an unloaded region
Then placeholder rows render within the same frame
And real rows replace them when the window fetch settles

Given one mock row scripted to deserialize as drifted
When the list renders that region
Then exactly one bounded error row appears and siblings render normally
```

#### GA-C003 Form, mutation and undo seam

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** GA-C001
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-ui/src/form/{mod.rs,field.rs}` (typed widget over entity input state: parse → validate, dirty tracking)
  - `crates/gpui-admin-core/src/mutation/mod.rs` (minimal staged→dispatch→settle; undo drops staged)
  - `examples/showcase/src/tracer/edit_panel.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/gpui-admin-core/src/mutation/fifo.rs` and failure-routing breadth (Epic D)
- **Verification Command:** `cargo test -p gpui-admin-core mutation::minimal && cargo test -p gpui-admin-ui form::`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if entity-held input state cannot host the parse-preserving field machine; capture the exact limitation before any workaround."
- **Description:** Two typed fields (text + enum-select) over the substrate's input states, keeping parsed values through domain-rule failures; save produces a sparse patch + previous; the minimal mutation coordinator stages the overlay (visible next frame — NFC-03), runs an undo window (revoke = never dispatched), dispatches to the mock, settles confirm/rollback.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a dirty tracer edit form
When save is pressed and undo is clicked within the window
Then the list view shows the original value again
And the mock provider records zero mutation calls

Given the same form saved without undo
When the mock confirms
Then the overlay graduates and the list shows the canonical value
```

#### GA-C004 Dock panel and persistence seam

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** GA-C002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/gpui-admin-ui/src/shell/{mod.rs,panel.rs}` (one panel kind registered by name; dump/load roundtrip)
  - `crates/gpui-admin-ui/src/state_store.rs` (layout file per `tech-spec/data-models/workspace-layout.schema.json`)
  - `examples/showcase/src/main.rs`
- **Scope (Out-of-Scope Files):**
  - Follow linkage, multi-workspace windows, placeholder-panel restoration (Epic G, deferred)
- **Verification Command:** `cargo test -p gpui-admin-ui shell::roundtrip`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - "STOP if dock layout dump/load cannot rebuild the tracer panel by its kind string; this seam gates the whole workspace design."
- **Description:** Mount the tracer list panel in the dock, register its kind string, persist layout to the schema-conformant JSON file on change, restore it at relaunch. Proves Routing-airlock entry 2 and the R9 envelope end to end at minimal scale.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given a running tracer app with the list panel split beside the edit panel
When the app exits and relaunches
Then the same arrangement restores from the layout file
And the layout file validates against the workspace-layout JSON Schema
```

#### GA-C005 Spike: windowed scrolling at the 10M envelope

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** GA-C002
- **Category:** Perf
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-C005.md` (sole output — no production code changes)
- **Scope (Out-of-Scope Files):**
  - All `crates/` production code
- **Verification Command:** `test -s .constitution/spikes/SPK-C005.md`
- **Expected Success Output:** `exit 0` (completed report at the designated path)
- **STOP Conditions:**
  - "STOP after the report; downstream tickets decide whether the fallback activates — do not implement either path here."
- **Description:** RSK-03 / NFC-01: drive the tracer list over a synthetic 10,000,000-row mock; measure sustained frame times during continuous scroll and random scrollbar jumps; probe memoized-view rebuild cost and window-fetch coalescing under fast scrubbing. Deliver go/no-go on random-access windowing at envelope scale, with the sequential-frontier fallback as the documented alternative.
- **Acceptance Criteria (Gherkin):**
```gherkin
Given the completed spike report at .constitution/spikes/SPK-C005.md
When it is reviewed
Then it contains measured frame-time distributions for scroll and jump scenarios
And an explicit chosen option with downstream ticket impact listed
```
