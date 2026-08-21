# ADR-008: Repo-Contained Compose Stack as the Showcase Backend

**Status:** Accepted.

## Context

CAP-604/1001 require a real, publicly reachable Backend class for the verified Provider and the showcase. Options: depend on a hosted third-party instance (fragile, credential-bound, violates reproducibility) or ship the environment.

## Decision

`backends/showcase/compose.yaml`: **PostgreSQL 17 + PostgREST 13 + MinIO (S3-compatible object store)**, seeded by `data-models/showcase-backend.sql`. This is simultaneously: the introspection target for the catalog frontend, a realistic OpenAPI source (PostgREST serves a generated description) for the second frontend, the live Backend for the Supabase/PostgREST Provider, the Object Store behind file/image fields (CAP-608), and the Conformance Suite's integration target in CI.

## Consequences

- One `docker compose up` gives any contributor the full E2E environment; CI runs conformance against it on the Linux lane.
- The demo exercises the *same* stack class as the first-party Provider's production target (PostgREST is Supabase's data API), so showcase and verification never diverge; MinIO keeps the Object Store story reproducible for the same reason.
- Container tooling becomes a contributor prerequisite (documented); the showcase app itself still builds and runs against a mock Provider without it.

> **Amendment (2026-08-21 realign):** MinIO added per CAP-608 (Object Store P0 via S3-compatible first-party implementation).
