# Out of Scope: Offline-First Operation

**Status:** Rejected (structural).
**Since:** v0.1.0.

## Context

Offline-first tools queue local edits while disconnected and reconcile later. (Distinct from *offline builds*, which ARE in scope: NFC-11 requires air-gapped buildability.)

## Decision

A running admin application requires connectivity to its Backend to mutate data. Brief disconnections are handled gracefully (clear status, safe recovery, no lost input), but there is no durable offline mutation queue or merge machinery.

## Reasoning

The run-time truth model is one-way: the Backend is the sole authority, and Pending Changes are short-lived, revocable previews — not a second ledger. A durable offline queue would require conflict resolution machinery that either demands Backend cooperation (violating sovereignty as a *requirement*) or silently invents merge semantics the Backend never agreed to. Admin work is also overwhelmingly performed connected.

## Reopening Conditions

Sustained adopter demand from field-work domains, and only ever as an opt-in Capability for cooperating Backends — never as a core assumption.
