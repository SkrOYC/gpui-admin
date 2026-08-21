# Out of Scope: Server-Dispatching Autosave

## Context
Automatic debounced dispatch of form edits to the Backend (react-admin AutoSave-style).

## Decision
`rejected`

## Reason
Blur-triggered dispatches destroy the Undo Window's revocation-by-non-occurrence guarantee (already sent = nothing to revoke), flood per-record FIFO queues, multiply conflict surfaces, and punish slow Backends. The user value — never losing typed work — is fully delivered by local Draft persistence (CAP-309) without touching the mutation lifecycle.

## Consequences
This rejection is architectural, not scoping: any future proposal must first show how Undo Window semantics survive.
