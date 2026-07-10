# ADR-003: Provider Contract as `async fn` in Trait + Injected `HttpClient`

**Status:** Accepted.

## Context

R1's contract could return the substrate's `Task` type (coupling every Provider to the UI framework) or be plain async. Separately, Providers need HTTP; the substrate has a first-class abstraction (`gpui_http_client::HttpClient`, injected app-wide, with `FakeHttpClient` for tests — verified in source), but no published production implementation exists.

## Decision

- Provider methods are **`async fn` in the trait** (Rust 2024; Providers are always used generically through monomorphized bindings, so no dyn-dispatch cost or object-safety constraint applies).
- Providers receive an injected **`Arc<dyn HttpClient>`**; the Replica boundary wraps returned futures into substrate `Task`s via `cx.spawn`.
- We ship a thin **reqwest-backed `HttpClient` adapter** (`reqwest 0.13`) running on a dedicated tokio-runtime thread — the upstream pattern — inside `gpui-admin-core::provider`.

## Consequences

- Providers are UI-framework-free: testable as plain async with `FakeHttpClient`, implementable by Contributors who never touch the substrate (CAP-1003).
- The Conformance Suite runs Providers without any UI runtime.
- We own ~100 lines of adapter and its runtime-lifecycle care (shutdown ordering documented — upstream notes the tokio-shutdown hazard).
- Non-HTTP Providers (e.g., in-process test backends) simply ignore the injected client — the contract is transport-agnostic; HTTP is a convenience, not an assumption.
