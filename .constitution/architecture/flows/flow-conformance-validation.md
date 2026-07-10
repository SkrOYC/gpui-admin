# Flow: Provider Conformance Validation

**Mapping:** CAP-603 (public Conformance Suite validates any Provider). Enables CAP-604 (first-party Provider verified by the same suite) and CAP-1003 (P1 — Contributors prove third-party Providers).

The suite is the contract's executable meaning. Rule RSK-07 binds: a contract change without a suite change is an invalid change.

```mermaid
sequenceDiagram
    actor C as Contributor / Maintainer
    participant P as Candidate Provider implementation
    participant C1 as C1 Conformance Suite
    participant BE as Real or scripted Backend

    C->>C1: run suite against P
    loop contract obligations
        C1->>P: elementary reads, windowed lists,<br/>sparse mutations with previous-state
        P->>BE: translate & execute
        BE-->>P: responses
        P-->>C1: typed results / normalized errors
        C1->>C1: verify semantics (ordering honesty, error taxonomy<br/>fidelity, window correctness, idempotent reads)
    end
    loop declared Capabilities only
        C1->>P: capability obligations (feed delivery + reconnect,<br/>cursor sequencing, conditional-update conflicts, counting)
        C1->>C1: verify OR confirm capability is honestly undeclared
    end
    alt all obligations met
        C1-->>C: conformant (declared Capability set certified)
    else violations
        C1-->>C: precise failure list — which obligation,<br/>which semantics, observed vs required
    end
```
