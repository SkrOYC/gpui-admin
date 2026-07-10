//! NORMATIVE CONTRACT — Provider surface (R1 / C1).
//!
//! This file is the source-of-truth signature set for `gpui-admin-core::provider`.
//! Implementation may add inherent items but MUST NOT weaken, rename, or widen
//! these public signatures without the paired Conformance Suite change
//! (Compatibility Policy §3, RSK-07) .
//!
//! Vocabulary follows `.constitution/prd/glossary.md` strictly.

use std::future::Future;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Resource identity (emitted by derivation, ADR-001; authored via builders, ADR-002)
// ---------------------------------------------------------------------------

/// Implemented by derivation output for every Resource. Never hand-written.
pub trait Resource: 'static + Sized {
    /// Stable resource key used by the Routing airlock and Providers (e.g. "users").
    const KEY: &'static str;
    /// Record shape captured in the Schema Snapshot.
    type Record: Clone + Send + Sync + serde::de::DeserializeOwned + serde::Serialize + 'static;
    /// Record identity. Hash/Eq/Clone required by the record store.
    type Id: Clone + std::hash::Hash + Eq + Send + Sync + std::fmt::Debug + 'static;
    /// Typed field identity: the per-Resource generated field enum.
    type Field: FieldIdent;
    /// Payload for `create` — excludes server-owned Fields (Snapshot-classified).
    type CreatePayload: Send + Sync + serde::Serialize + 'static;
}

/// Generated per-Resource field enum contract: dense, indexable, bitset-friendly.
pub trait FieldIdent: Copy + Eq + std::hash::Hash + std::fmt::Debug + 'static {
    /// Total number of Fields in the Resource.
    const COUNT: usize;
    /// Dense index in `0..COUNT` (bitset position).
    fn index(self) -> usize;
    /// Snapshot-canonical field name (serialization key).
    fn name(self) -> &'static str;
}

/// Set of Fields over a Resource's field enum (dirty sets, filter/sort sets,
/// invalidation diffs). Backed by a fixed-width mask; intersection is one AND.
pub struct FieldSet<F: FieldIdent> { /* opaque */ _marker: std::marker::PhantomData<F> }

/// Sparse update: dirty Fields + their new values, serializable per Provider dialect.
/// Produced by form dirty-tracking (ADR-009).
pub struct UpdatePatch<R: Resource> { /* opaque */ _marker: std::marker::PhantomData<R> }

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

/// Closed comparison operator set. Providers translate best-effort and MUST
/// reject unsupported operators loudly (`DataError::UnsupportedQuery`).
pub enum FilterOp { Eq, Neq, In, Gt, Gte, Lt, Lte, Contains, IsNull, IsNotNull }

/// One typed predicate over a Field.
pub struct FilterClause<R: Resource> {
    pub field: <R as Resource>::Field,
    pub op: FilterOp,
    pub value: FilterValue, // typed scalar/list; serialization dialect is Provider-owned
}

pub enum FilterValue { /* Bool, I64, F64, Text, Timestamp, Uuid, List(Vec<FilterValue>), ... */ }

pub struct Query<R: Resource> {
    /// AND-composed typed predicates.
    pub filters: Vec<FilterClause<R>>,
    /// Free-text Search (distinct from Filters; CAP-203). Provider-supported only.
    pub search: Option<String>,
    pub sort: Vec<(R::Field, SortDir)>,
    /// Per-Provider extension escape hatch (opaque to core; keys documented by the Provider).
    pub extension: Option<serde_json::Value>,
}

pub enum SortDir { Asc, Desc }

/// Requested slice, shaped by the declared pagination Capability (ADR-006).
pub enum Window {
    /// RandomAccess capability: absolute row range.
    Range { offset: usize, len: usize },
    /// SequentialCursor capability: continue from an opaque cursor (None = start).
    Cursor { after: Option<OpaqueCursor>, len: usize },
}

pub struct OpaqueCursor(pub Box<[u8]>);

pub struct GetListParams<R: Resource> {
    pub query: Query<R>,
    pub window: Window,
}

pub struct GetListResult<R: Resource> {
    pub items: Vec<R::Record>,
    /// None on Backends that cannot count (SequentialCursor without Counting).
    pub total: Option<usize>,
    /// Present iff the Provider declared SequentialCursor.
    pub next: Option<OpaqueCursor>,
}

// ---------------------------------------------------------------------------
// Error taxonomy — the only error currency crossing crate boundaries
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("not authenticated")]              Unauthenticated,
    #[error("forbidden")]                      Forbidden,
    #[error("not found")]                      NotFound,
    #[error("query not supported: {reason}")]  UnsupportedQuery { reason: String },
    #[error("transport failure (retryable: {retryable})")]
    Transport { retryable: bool, #[source] source: Box<dyn std::error::Error + Send + Sync> },
    #[error("provider failure")]
    Provider(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
pub enum MutationError<R: Resource> {
    #[error(transparent)]
    Data(#[from] DataError),
    /// Backend validation — routed onto the exact Fields (CAP-305).
    #[error("validation failed")]
    Validation(Vec<(R::Field, String)>),
    /// ConditionalUpdate capability precondition failed (ADR-009).
    #[error("conflict: record changed on the Backend")]
    Conflict,
}

// ---------------------------------------------------------------------------
// The Provider contract (ADR-003: async fn in trait; used generically, never dyn)
// ---------------------------------------------------------------------------

/// Sole communication path to a Backend (CAP-601). Constructed with an injected
/// `Arc<dyn gpui_http_client::HttpClient>` where transport is HTTP; the contract
/// itself is transport-agnostic.
pub trait DataProvider: Send + Sync + 'static {
    /// Static declaration of Backend cooperation (CAP-602). Absence of a
    /// capability MUST degrade richness only, never correctness.
    fn capabilities(&self) -> Capabilities;

    fn get_list<R: Resource>(
        &self, params: GetListParams<R>,
    ) -> impl Future<Output = Result<GetListResult<R>, DataError>> + Send;

    fn get_one<R: Resource>(
        &self, id: R::Id,
    ) -> impl Future<Output = Result<R::Record, DataError>> + Send;

    fn get_many<R: Resource>(
        &self, ids: &[R::Id],
    ) -> impl Future<Output = Result<Vec<R::Record>, DataError>> + Send;

    fn create<R: Resource>(
        &self, payload: &R::CreatePayload,
    ) -> impl Future<Output = Result<R::Record, MutationError<R>>> + Send;

    /// Sparse patch + previous baseline (ADR-009). PATCH dialects send `patch`;
    /// PUT dialects reconstruct `previous + patch`; ConditionalUpdate derives
    /// its precondition from `previous`.
    fn update<R: Resource>(
        &self, id: R::Id, patch: &UpdatePatch<R>, previous: &R::Record,
    ) -> impl Future<Output = Result<R::Record, MutationError<R>>> + Send;

    fn delete<R: Resource>(
        &self, id: R::Id, previous: &R::Record,
    ) -> impl Future<Output = Result<(), MutationError<R>>> + Send;
}

#[derive(Clone, Copy, Debug)]
pub struct Capabilities {
    pub pagination: PaginationMode,     // ADR-006
    pub counting: bool,                 // total available?
    pub search: bool,                   // CAP-203
    pub conditional_update: bool,       // CAP-307 / ADR-009
    pub change_feed: bool,              // gates SyncProvider paths (CAP-605)
}

#[derive(Clone, Copy, Debug)]
pub enum PaginationMode { RandomAccess, SequentialCursor }

// ---------------------------------------------------------------------------
// Optional capability sub-traits
// ---------------------------------------------------------------------------

/// Implemented only when `capabilities().change_feed` (P1: CAP-605).
pub trait SyncProvider: DataProvider {
    /// Subscribe to a Resource's change feed. Delivery: at-least-once, unordered.
    /// Reconnect contract: any interruption ⇒ core marks all warm queries stale.
    fn subscribe<R: Resource>(
        &self, sink: FeedSink<R>,
    ) -> impl Future<Output = Result<FeedSubscription, DataError>> + Send;
}

pub enum FeedEvent<R: Resource> {
    Upserted(R::Record),
    Deleted(R::Id),
}
pub struct FeedSink<R: Resource> { /* opaque sender */ _marker: std::marker::PhantomData<R> }
pub struct FeedSubscription { /* opaque; Drop = unsubscribe */ }

// ---------------------------------------------------------------------------
// Authentication surface (R7; none-mode is P0 — CAP-701)
// ---------------------------------------------------------------------------

pub trait AuthProvider: Send + Sync + 'static {
    fn check_session(&self) -> impl Future<Output = Result<Identity, DataError>> + Send;
    fn login(&self, credentials: Credentials) -> impl Future<Output = Result<Identity, DataError>> + Send;
    fn logout(&self) -> impl Future<Output = Result<(), DataError>> + Send;
    /// Advisory permission hints (CAP-703). NEVER authorization.
    fn permission_hints(&self) -> impl Future<Output = Result<PermissionHints, DataError>> + Send;
}

/// Ships in core: every method succeeds with an anonymous Identity.
pub struct NoAuth;

pub struct Identity { pub display_name: Option<String>, /* opaque claims */ }
pub struct Credentials { /* provider-defined; opaque to core */ }
/// Structurally advisory: consumable only by affordance APIs, not by mutation paths.
pub struct PermissionHints { /* opaque */ }
