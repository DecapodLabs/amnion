# Amnion

Amnion is the human-facing terminal UI/UX for governed agent work: the quiet
place where Pincher execution becomes visible and Decapod evidence stays
legible. The primary unit is a governed intent, not a log stream.

The boundary is deliberate:

```text
Amnion   -> projects state, routes attention, exposes read-only controls
Pincher  -> runs agent/provider/tool execution and emits runtime events
Decapod  -> owns sessions, todos, workspaces, approvals, validation, proofs,
            and promotion truth
```

Amnion does not run an agent loop, call a model provider, execute tools, apply
patches, persist governance state, or locally grant approval/proof. A local
Pincher event such as `ready` remains an observation until both validation and
proof evidence are explicitly present from Decapod. Missing, stale, unavailable,
malformed, unknown, and contradictory source data remain visible as such.

## First experience

The current source is a deterministic fixture/replay adapter. It is a deliberate
stand-in while Pincher’s host-facing event contract is still provisional; it is
not a claim that Amnion has production Pincher transport integration.

This first slice is intentionally a foreground demonstration of the projection
boundary: the fixture source can be replaced by a live adapter without changing
the reducer or terminal presentation.

```bash
cargo run
```

The screen opens with twelve representative intents covering working, approval,
blocked, validation failure, proof pending, authoritative readiness, failed,
handed-off, stale, unavailable, unknown-future-event, and conflicting-evidence
states. The default view is quiet and shows meaningful activity and attention
only.

Controls:

- `↑` / `↓` or `k` / `j`: move between intents
- `Enter` or `d`: open or close progressively deeper detail
- `v`: cycle `Quiet` → `Summary` → `Detailed` → `Debug`
- `q` or `Esc`: quit

Verbosity changes presentation only. It cannot hide blockers, failures, stale
state, source conflicts, or missing authority. There are no mutating controls in
this slice.

## Projection contract

The reducer in `src/projection.rs` preserves intent, run, session, agent, todo,
work-unit, workspace, touched-file, approval, blocker, validation, proof,
handoff, source-event, timestamp, source-type, freshness, unknown-event, and
diagnostic data. It sorts replay by source timestamp and event ID, ignores
duplicate event IDs, and retains unsupported events for inspection.

The projection separates observed runtime activity from projected execution
state, authoritative Decapod evidence, human attention, and source freshness.
The fixture adapter implements the small `EventSourceAdapter` boundary so a
future Pincher adapter can replace it without changing the reducer or UI.

The current producer shape is informed by Pincher’s public Rust types, but the
event envelope is not declared as Pincher’s final public protocol. Pincher must
publish that contract, compatibility policy, and shared fixtures before a real
transport adapter is added.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
decapod validate
```

Architecture decisions are recorded in
[`docs/adr/001-projection-first.md`](docs/adr/001-projection-first.md).

## Current limitations

- The source is fixture/replay only; there is no Pincher process, socket, or
  network integration.
- Decapod enrichment and governed human-action routing are follow-on work.
- Detail views currently show compact evidence references rather than full
  transcripts, diffs, proof artifacts, or handoff documents.
- Terminal accessibility and narrow-width behavior need a dedicated contract.
