# Open Decisions — 2026-08-21 (Realign interview)

Register of decisions deliberately left unresolved by the operator. Downstream stages must **not** answer these silently; each may be closed only by an explicit operator ruling in a later session.

## OD-01 — Audit log & record revisions

- **Asked:** A-Q2 (Theme A).
- **Ruling for now:** both rejected for v1; explicit `out-of-scope/` files required.
- **Explicitly open:** operator stated these "will be judged in the future." Reconsideration triggers to weigh at that time: compliance-driven adopter demand (the air-gapped JTBD audience values audit trails), and a cooperating Backend willing to expose revision/audit surfaces as Capabilities.
- **Constraint on any future pass:** neither feature can enter planning without a Stage 1 Evolution pass reversing the out-of-scope decision first; both demand backend cooperation far beyond elementary reads/writes, so they can only ever arrive as declared Capabilities.

## OD-02 — Concurrency strengthening beyond warning + conditional updates

- **Asked:** A-Q3 (Theme A).
- **Ruling for now:** mid-edit drift warning (CAP-306) + ConditionalUpdate capability (CAP-307) only. Advisory locking was evaluated and rejected for now.
- **Explicitly open:** operator will "revisit later when things stabilize." Candidate directions at revisit: advisory locking as a Capability (option b), or escalating the drift warning to a field-level diff with merge/reload/discard choices (option c).
- **Constraint on any future pass:** whichever direction is chosen must compose with, not replace, the existing pair; locking semantics (advisory vs mandatory, timeouts, orphaned locks) carry heavy conformance-suite obligations and require their own Stage 1→4 pass.

---
*Carried forward: none — no prior open-decisions register exists (first interview).*
