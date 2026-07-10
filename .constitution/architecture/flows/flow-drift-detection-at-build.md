# Flow: Drift Detection at Build

**Mapping:** CAP-104 (Drift fails the build, pointing at the stale declaration). Supports NFC-11 (air-gapped build), NFC-12 (exact-location failures).

The correctness pillar's enforcement moment. A schema migration happened; the Adopter recaptures; the build refuses until owned declarations agree with truth.

```mermaid
sequenceDiagram
    actor A as Adopter
    participant B1 as B1 Introspection Frontend
    participant B2 as B2 Snapshot Store
    participant B4 as B4 Derivation & Verification

    Note over A: Backend schema migrated<br/>(e.g., a Field became optional)
    A->>B1: re-capture
    B1->>B2: overwrite Snapshot (reviewable diff)

    A->>B4: build (fully offline)
    B4->>B2: read Snapshot (file handoff)
    B4->>B4: unify owned declarations with Snapshot truth

    alt declarations agree with truth
        B4-->>A: statically specialized artifacts<br/>(typed Records, field identity, View derivations)
    else Drift detected
        B4-->>A: BUILD FAILURE naming the exact stale<br/>declaration and its location (NFC-12)
        Note over A: Fix guided declaration-by-declaration;<br/>the application cannot exist while stale.
    end
```
