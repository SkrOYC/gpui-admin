# Flow: Adopter Onboarding & Showcase

**Mapping:** CAP-1001 (public showcase demonstrating every P0 capability against the verified Backend), CAP-1002 (fresh install → working admin over the Adopter's own Schema Source in under 30 minutes).

The adoption funnel as an executable journey. Every minute is architecture-relevant: each step leans on a container designed to make it short.

```mermaid
sequenceDiagram
    actor A as Adopter (fresh machine)
    participant TC as Toolchain (B1/B2/B3)
    participant B4 as B4 Derivation & Verification
    participant APP as Their Admin Application (R1–R9)
    participant SS as Their Schema Source
    participant BE as Their Backend

    Note over A: minute 0 — install toolchain,<br/>create project from starter
    A->>TC: capture own Schema Source
    TC->>SS: introspect (the one online act)
    TC-->>A: committed Snapshot (reviewable)

    A->>TC: scaffold chosen Resources
    TC-->>A: owned declarations — readable defaults,<br/>edit or accept (minute ~10)

    A->>B4: build (offline)
    alt clean
        B4-->>A: application binary
    else any declaration inconsistency
        B4-->>A: failure naming the exact line —<br/>the 30-minute path has no debugging maze in it
    end

    A->>APP: run against own Backend (configured Provider)
    APP->>BE: first reads through the Gateway
    APP-->>A: working admin — windowed Lists, forms,<br/>Workspace — inside the 30-minute budget (CAP-1002)

    Note over A,BE: CAP-1001 — the public showcase is this same<br/>journey, pre-walked: composition of every runtime<br/>container + the conformance-verified Provider against<br/>a real, publicly reachable Backend, demonstrating<br/>every P0 capability live
```
