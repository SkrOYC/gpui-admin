//! First-party Supabase / PostgREST Provider for gpui-admin. Consumes only
//! `gpui-admin-core`'s public contracts, keeping it deliberately unprivileged
//! (CAP-603/604). Implementation lands in Epic E.
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
