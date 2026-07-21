# ADR-001: Establish Amnion as a projection-first terminal client

- Status: Accepted for the first vertical slice
- Date: 2026-07-21

## Context

Amnion is the human-facing terminal surface for governed agent work. Pincher
owns agent/provider/tool execution. Decapod owns custody and proof-backed
governance truth. A first implementation must make this boundary executable
without inventing a second authority or pretending that Pincher’s current open
event payloads are a stable host protocol.

## Decision

Amnion is a foreground Rust + Ratatui projection client. Its domain reducer
consumes a transport-neutral event envelope, produces read-only intent
projections, and preserves source custody, freshness, unknown input, and
contradictory evidence. The UI reads those projections and owns only ephemeral
selection, detail, and verbosity state.

Decapod remains authoritative for sessions, todos, workspaces, approvals,
validation, proofs, and promotion. Pincher remains authoritative for execution
and runtime activity. Amnion may display and later route human actions, but it
cannot optimistically change governance state.

The current adapter is fixture/replay data. It exercises real reducer semantics
before a premature transport is introduced. The adapter is isolated behind
`EventSourceAdapter`, so a future Pincher integration can provide subscription,
polling, or replay without changing the projection or presentation layers.

## Rationale

- Rust provides a small, foreground-native binary with explicit types and no
  required background service.
- Ratatui and Crossterm support a stable, local terminal surface without
  turning Amnion into a web, Electron, or persistent application platform.
- Projection semantics precede presentation breadth because truthful custody,
  authority, freshness, and attention are more important than widget count.
- Fixtures make unknown, stale, unavailable, malformed, and conflicting input
  testable while keeping the producer contract honestly provisional.

## Consequences

The first slice has no live Pincher connection, full conversation view, direct
Decapod enrichment, or mutating human controls. Those are separate issues with
their own contracts and proof expectations. In return, replay is deterministic,
duplicate events are idempotent, local `ready` cannot become authoritative
completion, and quiet mode can be tested without weakening safety signals.

