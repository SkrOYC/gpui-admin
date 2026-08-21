# Flow: Object Upload Lifecycle (file/image Fields)

**Mapping:** CAP-608 (Object Store port, first-party implementation against the dominant de-facto object-storage standard), with PB-2 ruling (object-first inside the staged lifecycle). Companion to the Mutation Lifecycle flow.

Ordering invariant: a committed Record never carries a dangling reference. Bytes land before the record write; every failure path either lands nothing or leaves an observable, best-effort-cleaned orphan.

```mermaid
sequenceDiagram
    actor O as Operator
    participant R6 as R6 View & Form Engine
    participant R3 as R3 Mutation Coordinator
    participant R10 as R10 ObjectStore Port
    participant OS as Object Store (configured)
    participant R1 as R1 Provider Gateway
    participant BE as Backend

    Note over O,R6: unconfigured app → Field degrades to<br/>reference-as-text input; this flow never runs

    O->>R6: attach file/image in form
    O->>R6: save
    R6->>R3: mutation intent (reference Field staged as pending-upload)

    R3->>R2: stage overlay (Pending Change)
    alt Operator revokes during Undo Window
        R3->>R3: revoke — nothing dispatched, nothing uploaded
    else window elapses
        R3->>R10: put object (bytes; progress surfaced to R6)
        R10->>OS: upload
        alt upload succeeds
            OS-->>R10: object reference (key/URL)
            R10-->>R3: reference bound into patch
            R3->>R1: dispatch record mutation carrying reference
            alt confirmed
                R1-->>R3: settled: confirmed
            else record rejected / transport failure
                R1-->>R3: settled: failed
                R3->>R2: roll back overlay (subtraction — replica truth intact)
                R3->>R10: best-effort delete of uploaded object
                R10->>OS: cleanup delete
                Note over R3,OS: cleanup failure is NOT silent: logged to R8<br/>diagnostics as an orphaned-object report.<br/>Scoped exception to subtraction-only rollback:<br/>compensating action on infrastructure, never on records
            end
        else upload fails
            R10-->>R3: object error (taxonomy)
            R3->>R2: roll back overlay
            R3->>R5: failure event → preserved input + retry/discard (CAP-304)
            R3->>R6: form state refreshed from overlay rollback
            Note over R3,BE: NO record dispatch occurred — Backend untouched
        end
    end
```

**Failure path summary:** crash between upload success and record settlement leaves an orphaned object discoverable via R8's orphan report sweep; it can never leave a committed Record pointing at nothing.
