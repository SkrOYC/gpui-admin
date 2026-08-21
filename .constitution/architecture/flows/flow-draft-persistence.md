# Flow: Draft Persistence & Restoration

**Mapping:** CAP-309 (local Draft persistence, P0); NFC-21/NFC-22 adjacency. Companion to the Mutation Lifecycle flow — Drafts live entirely outside it, which is the point.

A Draft is operator input persisted locally, never dispatched. It interacts with backend truth only at restoration time, through the already-existing mid-edit drift warning (CAP-306).

```mermaid
sequenceDiagram
    actor O as Operator
    participant R6 as R6 View & Form Engine
    participant R9 as R9 Local State Store
    participant R2 as R2 Resource Replica

    O->>R6: types into Edit form (dirty tracking active)
    R6->>R9: debounced persist Draft {resource, record_id, typed values}
    Note over R9: same envelope family as layout state;<br/>no credentials, no wholesale Record copies beyond<br/>the edited Field values themselves

    alt session continues
        Note over R6,R9: Draft kept until confirmed save or discard
    else crash / quit mid-edit
        Note over O,R6: relaunch (<500ms, NFC-02)
        O->>R5: reopen the Record (Workspace restore or navigation)
        R5->>R6: mount Edit Panel
        R6->>R9: fetch Draft for (resource, record_id)
        alt Draft exists
            R9-->>R6: restore verbatim (parse-preserving state incl.<br/>values that failed domain rules)
            R6->>R2: compare against current backend truth
            alt Record changed since drafting
                R6-->>O: Draft restored AND mid-edit drift warning shown<br/>(what changed on the Backend while you were away)
            else unchanged
                R6-->>O: Draft restored silently
            end
        else no Draft
            R6-->>O: fresh form
        end
    end

    O->>R6: confirm save (enters Mutation Lifecycle flow) / explicit discard
    R6->>R9: clear Draft
```

**Failure paths:** Draft writes are best-effort local I/O — a failed write degrades to the pre-framework status quo (typed work lives in the open form only), never blocks typing. A Draft for a deleted Record is dropped at restoration with a notice. Drafts never cross the Provider Gateway; offline drafting works identically.
