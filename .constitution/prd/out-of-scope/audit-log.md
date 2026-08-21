# Out of Scope: Audit Log

## Context
An event stream of who-did-what across Records surfaced by the framework.

## Decision
`rejected`

## Reason
Requires backend-side audit infrastructure far beyond elementary reads/writes; normalizing heterogeneous audit APIs is a subsystem with no sovereign-baseline implementation. Client-side action logging already exists via local tracing (R8).

## Consequences
Revisit trigger recorded as OD-01. Any future pass must model it as an opt-in Capability; never simulate audit truth client-side.
