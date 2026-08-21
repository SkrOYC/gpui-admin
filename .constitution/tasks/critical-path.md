# Critical Path & Build Order

## 0. Version

**v0.3.0** — see [`changelog.md`](./changelog.md).

## Active Backlog Summary

- **Total Active Story Points:** **121** (Epic B: 22 · C: 21 · D: 26 · E: 22 · F: 30 — 32 tickets, 2 of them Spikes). Epic A (Foundation & OSS Bootstrap — 9 pts, 5 tickets) completed 2026-07-10 and archived to `completed/`. (v0.2.0 realign deltas: +GA-B000 substrate re-pin; GA-B003/B005 scope growth for bulk/permission vocabulary; GA-E001 format-2 emitter; GA-F001 full builder surface — see `../reports/2026-08-21-interview-realign.md`.)
- **Execution discipline:** one branch per epic (`type/description`), whole-epic squash PR into protected `master`, epics executed serially A → F (interview decision).
- **Critical Path (dependency spine gating Phase 1 completion; the GA-A001 foundation it builds on is complete):**
  1. GA-B000 (substrate re-pin) → GA-B001 → GA-B002 → GA-B003 (contract core incl. bulk + permission vocabulary)
  2. GA-B005 → GA-B006 (conformance + mock)
  3. GA-C001 (tracer resource + minimal replica)
  4. GA-C002 (windowed seam) → GA-C005 (scroll spike — RSK-03 gate)
  5. GA-C003 (form/mutation/undo seam)
  6. GA-D001 → GA-D002 → GA-D003 (store, query layer, overlay lifecycle)
  7. GA-D004 → GA-D005 / GA-D006 → GA-D007 (invalidation, freshness, feed → interleaving suite)
  8. GA-E001 (format-2 emitter) → GA-E004 / GA-E005 → GA-E006 (frontends → equivalence)
  9. GA-F001 (full builder surface) → GA-F002 (derivation — RSK-05 gate via GA-F006) → GA-F003 → GA-F004 / GA-F005 / GA-F007

## Build Order Diagram

```mermaid
flowchart LR
    %% Epic A (Foundation & OSS Bootstrap) completed 2026-07-10 — see completed/.
    subgraph B[Epic B — Contract & Conformance]
        B000[GA-B000 re-pin] --> B001[GA-B001] --> B002[GA-B002] --> B003[GA-B003] --> B005[GA-B005] --> B006[GA-B006]
        B004[GA-B004]
    end
    subgraph C[Epic C — Tracer Bullet]
        C001[GA-C001] --> C002[GA-C002] --> C004[GA-C004]
        C001 --> C003[GA-C003]
        C002 --> C005[GA-C005 spike]
    end
    subgraph D[Epic D — Replica & Mutation Core]
        D001[GA-D001] --> D002[GA-D002] --> D003[GA-D003] --> D007[GA-D007]
        D002 --> D004[GA-D004] --> D005[GA-D005] --> D007
        D004 --> D006[GA-D006] --> D007
    end
    subgraph E[Epic E — Toolchain MVP]
        E001[GA-E001] --> E004[GA-E004]
        E002[GA-E002] --> E003[GA-E003] --> E004
        E001 --> E005[GA-E005]
        E004 --> E006[GA-E006]
        E005 --> E006
    end
    subgraph F[Epic F — Derivation & Authoring]
        F001[GA-F001] --> F002[GA-F002] --> F003[GA-F003] --> F005[GA-F005]
        F001 --> F004[GA-F004]
        F002 --> F006[GA-F006 spike]
        F002 --> F007[GA-F007]
    end
    B006 --> C001
    C001 --> D001
    D003 --> F001
    E001 --> F002
    E006 --> F004
```

## Phasing Strategy

**Phase 1 (this backlog) delivers:** a public, CI-gated, name-reserved OSS workspace; the full Provider contract with executable conformance and a fully capable mock; the tracer-bullet vertical proving both substrate seams (windowed table, dock persistence) plus the 10M-scroll feasibility verdict; the complete Replica/Mutation state machines with the adversarial interleaving suite; both introspection frontends with byte-equivalence guarantees; and the derivation pipeline whose exit criterion is *deleting the tracer's hand-written code and regenerating it with identical behavior*, with drift-as-build-failure, the Scaffolder, `check`, and the build-budget verdict.

**Deferred (outlined; each gets its own Stage-4 evolution pass informed by Phase 1 reality — no tickets yet by design; scopes below reflect the 2026-08-21 realign rulings):**

- **Epic G — Shell & Dock full:** multi-Workspace windows (one OS window each), full Registry warmth, Follow linkage, edit focus-not-duplicate, deep-link entry under Adopter-declared schemes incl. typed Query payloads (CAP-505/208), placeholder-panel restoration, feedback surface, declaration-driven sidebar + breadcrumbs (CAP-507), command palette (CAP-508), OS-following theming (CAP-509), prev/next navigation (CAP-209).
- **Epic H — Views & Forms full:** all four Views per Resource over the input catalog (CAP-312), type-faithful temporal rendering (CAP-211), presentation-edge formatting policy (CAP-210), filters/sort/search UI incl. labeled loaded-window fallback (CAP-203), column masks + sort overrides (CAP-207), saved-query bookmark UI (CAP-208), Draft persistence (CAP-309), destructive-action confirmation (CAP-308), provisional band rendering, server-validation routing to fields, mid-edit drift banner, degraded-connectivity UX, Quick Edit on simple widgets (CAP-310).
- **Epic I — Relations:** relation airlock edges honoring Provider Bindings, cross-binding structurally-verified-only relations with UNVERIFIED markers (CAP-405), belongs-to inputs, has-many lists as target-replica queries, many-to-many over junctions incl. multi-select pickers (CAP-404), tree hierarchy cues (CAP-212), reparenting invalidation E2E.
- **Epic J — Verified Provider, Object Store & E2E:** PostgREST/Supabase Provider through conformance against the compose fixture (both dialect behaviors, bulk semantics), first-party S3-compatible ObjectStore implementation + MinIO compose wiring through the upload lifecycle (CAP-608), realtime feed at P1 boundary, connectivity state machine wiring per binding.
- **Epic K — Showcase & Launch:** showcase app composing everything incl. multi-backend demo binding (CAP-607), onboarding docs (30-minute path, CAP-1002), diagnostics panel, release automation, first tagged release; NFC-05 memory measurement ticket executes here (post-SPK-C005 methodology).

P1/P2 product scope (auth session kit CAP-702, permission vocabulary affordances CAP-703, bulk actions UI CAP-901, custom actions, widget overrides, presence, Windows platform, grid-editing spike CAP-311) remains governed by `prd/capabilities.md` and enters planning only after the deferred P0 epics.
