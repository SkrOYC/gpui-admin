# Flow: Typed Filters, Sort & Search

**Mapping:** CAP-202 (typed Filters/sort, invalid Fields inexpressible, multi-column ordering expressible), CAP-203 (free-text Search where the Backend supports it; labeled client-side fallback where it does not). Column show/hide masks and prev/next navigation (CAP-207/209) operate over these same query entries without changing their identity.

Filter and sort identity comes from build-time typed field identity — a nonexistent Field cannot be referenced. A changed (filter, sort) pair is a *different query entry*; results are never client-simulated.

```mermaid
sequenceDiagram
    actor O as Operator
    participant R6 as R6 View & Form Engine
    participant R2 as R2 Resource Replica
    participant R1 as R1 Provider Gateway
    participant BE as Backend

    Note over R6: Filter/sort controls derive from typed<br/>field identity (B4). Invalid Field = build failure,<br/>not a runtime path.

    O->>R6: set Filter (typed predicate) / change sort
    R6->>R2: observe query under new (filter, sort) key
    R2->>R2: new query entry (prior entry stays warm,<br/>subject to staleness/eviction)
    R2->>R1: fetch first window for new key
    R1->>BE: translated read (closed operator set)
    alt operator unsupported by this Provider
        R1-->>R6: loud, typed rejection (never silent drop)
        R6-->>O: filter marked unsupported for this Backend
    else supported
        BE-->>R1: rows
        R1-->>R2: install; memoize view
        R2-->>R6: notification → rows render
    end

    O->>R6: type in Search box (free-text)
    alt Search supported (contract surface)
        R6->>R2: query with search term (part of query key)
        R2->>R1: fetch (debounced by R6)
        R1->>BE: search-capable read
        BE-->>R1: matches
        R1-->>R2: install → render
    else not supported
        R6-->>O: labeled client-side fallback: case-insensitive substring<br/>over LOADED window rows only, visibly badged "searching<br/>loaded rows" — honest partial coverage, never silent completeness
    end
```
