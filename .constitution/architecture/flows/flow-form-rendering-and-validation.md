# Flow: Form Rendering, Input Catalog & Validation Layers

**Mapping:** CAP-312 (declaration-derived input catalog incl. exact-decimal numbers and Object-Store-backed file/image inputs), CAP-211 (type-faithful temporal rendering), supporting CAP-301 (entry-time validation) and CAP-305 (backend precedence). The read-side companion is the Mutation Lifecycle flow.

```mermaid
sequenceDiagram
    participant R6 as R6 View & Form Engine
    participant R4 as R4 Registry (derived artifacts)
    participant OS as Object Store (when configured)
    actor O as Operator

    Note over R4,R6: build time: Scaffolder materialized widget inference<br/>from Snapshot truth as explicit owned declarations;<br/>B4 verified them against that truth (Drift = build failure)

    R4->>R6: mount form from declaration (widgets, rules, layout sections)
    loop each Field on entry
        R6->>R6: render per catalog — exact-decimal Number for exact-numeric<br/>truth; temporal kinds type-faithfully (instants in machine zone,<br/>Adopter override honored; naive/date-only verbatim, zone-nature marked)
        O->>R6: input
        alt parse fails
            R6-->>O: typed error at the field edge; raw input preserved
        else declarative rule fails (required/bounds/length/pattern/membership)
            R6-->>O: rule message at the field; value kept
        else Adopter closure rule fails
            R6-->>O: closure message at the field; value kept
        else valid
            R6->>R6: typed value held for submission assembly
        end
    end

    O->>R6: save → Mutation Lifecycle flow
    Note over R6: Backend validation arriving later lands on the exact Field<br/>and OVERRIDES any client verdict (CAP-305 precedence)

    note over R6,OS: file/image Fields route through the Object Upload<br/>Lifecycle flow when an Object Store is configured;<br/>reference-as-text inputs when it is not
```

**Failure paths:** every validation layer fails soft at its own field with input preserved; no layer can block unrelated fields. Widget inference diverging from truth is a build failure, never a runtime surprise. Unconfigured Object Store degrades file/image inputs honestly rather than hiding or faking them.
