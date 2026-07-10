//! Core engine for gpui-admin: the Provider Gateway (R1), the Replica record
//! store (R2), and the Mutation lifecycle (R3).
//!
//! This crate depends on `gpui` but never on `gpui-component`; the widget layer
//! lives in `gpui-admin-ui`. Implementation lands in Epics B-D.
#![deny(missing_docs)]
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
