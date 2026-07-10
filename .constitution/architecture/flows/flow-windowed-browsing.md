# Flow: Windowed Browsing (List & Show)

**Mapping:** CAP-201 (fluid Lists at any collection size), CAP-204 (Show View), CAP-205 (per-Record fault containment). Supports NFC-01 (frame-rate scrolling at 10M Records), NFC-20.

The identity flow: a List renders only visible rows from a memoized query view; scroll misses demand windows asynchronously; a malformed Record degrades exactly one row.

```mermaid
sequenceDiagram
    actor O as Operator
    participant R5 as R5 Workspace Shell
    participant R4 as R4 Registry
    participant R6 as R6 View & Form Engine
    participant R2 as R2 Resource Replica
    participant R1 as R1 Provider Gateway
    participant BE as Backend

    O->>R5: open List panel for a Resource
    R5->>R4: resolve Resource handler
    R4->>R4: warm Replica if cold (owning handle held)
    R4-->>R5: concrete handler
    R5->>R6: mount List View
    R6->>R2: observe query (filter, sort); demand visible range

    alt range miss
        R2->>R1: fetch window [i..j] (async; generation-tagged)
        R6-->>O: placeholders for unloaded rows (UI never blocks)
        R1->>BE: windowed read
        BE-->>R1: rows + total (if counting Capability)
        R1-->>R2: typed results
        R2->>R2: install records; rebuild memoized view
        R2-->>R6: change notification (coalesced per frame)
        R6-->>O: rows render
    else range loaded
        R2-->>R6: read-time view, O(1)
    end

    O->>R6: scroll (visible range changes)
    R6->>R2: demand new range (in-flight ranges deduplicated)

    alt one Record malformed (Drift at run time)
        R2->>R2: store as contained per-Record fault
        R6-->>O: single bounded error row;<br/>all sibling rows fully usable (CAP-205)
    end

    O->>R6: open a Record
    R6->>R2: read Record (warm) / fetch one (miss)
    R6-->>O: Show View through declared Projection (CAP-204)
```
