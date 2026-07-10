# Flow: Workspace Arrangement & Restoration

**Mapping:** CAP-501 (split/tab/float Panels), CAP-502 (layout persists locally, restores intact). Supports NFC-22; implements RSK-08's total-restoration rule.

One process, one OS window per Workspace, shared warm Replicas. Restoration rebuilds Panels through the same string-resolution airlock Deep Links use.

```mermaid
sequenceDiagram
    actor O as Operator
    participant R5 as R5 Workspace Shell
    participant R9 as R9 Local State Store
    participant R4 as R4 Registry
    participant R6 as R6 View & Form Engine

    O->>R5: split / tab / float Panels (CAP-501)
    R5->>R5: update panel tree
    R5->>R9: persist layout (debounced; local file I/O)
    Note over R9: layout only — never Backend data,<br/>never credentials, never shareable state

    Note over O,R5: ... application relaunch ...

    R5->>R9: load persisted Workspace layouts
    R9-->>R5: layout descriptions (panel kinds as strings + state)
    loop each described panel
        R5->>R4: resolve panel kind string (Routing airlock, entry 2)
        alt resolvable
            R4-->>R5: concrete handler
            R5->>R6: mount View (Replica warms lazily on first read)
        else unresolvable (Resource removed since persist)
            R5->>R5: mount inert placeholder panel naming<br/>what it referenced (RSK-08: restore is total,<br/>never crashes, never drops the arrangement)
        end
    end
    R5-->>O: Workspace intact (NFC-22)
```
