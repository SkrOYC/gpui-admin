# Out of Scope: Built-In Data Export

**Status:** Deferred (expressible later as an action).
**Since:** v0.1.0.

## Context

Tabular export (e.g., spreadsheet-compatible files) is a staple of web admin frameworks.

## Decision

No built-in export capability in the core. When custom and bulk actions land (CAP-901/902, P1), export becomes expressible as an ordinary action an Adopter or the ecosystem provides.

## Reasoning

Export is a *consumer* of the action surface, not a peer of the critical path. Building it into the core before the action surface exists would create a bespoke one-off; building it after makes it a validation case for the extensibility model. Honest sequencing also avoids implying that the client can faithfully export server-filtered, server-sorted collections it only ever windows.

## Reopening Conditions

Automatic once the action surface ships; likely as a first-party example action rather than core.
