# Interfaces

<!-- decapod:capability-overlay:public-api:start -->

## Public API Capability Overlay

### API Contract Requirements
- All public endpoints MUST define explicit request/response schemas
- Versioning strategy MUST be documented (URL path or header-based)
- All public endpoints MUST implement idempotency for mutating operations
- Rate limiting and pagination MUST be implemented for list endpoints

### Compatibility Guarantees
- Backward-compatible changes ONLY within a version
- Breaking changes require new version (v1, v2, etc.)
- Deprecation and removal policy MUST be selected for this project and proven against its consumers

### Security Requirements
- All public endpoints MUST implement authentication
- Abuse-control enforcement point MUST be a documented project decision
- Input validation MUST reject malformed requests with typed errors
<!-- decapod:capability-overlay:public-api:end -->

## Inbound Contracts

Pincher provides typed runtime state/events and custody identifiers. Decapod
provides authoritative approval, validation, and proof results.

## Outbound Dependencies

Human controls route to Pincher/Decapod through the governed integration; no
parallel store or provider adapter is owned by Amnion.

## Pincher Host Contract

Pincher produces typed runtime state and events. Amnion consumes them as a
projection and preserves the identifiers needed to query authoritative
Decapod state.

| Contract | Producer | Consumer | Authority |
| --- | --- | --- | --- |
| Agent/run state | Pincher | Amnion view model | Pincher for runtime |
| `Event` stream | Pincher | Amnion activity/detail views | Pincher event identity |
| Session/task/workspace/work-unit refs | Pincher | Amnion | Decapod records |
| Approval/validation/proof result | Decapod via Pincher | Amnion | Decapod |

Every event projection retains event id, timestamp, event type, source, and
optional session/task/work-unit ids. Unknown event types remain visible as
safe generic activity rather than being discarded or reinterpreted.

## Data Ownership

Amnion owns projections and local view state. Pincher owns runtime state/events;
Decapod owns durable custody and proof.

## Human actions

Human actions are commands to inspect, start/stop, request attention, or route
an approval decision through the governed integration. Amnion displays a
pending state until Pincher/Decapod returns a typed result. UI state alone can
never grant approval, mark proof complete, or promote a run.

## Failure Semantics

| Failure | Projection | Action |
| --- | --- | --- |
| Event/provider delay | stale/pending indicator with last update | wait or inspect |
| Decapod interlock | blocked + required approval reference | route human attention |
| Validation/proof failure | failed-with-cause + evidence link | inspect/handoff |
| Lost/invalid custody reference | unavailable + explicit source error | do not infer state |

## Compatibility

The initial consumer can use local typed Rust values or serialized events. A
transport contract is deferred; when introduced it must be versioned and name
producer, consumer, lifecycle, correlation/idempotency fields, error mapping,
and migration evidence.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `cbb46fea4ed69a2e244419b99c731f321e0d22223264c74afc034828705c58f2`
- Significant implementation surfaces: `.github/` (1 files), `README.md/` (1 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->
