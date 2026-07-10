# Flow: Mid-Edit Drift Warning

**Mapping:** CAP-306 (non-blocking warning when an edited Record changes on the Backend). Bounds the accepted last-write-wins residual (RSK-06); upgraded by CAP-307 (P1) where the Backend cooperates.

Pure client-side protection requiring zero Backend cooperation: the form observes its Record's truth in the Replica.

```mermaid
sequenceDiagram
    actor O as Operator
    participant R6 as R6 View & Form Engine (Edit form, dirty)
    participant R2 as R2 Resource Replica
    participant R1 as R1 Provider Gateway
    participant BE as Backend

    O->>R6: editing (form dirty)
    R6->>R2: observing the Record's truth

    par concurrent change lands
        BE-->>R1: change feed event (Capability)<br/>OR staleness refetch result
        R1-->>R2: install new Record truth (last-write-wins)
    end

    R2-->>R6: change notification for observed Record
    R6->>R6: dirty + truth changed beneath loaded baseline?
    R6-->>O: non-blocking banner: "changed on the Backend"<br/>with view-changes affordance — editing continues (CAP-306)

    alt Operator saves anyway
        Note over R6: sparse patch limits the blast radius to<br/>dirty Fields only; outcome is last-write-wins<br/>(PRD-accepted) unless conditional-update<br/>Capability exists (P1, CAP-307)
    end
```
