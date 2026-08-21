# Non-Functional Constraints

All constraints are measurable acceptance gates, not aspirations. "Reference hardware" = a mid-range developer laptop of the release year (defined precisely per release in test documentation).

## Performance (experience-defining — this product is UX-led)

| ID | Constraint | Measure |
| :--- | :--- | :--- |
| NFC-01 | List scrolling sustains the display's refresh rate over windowed collections of at least 10,000,000 Records. | No sustained frame drops below 60 fps during continuous scroll on reference hardware; unloaded rows show placeholders, never block. |
| NFC-02 | Cold start to an interactive, restored Workspace. | < 500 ms on reference hardware, excluding first Backend response. |
| NFC-03 | A Pending Change is visible in every affected open Panel. | Within one frame of the Operator's action (~16 ms); Backend latency never delays the optimistic presentation. |
| NFC-04 | Interaction feedback (click, focus, panel drag, tab switch). | < 100 ms perceived response; no interaction blocks on network. |
| NFC-05 | Warm memory envelope during active administration at scale. | **Scale:** resident set with one 10,000,000-Record Resource under continuous browsing on reference hardware. **Meter:** instrumented run, methodology fixed by the post-spike measurement ticket. **Goal:** ≤ 1 GB RSS (initial target; recalibrated by measurement before release hardening). **Fail:** > 2 GB sustained, or unbounded growth over an hour-long session. |

## Build & Iteration (the Adopter's experience)

| ID | Constraint | Measure |
| :--- | :--- | :--- |
| NFC-10 | Editing one Resource declaration to a rebuilt, running application. | < 30 s incremental on reference hardware. |
| NFC-11 | Snapshot capture and Scaffold generation run with no network access at build time. | A fully air-gapped machine can build the application from a committed repository. |
| NFC-12 | Every Drift build failure names the exact stale declaration and its file location. | Zero "generic mismatch" failures accepted. |

## Reliability & Data Safety

| ID | Constraint | Measure |
| :--- | :--- | :--- |
| NFC-20 | One malformed Record never disables a View. | Fault injection on any single Record leaves all sibling Records browsable and editable. |
| NFC-21 | A rejected mutation never loses Operator input. | The typed content remains recoverable (retry/discard) in 100% of rejection paths, including after the originating form closed. |
| NFC-22 | Application relaunch restores the prior Workspace. | Layout, panel arrangement, and open Views restored in 100% of clean and crash exits (last persisted state). |
| NFC-23 | After Backend confirmation or reconnection, all open Panels converge to Backend truth without manual refresh. | Transient divergence permitted; permanent divergence is a defect. |

## Security & Privacy

| ID | Constraint | Measure |
| :--- | :--- | :--- |
| NFC-30 | The client is never the enforcement boundary. | Every mutation path remains correct (fails safely, surfaces the denial) when the Backend rejects it; UI affordances are demonstrably advisory. |
| NFC-31 | No telemetry, phone-home, or external service contact unless the Adopter explicitly configures it. | Network capture on a default build shows connections only to the configured Backend and configured Object Store. Distribution, update checking, and error reporting are wholly the Adopter application's concerns; the framework's only extensibility seam for exporters is the framework's standard structured-logging subscriber hook. |
| NFC-32 | The framework never persists credentials in plaintext. | Secrets at rest use the platform's secure storage or are absent. |

## Operability & Distribution

| ID | Constraint | Measure |
| :--- | :--- | :--- |
| NFC-40 | Platform coverage: Linux and macOS at P0; Windows at P1. | P0 platforms are continuously verified; unverified platforms are never publicly claimed. |
| NFC-41 | A built admin application is a self-contained distributable. | Runs on a clean machine of the target platform with no runtime installation steps beyond the artifact itself. |
| NFC-42 | Scale envelope (design assumption, stated honestly): up to ~200 Resources per application; Record counts unbounded via windowed browsing; tens of concurrent Operators per Backend (low-contention assumption). | Envelope documented publicly; behavior beyond it is best-effort, not silently broken. |

## Accessibility & Input

| ID | Constraint | Measure |
| :--- | :--- | :--- |
| NFC-50 | Core Operator flows (navigate, browse, edit, save, undo) are fully keyboard-operable. | Each P0 capability's happy path completable without a pointing device (target: P0 for focus/edit flows, P1 for full panel management). |
