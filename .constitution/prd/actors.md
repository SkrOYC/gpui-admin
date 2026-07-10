# Actors

## Primary

### Adopter

- **Role:** Professional developer who builds and maintains an admin application for their organization or product using the framework and toolchain.
- **Operating context:** Owns the admin application's code. May or may not own the Backend — it can be a legacy system, a third-party service, or a database managed by another team. Works in environments ranging from open startups to compliance-restricted or air-gapped networks.
- **Goals:**
  - Go from an existing Schema Source to a working, trustworthy admin in hours.
  - Trust that a schema migration can never silently ship a broken screen (Drift is a build failure).
  - Deliver operators a tool that feels professional without hand-building UI.
  - Own the result completely: no vendor dependency, no external runtime services required.
- **Frictions today:**
  - Hand-built CRUD screens are weeks of repetitive, error-prone work.
  - Web-based admin frameworks surface configuration mistakes at runtime, in front of operators.
  - Browser ceilings: one view at a time, sluggish large collections, page-shaped workflows.

## Secondary

### Operator

- **Role:** Support, operations, or back-office staff member who lives in the built admin application daily.
- **Operating context:** Works across several Resources at once (e.g., a customer, their orders, and related tickets simultaneously). Handles large collections. Interruptions are constant; work is resumed, not restarted.
- **Goals:**
  - Arrange a multi-Panel Workspace that matches their workflow and find it intact tomorrow.
  - Browse huge collections fluidly; follow master-detail relationships live.
  - Edit quickly with immediate feedback, a brief chance to revoke, and no silent loss of work.
- **Frictions today:**
  - Page-per-view admins force constant navigation and lost context.
  - Mistakes are hard to catch before they land; failures lose typed input.
  - Large tables paginate slowly and jump around.

### Contributor

- **Role:** External open-source developer who extends the framework — most commonly by authoring a new Provider or widget.
- **Operating context:** Arrives with knowledge of their own Backend, not of framework internals. Works against public documentation and the Conformance Suite.
- **Goals:**
  - Author a Provider for their Backend and *prove* it correct via the Conformance Suite without reading framework internals.
  - Understand clearly which behaviors are core and which are Capabilities.
- **Frictions today:** Frameworks whose extension points are underspecified, so third-party integrations rot or behave subtly differently from first-party ones.

## Contextual (non-user)

### Backend Owner

- **Role:** The party who controls the system of record — possibly a different team, a vendor, or nobody reachable.
- **Operating context:** May refuse or be unable to add version columns, realtime feeds, counting, or any client-serving affordance. Enforces authorization on their side regardless of what any client displays.
- **Why modeled:** This actor's *non-cooperation* is a design input, not an edge case. The product must be correct when the Backend Owner does nothing, and must reward them proportionally when they opt into Capabilities. Authorization truth always belongs to this actor — never to the client.
