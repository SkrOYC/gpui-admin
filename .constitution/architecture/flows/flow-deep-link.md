# Flow: Deep Link Entry

**Mapping:** CAP-505 (Deep Link = one primary Panel, fresh Workspace, personal layout never encoded). Session decision: single process, one OS window per Workspace, links activate the running instance. (OS-level registration is P1: CAP-506.)

```mermaid
sequenceDiagram
    actor O2 as Operator (receiving a link)
    participant APP as Application (single instance)
    participant R5 as R5 Workspace Shell
    participant R4 as R4 Registry
    participant R6 as R6 View & Form Engine
    participant R2 as R2 Resource Replica

    O2->>APP: open Deep Link (in-app address entry at P0;<br/>OS handoff at P1)
    alt instance already running
        APP->>APP: activate running instance (no second process)
    else cold start
        APP->>APP: launch (<500ms to interactive, NFC-02)
    end

    APP->>R5: open FRESH Workspace window for the link
    Note over R5: the link identifies content, never anyone's<br/>layout — receiver's existing Workspaces untouched
    R5->>R4: resolve resource string (Routing airlock, entry 1)
    alt resolvable
        R4-->>R5: concrete handler
        R5->>R6: mount the one primary Panel (e.g., Show of a Record)
        R6->>R2: read/fetch Record
        Note over R2: Replicas are process-shared — if another<br/>window already warmed this Resource,<br/>the data is instantly available
    else unknown resource string
        R5-->>O2: clear "not found" panel (link rot is honest,<br/>not a crash)
    end
```
