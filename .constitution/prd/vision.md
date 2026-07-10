# Vision

## 0. Version

**v0.1.1** — see [`changelog.md`](./changelog.md).

## Executive Summary

**gpui-admin is a framework and companion toolchain for building native desktop administration applications over backends the builder does not necessarily control.**

Data-driven products need administration interfaces — the screens where staff browse, correct, and operate the business's records. Today those interfaces are either hand-built (weeks of repetitive effort) or produced by web-based admin frameworks that inherit the browser's ceilings: one view at a time, page-shaped navigation, runtime configuration errors discovered by users, and sluggishness on large collections.

**The identity of this product is the experience it produces: an admin that feels like professional desktop software, not a web page.** Multi-panel workspaces — split, tabbed, and floating — with live master-detail browsing, instant startup, and fluid interaction over collections of millions of records. The workspace an operator arranges today is the workspace that greets them tomorrow.

Two trust pillars support that identity:

1. **Build-time correctness.** Resource declarations are verified against a captured snapshot of the backend's schema before the application can be built. A field typo, a broken relation, or drift after a schema migration is a *build failure with a pointed message*, never a runtime surprise in front of an operator.
2. **Backend sovereignty.** The framework is correct against an arbitrary, uncooperative backend. Anything requiring backend cooperation — realtime feeds, cursor paging, conditional updates — is an opt-in capability, never load-bearing. The client is never the security boundary. The result is an admin the adopter fully owns: buildable offline, runnable without external services, dependent on no vendor.

**Target archetype:** a developer library/framework plus an offline code-generation toolchain, producing compiled, native desktop applications for Linux and macOS first. Developed and distributed as open source from the first release.

## Jobs to Be Done (JTBD)

1. **Stand up an admin in hours.** *When my product's data already lives in a backend, I want to generate a trustworthy administration interface from its schema in hours, not weeks, so my team can operate the business without me hand-building screens.*
2. **Make drift impossible to ship.** *When my backend schema evolves, I want the admin to refuse to build until its declarations agree with the new truth, so a schema migration can never silently ship a broken screen.*
3. **Operate data at desktop grade.** *When operators spend their day in the admin, I want multi-panel workflows, live master-detail, and fluid browsing of huge collections, so operating data feels like using a professional tool rather than clicking through pages.*
4. **Own the tool completely.** *When my organization's rules restrict tooling (compliance, air-gap, vendor policy), I want an admin I can build, audit, and run entirely under my own control, so adoption never requires trusting a third party.*
5. **Edit without fear.** *When I correct records as an operator, I want my changes visible immediately, revocable for a moment, and never silently lost on failure, so I can work fast and stay confident.*

## Appendix: Operator Preferences

*Non-binding implementation hints supplied by the operator. These do not govern requirements; they inform later stages.*

- **Language/substrate:** Rust; GPUI (Zed's UI framework) with the `gpui-component` widget library. Pin exact published versions; upgrade deliberately.
- **Schema Source frontends (v1):** relational-database catalog introspection (PostgreSQL) *and* API-description introspection (OpenAPI), both emitting one neutral snapshot format.
- **First verified Provider:** Supabase/PostgREST, kept strictly behind the neutral Provider contract with a public conformance suite.
- **Licensing/packaging:** `MIT` (updated 2026-07-10; supersedes the earlier `MIT OR Apache-2.0` hint); crates.io family `gpui-admin`, `gpui-admin-core`, `gpui-admin-macros`, `gpui-admin-cli`, `gpui-admin-ui`, `gpui-admin-conformance`, `gpui-admin-provider-supabase` (first four verified unclaimed 2026-07-09; remaining three checked at reservation time — the reservation ticket stops on any collision).
- **Pre-constitution design exploration:** an architecture draft (v4, session artifact) with a source-verified substrate record exists and should seed Stage 2; its technology decisions bind nothing at this layer.
