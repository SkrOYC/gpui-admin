# Schema Snapshot — companion notes (B2)

**Contract file:** [`schema-snapshot.rs`](./schema-snapshot.rs) (the exemplar shape is normative; per-project content varies).

## Purpose

The committed, machine-owned capture of external Schema Source truth. The single input (besides Adopter declarations) to the derivation engine (B4), and the seed for the Scaffolder (B3).

## Invariants

1. **Overwrite-always, never hand-edited.** The generation banner is the marker; `gpui-admin check` and the derivation macros treat manual edits as drift.
2. **Frontend-neutral:** both introspection frontends emit identical structure for equivalent sources; the format vocabulary (types, nullability, `server_owned`, `unique`, relations) contains no source-dialect concepts (CAP-102 discipline, RSK-04).
3. **`server_owned` classification** drives `CreatePayload` generation — server-defaulted/generated Fields never appear in create forms (mutation-lifecycle flow).
4. **Relations reference target keys** resolved against registered Resources at boot (CAP-402); the Snapshot itself never fails on unknown targets (capture must succeed on partial selections).
5. **Weak-truth conservatism:** where the source understates (OpenAPI without nullability), the frontend captures the *safer* reading and the Scaffolder marks the inferred default visibly for Adopter review.

## Migration notes

Format marker (`snapshot format: N`) checked at derivation. Bump rules: additive metadata = same major format (older library ignores unknown const tables); shape changes to Record/Field emission = format bump, derivation rejects with a "re-run introspect" diagnostic (Compatibility Policy §4). The Snapshot is regenerable at any time from the source of truth — no data migration ever applies to it.
