# Flow: None-Mode Operation & Session Interruption

**Mapping:** CAP-701 (P0 — fully featured with no authentication). Also sketches the P1 routes it deliberately reserves (CAP-702/703) so their later addition is pure extension, not rework.

```mermaid
sequenceDiagram
    participant BOOT as Startup
    participant R7 as R7 Session & Access
    participant R5 as R5 Workspace Shell
    participant R6 as R6 View & Form Engine
    participant R1 as R1 Provider Gateway

    BOOT->>R7: auth mode from Adopter's owned composition
    alt none-mode (P0 default)
        R7-->>R5: no sign-in surface; identity absent
        Note over R6: every capability fully functional —<br/>private-network deployments are first-class (CAP-701)
    else pluggable auth configured (P1, CAP-702)
        R7-->>R5: sign-in surface before Workspace
    end

    rect rgb(245, 242, 238)
        Note over R1,R5: reserved interruption route (exists at P0,<br/>exercised at P1)
        R1-->>R7: normalized "unauthenticated" from any call
        R7->>R5: raise session interruption
        Note over R5: Workspace persists; dirty form input preserved;<br/>Views contain zero auth logic — re-auth resumes in place
    end

    rect rgb(238, 242, 245)
        Note over R1,R6: advisory affordances (P1, CAP-703)
        R7-->>R6: permission hints (hide/disable) — advisory only
        R1-->>R7: normalized "forbidden" corrects a stale hint
        Note over R6: enforcement lives in the Backend, always
    end
```
