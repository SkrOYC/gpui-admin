# Out of Scope: Hosted Service / SaaS Offering

**Status:** Rejected (structural).
**Since:** v0.1.0.

## Context

A common commercial path for admin tooling is a hosted service: the vendor runs the admin, the customer connects their data.

## Decision

gpui-admin is a framework and toolchain the Adopter fully owns. There is no hosted component, no account, no vendor-run service in the product.

## Reasoning

Backend sovereignty is a product pillar: the Adopter must be able to build, audit, and run the result entirely under their own control, including air-gapped. A hosted offering would invert the trust relationship the product is built to guarantee, and would additionally require server-side state (e.g., stored workspace layouts) that the design explicitly refuses.

## Reopening Conditions

A future *separate* commercial product could exist alongside the framework, but never as a dependency of it. Not part of this constitution.
