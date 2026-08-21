# Out of Scope: Record Locking

## Context
Advisory or mandatory record locks acquired on edit to exclude concurrent writers.

## Decision
`rejected`

## Reason
The concurrency story ships complete with CAP-306 (mid-edit drift warning) and CAP-307 (conditional updates). Lock semantics vary wildly (advisory vs mandatory, timeouts, orphaned locks) and add a fourth concurrency concept to document.

## Consequences
Revisit trigger recorded as OD-02 ('when things stabilize'); must compose with — never replace — warning + conditional updates.
