# Flow: Provider Contract & Capability Negotiation

**Mapping:** CAP-601 (sole pluggable communication path, elementary baseline), CAP-602 (opt-in Capabilities; absence degrades richness, never correctness), CAP-604 (a verified Provider against a real Backend ships in v1 — posture note below).

How the client shapes its own behavior from what the Provider declares — pagination physics and freshness strategy are *discovered*, not configured.

```mermaid
sequenceDiagram
    participant BOOT as Startup
    participant R1 as R1 Provider Gateway
    participant R2 as R2 Resource Replica
    participant R6 as R6 View & Form Engine
    participant BE as Backend

    BOOT->>R1: install configured Provider
    R1-->>BOOT: declared Capabilities<br/>{random-access paging? change feed? conditional update? counting?}

    rect rgb(240, 245, 240)
        Note over R2,R6: pagination physics (per Capability)
        alt random-access paging
            R6->>R2: any visible range demandable → true windowing,<br/>proportional scrolling with totals
        else sequential-only
            R6->>R2: frontier loading; no random jumps;<br/>totals treated as absent-able (CAP-606 posture at P1;<br/>baseline correctness at P0)
        end
    end

    rect rgb(240, 240, 245)
        Note over R1,R2: freshness strategy (per Capability)
        alt change feed declared
            BE-->>R1: event stream keeps warm Replicas fresh
        else no feed
            R2->>R2: staleness-driven refetch on observed queries<br/>(serve stale, refresh in background)
        end
    end

    Note over R1: every failure normalized into the shared error<br/>taxonomy before leaving the Gateway — no raw<br/>transport error crosses this boundary (CAP-601)

    Note over BE: CAP-604 posture: the first-party Provider is a<br/>consumer of the same public contract, verified by the<br/>Conformance Suite — zero private privileges
```
