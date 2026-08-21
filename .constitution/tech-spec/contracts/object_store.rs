//! NORMATIVE CONTRACT — ObjectStore port (R10 / CAP-608).
//!
//! Independent of the data Provider by design: binary objects are
//! infrastructure beside the Backend, never inside it. Records hold
//! references produced here; committed Records never carry dangling
//! references (object-first dispatch ordering, PB-2).
//!
//! Vocabulary follows `.constitution/prd/glossary.md` strictly.

use std::future::Future;
use std::sync::Arc;

/// Stable reference stored into Record Fields after successful upload.
/// `url` is populated when the implementation exposes one; `key` is always
/// the canonical storage address. Serialized form is what reference-as-text
/// degradation shows when no Object Store is configured.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ObjectRef {
    pub key: String,
    pub url: Option<String>,
}

/// Caller-supplied addressing hint (resource key + record id + field name);
/// implementations derive their final layout. Framework never assumes the
/// bucket/layout shape — that is implementation territory.
pub struct ObjectKeyHint<'a> {
    pub resource_key: &'a str,
    pub record_id: &'a str,
    pub field_name: &'a str,
}

/// The only error currency crossing the R10 boundary.
#[derive(Debug, thiserror::Error)]
pub enum ObjectError {
    #[error("no object store configured")]   Unconfigured,
    #[error("forbidden by the object store")] Forbidden,
    #[error("transport failure (retryable: {retryable})")]
    Transport { retryable: bool, #[source] source: Box<dyn std::error::Error + Send + Sync> },
    #[error("store failure")]
    Storage(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Pluggable binary-object port. Implemented first-party against the
/// dominant de-facto object-storage API (operator preference appendix,
/// prd/vision.md); Contributors may target anything.
///
/// Dispatch ordering contract (with R3): upload happens BEFORE the record
/// mutation is sent. On record rejection or Undo-Window revocation the
/// coordinator calls `delete` best-effort; cleanup failures are logged to
/// diagnostics (orphan report) and are NOT surfaced as mutation failures.
pub trait ObjectStore: Send + Sync + 'static {
    /// Starts an upload immediately; the returned handle streams progress
    /// events while the transfer runs and is awaited for the final reference.
    fn put(
        &self,
        hint: &ObjectKeyHint<'_>,
        bytes: Arc<[u8]>,
        content_type: &str,
    ) -> UploadHandle;

    /// Fetch bytes for rendering (image previews). Views cache via the
    /// Replica-adjacent object cache; never block interaction on this (NFC-04).
    fn get(&self, reference: &ObjectRef)
        -> impl Future<Output = Result<Arc<[u8]>, ObjectError>> + Send;

    /// Best-effort delete. Used ONLY for cleanup of uncommitted uploads;
    /// never as a data-mutation path. Failures are logged, not propagated.
    fn delete(&self, reference: &ObjectRef)
        -> impl Future<Output = Result<(), ObjectError>> + Send;
}

/// Eager handle: progress observable while in flight; completion awaited.
pub struct UploadHandle { /* opaque */ }
impl UploadHandle {
    pub async fn finish(self) -> Result<(ObjectRef, UploadStats), ObjectError> { unimplemented!() }
    pub fn observe(&self) -> UploadProgress { unimplemented!() } // event stream for R6 UX
}
pub struct UploadStats { pub bytes: u64, pub duration: std::time::Duration }
