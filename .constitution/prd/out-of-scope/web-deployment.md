# Out of Scope: Web / Browser Deployment

**Status:** Deferred (not a v1 target).
**Since:** v0.1.0.

## Context

The underlying UI substrate has an in-progress browser compilation target, so a web-deployed admin is technically imaginable.

## Decision

v1 targets native desktop platforms only (Linux and macOS at P0, Windows at P1). No web build is promised, tested, or claimed.

## Reasoning

The product identity is explicitly "professional desktop software, not a web page" — multi-panel docking, instant cold start, and OS-integrated behavior are the differentiators. Splitting attention across a young browser target would dilute the identity while it is being established, and would drag browser ceilings back into a product defined by escaping them.

## Reopening Conditions

Substrate browser support reaching maturity *and* a demonstrated adopter need for zero-install distribution. Revisit no earlier than the identity-defining P0 surface has shipped and stabilized.
