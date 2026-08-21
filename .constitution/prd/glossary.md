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
| Deep Link | A shareable address under the application's declared scheme, identifying one primary Panel and optionally the typed Query behind it; never encodes a full Workspace or personal layout. | URL, permalink |
| Object Store | The pluggable external storage for binary objects (files, images), addressed by an independent contract beside the Provider; ships with an industry-standard first-party implementation. Unconfigured apps degrade to reference-as-text fields, never break. | blob storage, file system |
| Saved Query | A named, shareable Filter/Sort/Search combination over one Resource, serializable into a Deep Link and bookmarkable locally by an Operator. | preset view, smart folder |
| Draft | Operator form input persisted locally before dispatch, restored verbatim after relaunch or crash, cleared on confirmed save or discard. Never sent to the Backend. | autosave, session state |
| Provider Binding | The build-time association between one Resource and one Provider, allowing a single application to span multiple Backends. | connection mapping |
| Junction | A Resource whose Snapshot shows exactly two foreign-key relations, serving as the join table for a Many-to-Many declaration between two other Resources. | join table, pivot |
| Quick Edit | Single-Field inline editing directly in a List row for simple widgets, dispatched through the same Pending Change lifecycle as forms; not an Edit Panel. | inline editing, cell edit |
| Conformance Suite | The public test harness any Provider implementation must pass to be considered valid. | test kit, certification |
