# Flow: Schema Capture & Scaffold

**Mapping:** CAP-101 (offline Snapshot capture), CAP-102 (two Schema Source classes, one semantics), CAP-103 (one-time owned Scaffold).

The Adopter's first contact: from external truth to owned declarations. Both frontends converge on identical Snapshot semantics; scaffolding is a one-time act producing code the Adopter owns thereafter.

```mermaid
sequenceDiagram
    actor A as Adopter
    participant B1 as B1 Introspection Frontend<br/>(catalog OR api-description)
    participant SS as Schema Source (external)
    participant B2 as B2 Snapshot Store
    participant B3 as B3 Scaffolder

    A->>B1: capture (explicit invocation)
    B1->>SS: introspect structure (only network moment)
    SS-->>B1: types, nullability, defaults,<br/>server-owned fields, relations
    B1->>B1: normalize to neutral Snapshot semantics
    B1->>B2: write Snapshot (file handoff, overwrite-always)
    Note over B2: Reviewable diff in version control.<br/>Machine-owned; hand-edits prohibited.

    A->>B3: scaffold Resource (one-time)
    B3->>B2: read Snapshot (hermetic, no network)
    B3->>B3: infer labels, input kinds, requiredness,<br/>Relations, Projections
    B3-->>A: explicit, editable Resource declarations
    Note over A: Adopter reviews, edits, owns.<br/>Scaffolder never regenerates over these.

    alt weak Schema Source truth (api-description lacking nullability/relations)
        B3-->>A: conservative defaults materialized VISIBLY<br/>with markers for Adopter review
    end
```
