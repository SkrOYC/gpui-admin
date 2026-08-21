# Deep Link Grammar — Adopter-Declared Scheme (CAP-505 / CAP-208)

Normative format for the address form produced and consumed by the Routing airlock (R4). The scheme name itself is **declared by the Adopter application** (library principle); the framework registers nothing globally. Payloads are readable path + query parameters — never opaque blobs — so links stay reviewable in version control, chat, and diffs.

## Address forms

```
<scheme>://<resource-key>/record/<record-id>            # Show/Edit primary Panel
<scheme>://<resource-key>?<query-params>                # List primary Panel
```

## Query parameter serialization

Typed `Query<R>` serializes losslessly using snapshot-canonical field names (`FieldIdent::name()`):

| Parameter | Encoding | Example |
| :--- | :--- | :--- |
| `filter` | Repeatable. `<field>:<op>:<value>`; multi-values comma-separated (each value percent-encoded per the delimiter rule) | `filter=status:eq:active` · `filter=age:gte:21` |
| `sort` | Repeatable, order-significant. `<field>:<asc\|desc>` | `sort=name:asc` |
| `search` | Percent-encoded free text | `search=annual%20report` |

Delimiter rule: the delimiters themselves (`:`, `,`, `&`, `=`) are always literal structure; any occurrence of one of them *inside* a value (or inside a `<record-id>` path segment) is RFC 3986 percent-encoded (`%3A`, `%2C`, `%26`, `%3D`). List values are percent-encoded the same way — no backslash-style escaping exists in this grammar. Decoding is therefore unambiguous.

Percent-encoding per RFC 3986 applies to values; field names are already identifier-safe. Decimal values serialize exactly (never through binary floats).

## Behavioral guarantees

- A link carries **content only**: resource, record id, and/or query state. No Workspace layout, no panel arrangement, no session state (CAP-505 boundary; personal layouts live in R9).
- Unknown resource keys resolve to the honest "not found" panel — link rot is visible, never a crash.
- Unparseable query parts are dropped with a visible notice naming what was ignored; they never silently alter the query.
- Bookmarking a received link stores the parsed Saved Query locally (R9), not the raw string.
- OS-level handling of the declared scheme is P1 (CAP-506); at P0 links work via in-app entry and paste.
