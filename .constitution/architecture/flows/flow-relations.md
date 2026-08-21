# Flow: Relations (Pick-One & Related Lists)

**Mapping:** CAP-401 (belongs-to inputs, has-many Lists), CAP-402 (unknown targets cannot ship), CAP-403 (related Lists behave identically to top-level Lists), CAP-404 (many-to-many over detected Junctions — compiles to a Junction-filtered query on the target Replica; multi-select pick-one where declared; non-pure junctions fall back to ordinary has-many editing).

The unification: a related List **is** a query entry in the *target* Resource's Replica whose filter is the relation Field. Every List behavior (windowing, invalidation, Pending Changes) applies with no second mechanism.

```mermaid
sequenceDiagram
    participant BOOT as Startup
    participant R4 as R4 Registry
    actor O as Operator
    participant R6 as R6 View & Form Engine<br/>(Show View, Resource A)
    participant R2B as R2 Replica of Resource B (target)
    participant R1 as R1 Provider Gateway
    participant BE as Backend

    Note over BOOT,R4: CAP-402 — completeness assertion
    BOOT->>R4: register all derived Resource handlers
    R4->>R4: assert every declared Relation target registered
    alt target missing
        R4-->>BOOT: startup failure with clear message<br/>(never a mid-session surprise)
    end

    O->>R6: open Record of A (has-many B)
    R6->>R4: resolve relation edge A→B (first use)
    R4->>R4: resolve once, cache concrete edge;<br/>warm B's Replica if cold (warm = visited OR referenced)
    R4-->>R6: concrete typed edge

    R6->>R2B: observe query {relation Field = this Record}
    Note over R2B: an ordinary query entry — windowing,<br/>memoized view, staleness, overlay all apply (CAP-403)
    R2B->>R1: fetch window
    R1->>BE: related read
    BE-->>R1: rows
    R1-->>R2B: install → notification → related List renders

    Note over R2B: reparenting is automatic: a B whose relation<br/>Field changes touches a filter Field of this query →<br/>field-aware invalidation refetches membership

    Note over R6: belongs-to input (CAP-401): pick-one control<br/>searches B through the same Replica/query path

    Note over R6,R2B: many-to-many (CAP-404): declared over the Junction J;<br/>renders as an embedded List of B filtered {junction FK = this A}<br/>— same query-entry machinery; pick-one gains multi-select,<br/>each selection staging a Pending Change on a J row through<br/>the standard mutation lifecycle
```
