# Flow: Schema Migration & Declaration Reconcile

**Mapping:** CAP-104 (drift as build failure) — the *adopted* workflow after drift, which the v0.1.0 constitution named as accepted friction ("regenerate-and-reconcile") but never specified.

```mermaid
sequenceDiagram
    actor AD as Adopter
    participant CLI as gpui-admin CLI
    participant SNAP as Schema Snapshot (committed)
    participant CHK as check / derivation verifier
    participant DECL as Owned declarations

    BE->>BE: Backend schema migrates (Adopter-initiated or external)
    AD->>CLI: introspect <source> (the only online act)
    CLI->>SNAP: overwrite machine-owned Snapshot (reviewable diff)
    AD->>AD: review the diff in version control

    AD->>CHK: build (or gpui-admin check --format json)
    CHK-->>AD: drift list: exact stale declaration + file:line per item (NFC-12)<br/>(removed field referenced · nullability vs required ·<br/>relation target gone · projection over vanished field)
    loop each drifted declaration
        AD->>DECL: reconcile by hand — update, relocate, or delete;<br/>Scaffold is NOT re-run on owned files (one-time seed only)
    end
    AD->>CHK: rebuild
    CHK-->>AD: clean — typed artifacts regenerate against new truth
```

**Failure paths / guarantees:** reconciliation cannot be skipped — the build stays red until declarations agree. `check` and the build share one logic (two reporters, one verdict); CI runs `check` so drift lands in the pipeline, not on a laptop. Weak-truth changes surface with their conservative markers for review. Nothing here touches Operator data: this is a build-time-only loop.
