# Flow: Mutation Lifecycle (Create / Edit / Delete)

**Mapping:** CAP-301 (typed forms, entry-time validation), CAP-302 (honest optimistic presentation), CAP-303 (Undo Window; revoked = never sent; duration Adopter-configurable per Resource), CAP-304 (rejection preserves input, retry/discard), CAP-305 (Backend validation lands on the exact Field, overriding client verdicts), CAP-308 (destructive actions require explicit confirmation before staging). Quick Edit (CAP-310) rides this same lifecycle with the row as the feedback surface instead of a form. File/image Fields add the Object Upload Lifecycle flow at dispatch. Supports NFC-03 (one-frame optimistic visibility), NFC-21.

The system's most important flow. Staged → in-flight → settled, with the Undo Window making the common regret path a non-event.

```mermaid
sequenceDiagram
    actor O as Operator
    participant R6 as R6 View & Form Engine
    participant R3 as R3 Mutation Coordinator
    participant R2 as R2 Resource Replica
    participant R5 as R5 Workspace Shell
    participant R1 as R1 Provider Gateway
    participant BE as Backend

    O->>R6: edit Fields (each keystroke: parse → rules — declarative<br/>core set + Adopter closures; typed value preserved even when<br/>a rule fails; Draft debounced to R9 — see Draft Persistence flow)
    O->>R6: save
    R6->>R6: cross-field validation over typed values
    R6->>R3: mutation intent (sparse patch: dirty Fields only,<br/>+ previous state)

    R3->>R2: stage overlay (Pending Change)
    R2-->>R6: view reflects change within one frame (NFC-03)
    Note over R6: pending CREATE renders in a provisional band —<br/>never faked into a server-ordered position (CAP-302)
    R3->>R5: undo affordance visible (Undo Window running)

    Note over O,R5: DELETE path additionally requires explicit confirmation<br/>BEFORE staging (CAP-308) — confirmation gates intent,<br/>the Undo Window still guards regret after it

    alt Operator revokes during Undo Window
        O->>R5: undo
        R5->>R3: revoke
        R3->>R2: remove overlay (subtraction; truth restored)
        Note over R3,BE: NOTHING was ever dispatched (CAP-303)
    else window elapses
        R3->>R3: per-Record FIFO (await prior settlement if any)
        R3->>R1: dispatch
        R1->>BE: mutation
        alt confirmed
            BE-->>R1: canonical result
            R1-->>R3: settled: confirmed
            R3->>R2: install truth; graduate & clear overlay;<br/>membership-affecting? mark queries stale → refetch
        else Backend field validation
            BE-->>R1: per-field rejection
            R1-->>R3: settled: field-validation (typed per Field)
            R3->>R2: roll back overlay (subtraction)
            R3->>R6: errors land on the exact Fields (CAP-305)
        else other rejection / transport failure
            R1-->>R3: settled: failed (taxonomy)
            R3->>R2: roll back overlay
            alt originating form still open
                R3->>R6: preserved input + retry/discard in place
            else form closed
                R3->>R5: notification with preserved input,<br/>retry / edit / discard (CAP-304)
                Note over R5: failed CREATE stays in the provisional<br/>band in an error state — the band doubles<br/>as the failure surface
            end
        end
    end
```
