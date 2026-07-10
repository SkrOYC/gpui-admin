//! NORMATIVE CONTRACT — Typed builder authoring surface (ADR-002).
//!
//! What Adopters write and what the Scaffolder emits. This is the product's
//! public face; changes follow Compatibility Policy §3.
//!
//! Design invariants:
//!  - Every Field reference is the generated field enum — typos are unrepresentable.
//!  - Declarations are plain values: loopable, shareable, unit-testable.
//!  - Divergence-only: omitted config = inferred default (materialized by the
//!    Scaffolder as explicit code, so "default" is always visible in the repo).

use crate::provider::{FieldIdent, Resource};

/// Entry point. `R` is the Snapshot-generated Resource type.
pub fn resource<R: Resource>() -> ResourceBuilder<R> { unimplemented!() }

pub struct ResourceBuilder<R: Resource> { /* opaque */ _marker: std::marker::PhantomData<R> }

impl<R: Resource> ResourceBuilder<R> {
    /// Human labels (Resource-level). Field labels live in `field()`.
    pub fn label(self, singular: &'static str, plural: &'static str) -> Self { unimplemented!() }

    /// Per-Field configuration. One call per diverging Field.
    pub fn field(self, field: R::Field, cfg: impl FnOnce(FieldBuilder<R>) -> FieldBuilder<R>) -> Self { unimplemented!() }

    /// Typed relation declaration. Target registration is boot-asserted (CAP-402).
    pub fn belongs_to<Target: Resource>(self, via: R::Field) -> Self { unimplemented!() }
    pub fn has_many<Target: Resource>(self, target_fk: Target::Field) -> Self { unimplemented!() }

    /// Per-View projection overrides (ordered). Only where a View diverges
    /// from the default (all Fields, declaration order).
    pub fn list(self, cfg: impl FnOnce(ListBuilder<R>) -> ListBuilder<R>) -> Self { unimplemented!() }
    pub fn show(self, projection: impl Into<Projection<R>>) -> Self { unimplemented!() }
    pub fn create(self, projection: impl Into<Projection<R>>) -> Self { unimplemented!() }
    pub fn edit(self, projection: impl Into<Projection<R>>) -> Self { unimplemented!() }

    /// Finalize into the registrable declaration consumed by the Registry (R4).
    pub fn build(self) -> ResourceDecl<R> { unimplemented!() }
}

pub struct FieldBuilder<R: Resource> { _marker: std::marker::PhantomData<R> }

impl<R: Resource> FieldBuilder<R> {
    pub fn label(self, text: &'static str) -> Self { unimplemented!() }
    /// Input widget override; default inferred from Snapshot type by the Scaffolder.
    pub fn widget(self, widget: Widget) -> Self { unimplemented!() }
    pub fn required(self, required: bool) -> Self { unimplemented!() }
    pub fn filterable(self) -> Self { unimplemented!() }
    pub fn sortable(self) -> Self { unimplemented!() }
    /// Domain validation over the PARSED typed value (layer 2 of the form
    /// state machine; layer 1 parse is type-derived).
    pub fn validate(self, rule: fn(&FieldValue) -> Result<(), String>) -> Self { unimplemented!() }
    /// Escape hatch: custom input widget (CAP-903, P1 — signature reserved at v0.1).
    pub fn custom_widget(self, factory: CustomWidgetFactory<R>) -> Self { unimplemented!() }
}

pub struct ListBuilder<R: Resource> { _marker: std::marker::PhantomData<R> }

impl<R: Resource> ListBuilder<R> {
    pub fn projection(self, fields: impl Into<Projection<R>>) -> Self { unimplemented!() }
    pub fn default_sort(self, field: R::Field, dir: crate::provider::SortDir) -> Self { unimplemented!() }
    pub fn page_hint(self, rows: usize) -> Self { unimplemented!() }
}

/// Ordered Field list; `From<[R::Field; N]>` so call sites read as arrays.
pub struct Projection<R: Resource> { _marker: std::marker::PhantomData<R> }

/// Core widget set (each maps to a verified substrate component).
pub enum Widget { Text, TextArea, Number, Switch, Date, EnumSelect, RelationSelect }

pub enum FieldValue { /* typed scalar mirror of FilterValue */ }
pub struct CustomWidgetFactory<R: Resource> { _marker: std::marker::PhantomData<R> }

/// Built declaration; registered at startup. Registration set is boot-asserted
/// complete (relation targets + panel kinds — Routing airlock, both entries).
pub struct ResourceDecl<R: Resource> { _marker: std::marker::PhantomData<R> }

// ---------------------------------------------------------------------------
// Canonical example (Scaffolder output shape — doc-normative)
// ---------------------------------------------------------------------------
//
// pub fn users() -> ResourceDecl<schema::User> {
//     resource::<schema::User>()
//         .label("User", "Users")
//         .field(UserField::Email, |f| f.required(true).validate(rules::email))
//         .field(UserField::Status, |f| f.widget(Widget::EnumSelect).filterable().sortable())
//         .belongs_to::<schema::Organization>(UserField::OrgId)
//         .list(|l| l
//             .projection([UserField::Name, UserField::Email, UserField::Status])
//             .default_sort(UserField::Name, SortDir::Asc))
//         .build()
// }
