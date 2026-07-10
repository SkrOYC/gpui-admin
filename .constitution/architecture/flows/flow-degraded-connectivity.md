# Flow: Degraded Connectivity & Recovery

**Mapping:** Unhappy-path companion to CAP-201/304 and NFC-23; implements the session-decided degraded mode and the reconnect rule from `resilience.md` (PRD boundary: `out-of-scope/offline-first.md` — graceful interruption, no durable offline queue).

```mermaid
sequenceDiagram
    actor O as Operator
    participant R6 as R6 View & Form Engine
    participant R2 as R2 Resource Replica
    participant R1 as R1 Provider Gateway
    participant R5 as R5 Workspace Shell
    participant BE as Backend

    BE--xR1: requests failing
    R1->>R1: connectivity: healthy → degraded → unreachable
    R1-->>R5: state change
    R5-->>O: persistent status banner (honest, non-modal)

    Note over R2,R6: reads — warm data stays fully browsable<br/>with staleness indication; failed window regions<br/>render placeholder-with-retry, never an empty lie

    O->>R6: attempts a save while unreachable
    R6-->>O: attempt fails fast → preserved-input retry state<br/>(no durable queue; retry is an Operator act)

    BE-->>R1: connectivity restored
    R1->>R2: global staleness pass (same rule as change-feed<br/>reconnect: assume everything was missed)
    R2->>R1: refetch observed queries (serve stale, refresh)
    R1-->>R5: healthy
    R5-->>O: banner clears; Panels converge to Backend truth<br/>without manual refresh (NFC-23)
```
