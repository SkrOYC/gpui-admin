# ADR-010: Workspace Layout Persistence — Versioned JSON in the Platform Config Directory

**Status:** Accepted.

## Context

R9 must persist Workspace layouts locally (NFC-22) with total restoration (RSK-08). The substrate's dock already dumps/loads a serde layout state keyed by panel names (verified: `DockAreaState`, `PanelRegistry`); the choice is our envelope, location, and evolution rules.

## Decision

- **Format:** JSON, schema in `data-models/workspace-layout.schema.json` (generated from the persisted types via `schemars` so the contract cannot drift from code).
- **Location:** platform config directory (`dirs 6`), one file per named Workspace plus an index; atomic write-and-rename on save (debounced).
- **Envelope:** `{ format_version, workspaces: [...] }` wrapping the substrate's panel-state payloads. Panel payloads carry `panel_kind` strings resolved through the Registry airlock at restore.
- **Evolution:** `format_version` bump + in-loader migration; unknown versions or unresolvable `panel_kind`s restore as inert placeholder panels — restoration is total, never a crash, never a silent drop.

## Consequences

- Human-inspectable, diffable local state; trivially excluded from any sync/backup concerns (contains no Backend data or credentials — enforced by type: the persisted structs cannot represent them).
- JSON Schema doubles as documentation for Adopters building layout tooling.
- We own migrations forever; the placeholder-panel rule caps their worst case at cosmetic.
