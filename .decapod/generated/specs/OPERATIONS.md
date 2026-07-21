# Operations

<!-- decapod:capability-overlay:persistent-state:start -->

## Persistent State Operations Overlay

### Backup & Recovery
- Backup scope, schedule, retention, and restore evidence MUST be selected for the project
- Recovery point objectives MUST be explicit project decisions, not assumed values
- Recovery time objectives MUST be explicit project decisions, not assumed values
- Restore verification cadence MUST be recorded with the operational proof plan

### Migration Operations
- All schema changes via migration files
- Migration rollback procedures documented
- Zero-downtime migration strategy for production
- Migration health checks and rollback triggers
<!-- decapod:capability-overlay:persistent-state:end -->

## Host lifecycle

1. Start in a local foreground session.
2. Discover the Pincher/Decapod run list and establish a read projection.
3. Subscribe or poll for typed state/events with bounded refresh work.
4. Render quiet meaningful activity by default.
5. Route explicit controls to Pincher and wait for authoritative results.
6. Surface approvals, blockers, validation, proofs, and handoff evidence.

## Service Level Objectives

The TUI should remain responsive while Pincher runs; exact latency targets are
deferred until the first real adapter and view workload exist.

## Monitoring

Monitor source freshness, event lag, authority/query errors, refresh failures,
and unresolved attention states.

## Incident Response

Show unavailable/stale state, preserve source identifiers, stop unsafe controls,
and route the issue to Pincher/Decapod ownership.

## Recovery

If Pincher or Decapod is unavailable, retain the last source timestamp and show
an unavailable/stale state. Do not fabricate progress or approval. Reconnect
using custody identifiers and refresh from authority before resuming controls.

## Observability

The default view is concise. Detail mode may show event ids, source, timestamps,
validation errors, proof references, and raw logs subject to redaction. Amnion
does not own Pincher's execution logs or Decapod's audit records.

## Operational ownership

- Amnion: TUI responsiveness, refresh lifecycle, rendering failures, and
  readable human attention states.
- Pincher: loop execution, retries, cancellation, and event production.
- Decapod: session custody, approvals, validation, proofs, and promotion.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `cbb46fea4ed69a2e244419b99c731f321e0d22223264c74afc034828705c58f2`
- Significant implementation surfaces: `.github/` (1 files), `README.md/` (1 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->
