# Resilience & Cross-Cutting Concerns

## Trust Boundaries & Security

- **The only enforcement boundary is the Backend** (NFC-30). Every client-side gate — permission affordances, hidden actions, disabled controls — is advisory by construction. R7 exposes affordance *hints*; R1's normalized `forbidden` error is the corrective feedback loop that repairs a stale hint. No container may treat an affordance as a precondition for correctness.
- **Secrets:** session material lives in R7 and is persisted, if at all, only via platform secure storage (NFC-32). R9 (Local State Store) never holds credentials; Deep Links never carry session state.
- **Egress discipline:** R1 is the only container with Backend network access; B1 is the only tool with Schema Source access, and only during explicit invocation. A default build makes no other network contact (NFC-31) — structurally auditable because egress is confined to two named boundaries.

## Failure Handling

**Error taxonomy (normalized at R1, consumed everywhere):** unauthenticated · forbidden · not-found · unsupported-query (a Filter/Search construct this Provider cannot translate — rejected loudly, never dropped) · field-validation (carries per-Field detail) · conflict · transport (retryable flag) · provider-internal. Views never see raw transport failures.

**Reads (windows, records, refetches):**
- Per-request timeout; bounded automatic retries with backoff for retryable transport errors — reads are idempotent by contract.
- **Generation discipline:** every query entry carries a generation counter; invalidation increments it and abandons in-flight requests; a late response with a stale generation is discarded. Stale data can never resurrect.
- A failed window fetch renders as a placeholder-with-retry region, never an empty lie.

**Mutations:**
- **Never auto-retried.** Retry is an explicit Operator act from the preserved-input state (CAP-304, NFC-21).
- Per-Record FIFO in R3: a Record's second mutation waits for the first to settle, keeping overlays coherent.
- Rollback is subtraction: removing the overlay restores pure Backend truth; there are no compensating writes to fail.
- Undo Window semantics: a staged change is not yet dispatched, so revocation is non-occurrence — the strongest possible failure semantics for the most common regret path.

**Connectivity state machine (owned by R1, surfaced by R5):**
- `healthy` → normal operation.
- `degraded` (intermittent failures) → reads serve warm data with staleness indication; background refetch continues with backoff.
- `unreachable` → warm data remains fully browsable with a persistent status banner; mutation attempts fail fast into preserved-input state; no durable offline queue (per PRD `out-of-scope/offline-first.md`).
- Recovery from `unreachable` triggers the same global staleness pass as feed reconnection (below).

**Change feed (Capability-gated):**
- Delivery assumption: at-least-once, unordered. Absorbed by idempotent last-write-wins installs into R2.
- **Reconnect rule:** any feed interruption assumes missed events — every warm Replica's queries are marked stale; observed ones refetch (serve-stale-then-refresh). Resume-from-position is a further optional Capability, never assumed.
- Transient reordering under last-write-wins may briefly regress and self-heals on the next event — accepted, documented behavior.

## Data Integrity

- **One-way truth (run time):** Backend → Gateway → Replica → Views. The overlay is a read-time lens, never written back into record truth. Pending state and confirmed state are architecturally incapable of being confused.
- **One-way truth (build time):** Schema Source → Snapshot → (Scaffold once) → verification. Regeneration overwrites only machine-owned artifacts; contradictions halt the build.
- **Fault containment:** a Record that fails to conform is stored as a contained per-Record fault and rendered as a bounded error region (CAP-205, NFC-20); one bad Record can degrade exactly one row, nowhere else.
- **Honest derived state:** query views are memoized joins, rebuilt only on relevant change (field-aware invalidation); anything the client cannot truthfully compute (server ordering, filtered membership on complex predicates, totals on sequential Backends) is deferred to the Backend rather than simulated.

## Identity & Session

- Authentication is pluggable with a first-class none-mode (CAP-701). Session expiry mid-flight is normalized by R1 into `unauthenticated`, raised by R7 to R5 as an *interruption*: the Workspace persists, in-progress form input is preserved, and re-authentication resumes work in place (P1: CAP-702). Views contain zero authentication logic.

## Observability (local-only)

- **Trace correlation:** every Operator action mints a trace identifier that propagates through R6 → R3/R2 → R1 and back through settlement/notification, and is joined by feed-event traces. "Why is this panel stale?" is answerable from one filtered trace.
- **Structured local logs:** container-tagged, level-controlled, written locally. No network sink exists (NFC-31).
- **Developer-mode diagnostics panel (R8):** live introspection of Replica contents, query freshness/generations/window coverage, pending mutation lifecycle states, connectivity state, and declared Capabilities. Gated behind an explicit developer mode because it exposes live Backend data on screen; it is a first-class debugging tool for Provider authors (CAP-1003).

## Configuration

- All application composition (Provider selection + its configuration, auth mode, developer mode, theming hooks) is declared by the Adopter in owned code verified at build time — consistent with the code-first posture (PRD `out-of-scope/no-code-builder.md`). Runtime-tunable settings are confined to Operator preferences in R9 (never correctness-relevant).
- Capability declarations come from the Provider, not from configuration: the client discovers what its Backend can do from the contract surface, ensuring configuration cannot claim cooperation the Backend does not deliver.
