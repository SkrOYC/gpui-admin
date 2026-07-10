# Workspace Layout Store — companion notes (R9)

**Contract file:** [`workspace-layout.schema.json`](./workspace-layout.schema.json) (JSON Schema 2020-12, generated from the persisted types via `schemars` — code and contract cannot drift).

## Purpose & location

Per-machine persistence of Workspace arrangements (CAP-502, NFC-22). Lives in the platform config directory (`dirs 6`): one index file (`workspaces.json`) conforming to this schema. Saves are debounced and atomic (write-temp + rename).

## Invariants

1. **No Backend data, no credentials — by type.** The persisted structs cannot represent Records or secrets; `panel_kind` + minimal `state` (ids, query descriptions) only.
2. **Total restoration (RSK-08):** unknown `format_version` or unresolvable `panel_kind` restores as an inert placeholder panel naming what it referenced. Restoration never crashes and never drops the arrangement.
3. **Deep Links never pass through this store** — shared addressing carries content identity only; layouts are personal (CAP-505 boundary).
4. Crash exits restore the last persisted state (debounce window is the accepted loss bound).

## Migration notes

`format_version` bump + in-loader migration function per version step. Migrations transform old envelopes to current; on any migration failure, fall back to placeholder-panel restoration of whatever subtree survives. Old files are backed up alongside (`.bak-vN`) before rewrite.
