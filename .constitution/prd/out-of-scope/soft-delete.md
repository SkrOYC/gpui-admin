# Out of Scope: Soft Delete

## Context
Soft-delete semantics (tombstone flags, restore flows, deleted-records views) as a framework feature.

## Decision
`rejected`

## Reason
A 'deleted' flag is a Backend convention, not physics — it demands cooperation conventions the Provider baseline cannot assume. Deletion responsibility sits with the Operator under explicit confirmation (CAP-308) plus the Undo Window (CAP-303). Adopters can model tombstone fields as ordinary Fields today.

## Consequences
Must not reappear without a Stage 1 Evolution pass; if revisited, arrive only as a declared Capability with paired conformance obligations.
