# Flow: Multi-Backend Binding & Cross-Provider Relations

**Mapping:** CAP-607 (per-Resource Provider Binding), CAP-405 (cross-provider Relations, structurally verified only). Extends the Provider contract flow.

Each Resource carries a build-time Provider Binding (default binding covers single-Backend apps). Each Provider owns a namespaced Schema Snapshot; the Registry resolves handlers per binding and keeps Replicas warm per Resource regardless of which Backend serves them.

```mermaid
sequenceDiagram
    participant APP as Adopter application (build time)
    participant R4 as R4 Registry
    participant R6 as R6 View & Form Engine
    participant R2A as R2 Replica (users)
    participant R2B as R2 Replica (invoices)
    participant GA as Gateway (Provider A)
    participant GB as Gateway (Provider B)
    participant BEA as Backend A
    participant BEB as Backend B

    Note over APP: build: declarations verified against their OWN<br/>binding's Snapshot; cross-binding Relation declared<br/>by hand → structural check only (target registered,<br/>types compatible) → visible UNVERIFIED marker (CAP-405)

    APP->>R4: register Resources with bindings
    R4->>R4: boot assertions: relation targets registered across<br/>bindings; missing target = launch failure (CAP-402)

    Note over R6,BEB: run time — same one-way truth chain per binding:<br/>View demands from the target's Replica; the Replica<br/>fetches through its own Gateway. No second data path.
    R6->>R2A: observe query (users bound to A)
    R2A->>GA: fetch window
    GA->>BEA: read
    BEA-->>GA: rows
    GA-->>R2A: install → notify → render
    R6->>R2B: embedded has-many list for invoices (bound to B)<br/>= an ordinary query entry in B's Replica (CAP-403)
    R2B->>GB: fetch window
    GB->>BEB: read
    BEB-->>GB: rows
    GB-->>R2B: install → notify → related List renders
    Note over R6: embedded foreign-binding lists reuse the SAME<br/>windowed/freshness machinery; their latency and failures<br/>are contained to that panel (degraded UX rules apply)
```

**Failure paths:** one Backend unreachable degrades only that binding's panels (connectivity state is per-Gateway); a cross-provider relation renders contained errors when its foreign side fails — the domestic side stays fully usable. Verification honesty: Snapshot-backed checks exist only within a binding; anything crossing bindings says so in build output.
