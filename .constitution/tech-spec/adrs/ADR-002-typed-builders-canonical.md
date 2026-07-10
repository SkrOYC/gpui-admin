# ADR-002: Typed Builders as the Canonical Authoring Surface

**Status:** Accepted (session interview 2026-07-09, three-persona analysis).

## Context

The Resource declaration is the API Adopters write daily and the Scaffolder's output format — the hardest surface to change after OSS release. Candidates: typed builder code, a function-like `resource! {}` DSL, an attribute macro on a marker struct.

Analysis through three personas converged: the idiomatic Rust developer gets ordinary code with full IDE support and declarations-as-values (loops, sharing, tests); the Rust newcomer is taught keystroke-by-keystroke by autocomplete and rustc diagnostics, where a macro block would leave them without completion and with expansion errors; the react-admin refugee gets the tactile anti-"magic string" experience that motivated their switch (`UserField::Status` cannot be typo'd). The substrate itself is builder-idiomatic throughout, making the framework feel native to its platform. The attribute-macro option is structurally awkward here because the Record struct is generated — config cannot hang on fields that live elsewhere.

## Decision

**Typed builders are THE authoring surface** (`contracts/resource_builder.rs`): the Scaffolder emits them, documentation teaches them, v1 ships only them. A sugar macro that expands to builders remains a possible post-v1 addition if adopters ask — zero commitment now. The `gpui-admin-macros` crate serves only derivation (Snapshot consumption, typed artifact emission), not authoring syntax.

## Consequences

- Full IDE ergonomics and rustdoc coverage for the entire authoring surface; no DSL reference docs to write or maintain; no macro-parser maintenance tax during the adoption-critical window.
- Slightly more verbose than a DSL; a few validations (e.g., duplicate projection entries) move from compile-time to the startup assertion sweep — accepted, still pre-Operator.
- If DSL sugar ships later, docs must keep anointing builders as primary to avoid a two-dialect ecosystem.
