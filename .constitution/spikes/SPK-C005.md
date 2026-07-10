# Spike Report: GA-C005 Windowed Scrolling at the 10M Envelope

## 1. Context & Objective
- **Triggering upstream file/section:** `.constitution/architecture/risks.md` RSK-03; `.constitution/prd/constraints.md` NFC-01; `.constitution/tech-spec/adrs/ADR-006-pagination-capability-split.md`
- **Target:** Sustained frame rate of the virtualized-table ↔ query-entry binding over a synthetic 10,000,000-row collection: continuous scroll, random scrollbar jumps, fast scrubbing (window-fetch coalescing), memoized-view rebuild cost. Go/no-go verdict on random-access windowing at envelope scale vs the sequential-frontier fallback.

## 2. Codebase Baseline
- **Current State:** [To be completed during execution — findings from the GA-C002 seam under instrumentation]
- **Discovered Constraints:** [To be completed during execution]

## 3. Options & Trade-offs
- [To be completed during execution — measured frame-time distributions per scenario; random-access vs frontier fallback with concrete numbers]

## 4. Execution Directives
- **Chosen Option:** [To be completed during execution]
- **Why it fits:** [To be completed during execution]
- **Downstream Backlog Impact:** [To be completed during execution — expected: confirms or amends GA-D002 windowing parameters and the deferred Epic H list-binding tickets]
