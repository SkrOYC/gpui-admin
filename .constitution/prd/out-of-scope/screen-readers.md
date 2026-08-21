# Out of Scope: Screen-Reader Accessibility

## Context
Exposure of an accessibility tree for assistive technology beyond keyboard operability (NFC-50).

## Decision
`deferred`

## Reason
The substrate UI engine's accessibility support is real but young; committing before the substrate stabilizes risks shipping broken semi-support worse than declared absence. Keyboard operability of core flows remains P0/P1 per NFC-50.

## Consequences
Trigger for reopening: substrate accessibility-tree maturity verified during a deliberate bump PR. Re-evaluate at that point, not silently.
