# Out of Scope: Full Internationalization

**Status:** Deferred.
**Since:** v0.1.0.

## Context

Comparable admin frameworks offer pluggable translation layers covering every string, locale-aware formats, and right-to-left layouts.

## Decision

v1 ships with customizable **labels** (every Resource, Field, View, and action name is Adopter-overridable — this is already part of the declaration model, CAP-103/903) but no general translation framework, locale switching, or RTL support.

## Reasoning

Label customization covers the dominant real need (domain wording) at near-zero cost because declarations already own naming. A full i18n layer is a large, cross-cutting surface that would tax every capability in the critical path while serving a minority of early adopters. Deliberate deferral beats a half-baked translation story.

## Reopening Conditions

Post-v1, on demonstrated adopter demand — as a coherent pass over the whole framework surface, not incrementally.
