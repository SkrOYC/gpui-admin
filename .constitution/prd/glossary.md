# Glossary — Ubiquitous Language

Canonical domain terms. Downstream stages, code, and conversation must use these terms exactly; listed synonyms are prohibited.

| Term | Definition | Do Not Use |
| :--- | :--- | :--- |
| Adopter | The developer who uses the framework and toolchain to build an admin application. | user, dev, integrator, consumer |
| Operator | The person who uses a built admin application to browse and edit data. | end user, admin, staff, agent |
| Backend | The sovereign external system of record an admin application reads from and writes to. Its owner may be uncooperative or a third party. | server, API, database (when meaning the system) |
| Provider | The pluggable contract through which an admin application communicates with a Backend. | adapter, connector, driver |
| Capability | An optional Backend cooperation feature (e.g., realtime change feed, cursor paging, conditional update) a Provider declares; absence never breaks correctness. | feature flag, extension, add-on |
| Resource | A managed collection of like Records exposed for administration. | model, table, entity, collection |
| Record | A single item belonging to a Resource. | row, document, item, instance |
| Field | A named, typed attribute of a Record. | column, property, attribute |
| Relation | A declared, typed link from one Resource to another. | association, reference, foreign key |
| Schema Source | The external machine-readable truth describing Resources (a database catalog or an API description). | schema file, spec (unqualified) |
| Schema Snapshot | The versioned, human-reviewable capture of a Schema Source produced offline by the toolchain. | generated schema, cache |
| Scaffold | The one-time generated, thereafter Adopter-owned declaration of a Resource, with inferred defaults written out explicitly. | boilerplate, template, codegen output |
| Drift | Divergence between owned declarations and regenerated Schema Snapshot truth, or a Record that no longer matches its declared shape. | mismatch, corruption, desync |
| View | A presentation of a Resource. Core four: List, Show, Create, Edit. | page, screen, route |
| Projection | The ordered subset of Fields a View displays. | layout, column set |
| Filter | A typed predicate constraining which Records a List shows. | query, criteria |
| Search | Free-text matching across a Resource, distinct from Filters. | full-text, quick find |
| Workspace | The persistent arrangement of Panels an Operator composes. | layout, session, desktop |
| Panel | One dockable unit (split, tabbed, or floating) hosting a View. | tab, window, pane |
| Follow | Live master-detail linkage in which a detail Panel tracks a List Panel's selection. Browsing-only. | sync, link, binding |
| Pending Change | An optimistic local mutation shown immediately but not yet confirmed by the Backend. | draft, dirty write, queued edit |
| Undo Window | The brief period during which a Pending Change can be revoked before it is dispatched; revoked changes never reach the Backend. | grace period, rollback |
| Deep Link | A shareable address identifying a single primary Panel; never encodes a full Workspace. | URL, permalink |
| Conformance Suite | The public test harness any Provider implementation must pass to be considered valid. | test kit, certification |
