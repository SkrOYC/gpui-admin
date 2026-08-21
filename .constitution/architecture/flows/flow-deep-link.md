# Flow: Deep Link Entry

**Mapping:** CAP-505 (Deep Link = one primary Panel under the application's declared scheme, fresh Workspace, personal layout never encoded), CAP-208 (Saved Query sharing: links may carry the typed Query). Session decision: single process, one OS window per Workspace, links activate the running instance. (OS-level registration is P1: CAP-506.)

Payload shape: readable path + query parameters under the Adopter-declared scheme (`<app-scheme>://<resource>/record/<id>` or `<app-scheme>://<resource>?filter=…&sort=…&search=…`). Typed queries serialize losslessly via snapshot-canonical field names; no base64 blobs — links are reviewable and pasteable.

```mermaid
sequenceDiagram
    actor O2 as Operator (receiving a link)
    participant APP as Application (single instance)
    participant R5 as R5 Workspace Shell
    participant R4 as R4 Registry
    participant R6 as R6 View & Form Engine
    participant R2 as R2 Resource Replica

    O2->>APP: open Deep Link (in-app address entry / share-sheet at P0;<br/>OS handoff for the declared scheme at P1)
    alt instance already running
        APP->>APP: activate running instance (no second process)
    else cold start
        APP->>APP: launch (<500ms to interactive, NFC-02)
    end

    APP->>R5: open FRESH Workspace window for the link
    Note over R5: the link identifies content, never anyone's<br/>layout — receiver's existing Workspaces untouched
    R5->>R4: resolve resource string + optional typed Query<br/>(Routing airlock, entry 1)
    alt resolvable
        R4-->>R5: concrete handler (+ deserialized Query if present)
        R5->>R6: mount the one primary Panel (Show of a Record,<br/>or a List pre-loaded with the link's Saved Query)
        R5->>R5: offer local bookmark of a received query (R9)<br/>— sharing carries content; bookmarks are personal
        R6->>R2: read/fetch Record
        Note over R2: Replicas are process-shared — if another<br/>window already warmed this Resource,<br/>the data is instantly available
    else unknown resource string
        R5-->>O2: clear "not found" panel (link rot is honest,<br/>not a crash)
    end
```
