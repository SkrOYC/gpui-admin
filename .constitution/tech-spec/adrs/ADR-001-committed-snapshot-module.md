# ADR-001: Schema Snapshot as a Committed Rust Module

**Status:** Accepted.

## Context

The derivation engine (B4) needs schema truth at build time. Three mechanisms exist: a proc-macro reading a schema file at expansion time, a `build.rs` generation step, or an offline CLI emitting a committed source module.

## Decision

The CLI emits a **committed Rust module** (`schema.rs` per the format in `data-models/schema-snapshot.rs`); the derivation macros operate purely on those tokens. Proc-macro filesystem reads are rejected (untracked I/O breaks cargo's incremental model and hermeticity); `build.rs` generation is rejected (invisible, non-reviewable, runs on every consumer machine).

This is the `sqlx`-offline / `prost` workflow: capture is explicit and online-once; the build is hermetic.

## Consequences

- Schema changes are reviewable diffs in version control (NFC-11/12 satisfied structurally); air-gapped builds work by construction.
- Cost: capture is a manual act after Backend migrations; staleness is caught by the drift build-failure, not by magic freshness.
- The Snapshot format becomes a public compatibility surface (Compatibility Policy §4).
