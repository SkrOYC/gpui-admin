# Flow: Master-Detail Follow & Edit Protection

**Mapping:** CAP-503 (detail Follows list selection live; source close freezes follower), CAP-504 (Edit never Follows; focus-not-duplicate).

Follow is panel-local selection observation — causally obvious, browse-only. The two data-loss hazards (moving selection under an edit; twin edit panels) are made structurally impossible.

```mermaid
sequenceDiagram
    actor O as Operator
    participant LP as List Panel (R5/R6)
    participant DP as Detail Panel (Show, following)
    participant R5 as R5 Workspace Shell
    participant R2 as R2 Resource Replica

    O->>LP: open detail from a row
    R5->>DP: mount Show; observe LP's panel-local selection
    O->>LP: select another row
    LP-->>DP: selection change (observation)
    DP->>R2: read newly selected Record (warm or fetch)
    DP-->>O: detail tracks live (CAP-503)

    alt source List panel closes
        R5->>DP: observation dropped
        Note over DP: follower freezes on last value —<br/>now an ordinary independent panel
    end

    rect rgb(245, 240, 240)
        Note over O,R5: Edit protection (CAP-504)
        O->>LP: open EDIT from a row
        R5->>R5: Edit panels are keyed by (Resource, Record)
        alt already open for this Record
            R5-->>O: focus existing Edit panel — no duplicate,<br/>no divergent dirty twins
        else not open
            R5->>DP: mount independent Edit panel
            Note over DP: NEVER observes selection — a moving<br/>selection cannot discard unsaved edits,<br/>structurally rather than by prompt
        end
    end
```
