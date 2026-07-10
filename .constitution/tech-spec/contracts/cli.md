# CLI Contract — `gpui-admin` (standalone binary, ADR-007)

Native formalism for the toolchain interface (B1/B3). Built with `clap 4.6` derive; every command supports `--help`. Machine-readable output uses `--format json` where noted.

## Command tree

```
gpui-admin
├── introspect                      # B1 — capture Schema Snapshot (the only online act)
│   ├── postgres
│   │   --url <CONNECTION_URL>      # or PG* env vars; never persisted
│   │   --schema <NAME=public>      # DB schema to read
│   │   --include <GLOB>... / --exclude <GLOB>...
│   │   --out <PATH=src/schema.rs>  # overwrite-always (machine-owned artifact)
│   └── openapi
│       --spec <PATH|URL>           # OpenAPI 3.0/3.1 document
│       --include / --exclude / --out (as above)
│
├── scaffold                        # B3 — one-time owned declarations
│   <RESOURCE>...                   # Snapshot type names; "--all" for every Resource
│   --snapshot <PATH=src/schema.rs>
│   --out-dir <PATH=src/admin/>     # one file per Resource; REFUSES to overwrite
│   --force                         # explicit overwrite of an owned file (prompts)
│
├── check                           # dry-run drift report: Snapshot vs declarations
│   --format <human|json>           # CI-friendly; exit code carries the verdict
│
└── showcase                        # contributor convenience (ADR-008)
    ├── up / down                   # compose lifecycle for backends/showcase
    └── seed                        # (re)apply data-models/showcase-backend.sql
```

## Exit codes

| Code | Meaning |
| :--- | :--- |
| 0 | Success / no drift. |
| 1 | Usage or configuration error (bad flags, unreadable paths). |
| 2 | Schema Source unreachable or unreadable (introspect only). |
| 3 | Drift detected (`check`) — stale declarations listed with file:line. |
| 4 | Refused overwrite of an Adopter-owned file (`scaffold` without `--force`). |

## Behavioral guarantees

- `introspect` is the **only** networked command (plus `showcase`, local containers); `scaffold` and `check` are fully offline (NFC-11).
- `introspect --out` overwrites without prompting: the Snapshot is machine-owned (ADR-001). `scaffold` never overwrites without `--force`: declarations are Adopter-owned.
- Both introspection frontends emit byte-identical Snapshot structure for equivalent sources (golden-tested against the shared fixture corpus, RSK-04).
- Connection credentials are read from flags/env at invocation and never written to any file, log, or Snapshot.
- All human output goes to stderr; `--format json` payloads go to stdout (pipe-safe).
