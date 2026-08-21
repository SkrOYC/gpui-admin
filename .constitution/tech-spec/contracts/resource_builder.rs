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

    /// Provider Binding (CAP-607). Omitted = the application's default binding.
    /// Declared keys are verified at build against registered Providers.
    pub fn provider(self, key: &'static str) -> Self { unimplemented!() }

    /// Undo Window duration override for this Resource's mutations (CAP-303).
    /// Omitted = framework default; the Scaffolder materializes the default
    /// explicitly so it is visible and owned (infer-then-own).
    pub fn undo_window(self, duration: std::time::Duration) -> Self { unimplemented!() }

    /// Per-Field configuration. One call per diverging Field.
    pub fn field(self, field: R::Field, cfg: impl FnOnce(FieldBuilder<R>) -> FieldBuilder<R>) -> Self { unimplemented!() }

    /// Typed relation declaration. Target registration is boot-asserted (CAP-402).
    /// When `Target` binds to a DIFFERENT Provider, the relation is accepted but
    /// structurally verified only — target registered + field types compatible —
    /// never Snapshot-verified, and carries a visible UNVERIFIED marker in build
    /// output (CAP-405, advanced setups).
    pub fn belongs_to<Target: Resource>(self, via: R::Field) -> Self { unimplemented!() }
    pub fn has_many<Target: Resource>(self, target_fk: Target::Field) -> Self { unimplemented!() }

    /// Many-to-many over a detected Junction resource (CAP-404). Type-level on
    /// both the Junction and the Target — no instances passed. The two FK fields
    /// live ON THE JUNCTION: `via_self` must reference this Resource, `via_target`
    /// must reference `Target`; registration type-checks both and rejects swaps.
    /// Compiles to a Junction-filtered query on the target Replica; pick-one
    /// inputs gain multi-select behavior. Junction candidates come from Snapshot
    /// detection; non-pure junctions must use has_many instead (build guidance).
    pub fn many_to_many<J: Resource, Target: Resource>(self, via_self: J::Field, via_target: J::Field) -> Self { unimplemented!() }

    /// Per-View overrides. Forms accept layout composition (sections/columns);
    /// show takes a plain ordered projection.
    pub fn list(self, cfg: impl FnOnce(ListBuilder<R>) -> ListBuilder<R>) -> Self { unimplemented!() }
    pub fn show(self, projection: impl Into<Projection<R>>) -> Self { unimplemented!() }
    pub fn create(self, cfg: impl FnOnce(FormBuilder<R>) -> FormBuilder<R>) -> Self { unimplemented!() }
    pub fn edit(self, cfg: impl FnOnce(FormBuilder<R>) -> FormBuilder<R>) -> Self { unimplemented!() }

    /// Finalize into the registrable declaration consumed by the Registry (R4).
    pub fn build(self) -> ResourceDecl<R> { unimplemented!() }
}

pub struct FormBuilder<R: Resource> { _marker: std::marker::PhantomData<R> }

impl<R: Resource> FormBuilder<R> {
    /// Declaration-level layout primitive (Q10 ruling): named visual section.
    pub fn section(self, title: &'static str, cfg: impl FnOnce(FormBuilder<R>) -> FormBuilder<R>) -> Self { unimplemented!() }
    /// Declaration-level layout primitive: column count for the enclosed scope.
    pub fn columns(self, count: usize, cfg: impl FnOnce(FormBuilder<R>) -> FormBuilder<R>) -> Self { unimplemented!() }
    pub fn projection(self, fields: impl Into<Projection<R>>) -> Self { unimplemented!() }
}

pub struct FieldBuilder<R: Resource> { _marker: std::marker::PhantomData<R> }

impl<R: Resource> FieldBuilder<R> {
    pub fn label(self, text: &'static str) -> Self { unimplemented!() }
    /// Input widget override; default inferred from Snapshot type by the Scaffolder.
    pub fn widget(self, widget: Widget) -> Self { unimplemented!() }
    pub fn required(self, required: bool) -> Self { unimplemented!() }
    pub fn filterable(self) -> Self { unimplemented!() }
    pub fn sortable(self) -> Self { unimplemented!() }

    /// Declarative core rules (Q12 ruling): the closed vocabulary the framework
    /// understands — introspectable by diagnostics and emitted automatically by
    /// the Scaffolder wherever Snapshot truth implies them (bounds/length/pattern
    /// where source-declared; requiredness is NOT a FieldRule — it is carried by
    /// `required()`/payload shape, and the Scaffolder emits `required(...)` calls
    /// directly from nullability). Backend validation always overrides any client
    /// verdict (CAP-305).
    pub fn rule(self, rule: FieldRule) -> Self { unimplemented!() }

    /// Domain validation over the PARSED typed value (layer 2 of the form
    /// state machine; layer 1 parse is type-derived). The escape hatch after
    /// the declarative set.
    pub fn validate(self, rule: fn(&FieldValue) -> Result<(), String>) -> Self { unimplemented!() }

    /// Quick Edit (CAP-310): allow this simple-widget Field to be edited inline
    /// in List rows through the standard mutation lifecycle. Valid only for
    /// Switch/EnumSelect/Number/DecimalNumber/Text widgets — build error otherwise.
    pub fn quick_edit(self) -> Self { unimplemented!() }

    /// Escape hatch: custom input widget (CAP-903, P1 — signature reserved at v0.1).
    pub fn custom_widget(self, factory: CustomWidgetFactory<R>) -> Self { unimplemented!() }
}

/// Closed declarative rule set; membership is NOT here because enum types
/// carry it structurally (Scaffolder emits nothing for it — it cannot be violated).
pub enum FieldRule {
    Min(FieldValue),
    Max(FieldValue),
    MaxLength(usize),
    Pattern(&'static str),
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
/// File/Image variants require a configured Object Store; without one they
/// degrade to reference-as-text inputs (CAP-608).
pub enum Widget { Text, TextArea, Number, DecimalNumber, Switch, Date, DateTime, Time, Password, Email, Url, EnumSelect, RelationSelect, RichText, Markdown, FileUpload, ImageUpload }

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
