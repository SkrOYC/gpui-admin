# Out of Scope: Client-Side Security Enforcement

**Status:** Rejected (structural, permanent).
**Since:** v0.1.0.

## Context

Admin frameworks commonly expose permission configuration that *looks* like access control: hiding buttons, disabling forms, gating routes.

## Decision

The client is never a security boundary. gpui-admin will not offer, document, or imply any client-side enforcement mechanism. Permission-aware UI (CAP-703) is an explicitly *advisory* convenience, structurally distinguished from authorization so it cannot be mistaken for it.

## Reasoning

Any client-enforced rule is trivially bypassed by anyone who can speak to the Backend directly. Pretending otherwise is the most dangerous kind of feature: one that manufactures false confidence. The Backend Owner enforces authorization; the product's job is to *surface* denials well (preserve input, correct affordances), never to prevent them cosmetically and call it security.

## Reopening Conditions

None. This is a permanent structural position of the product.
