# Intent

<!-- decapod:declared-capabilities:start -->

## Declared Capability Surfaces

- `event-driven`
- `persistent-state`
- `public-api`

<!-- decapod:declared-capabilities:end -->
## Product Outcome

Amnion is the human-facing Rust terminal UI/UX for Pincher-managed governed
execution. It turns typed runtime state and events into a calm cockpit where a
human can understand custody, attention, progress, blockers, approvals, proof,
and handoff without reading a noisy raw log stream.

## Scope

| Area | Amnion owns | Authority |
| --- | --- | --- |
| Presentation | Intent browser, workspace/conversation views, status projection, and adjustable detail | Amnion |
| Human control | Explicit actions that request or acknowledge a governed transition | Decapod records result |
| Execution | Start/stop/view controls delegated to Pincher | Pincher |
| Governance | Session, task, workspace, approval, validation, proof, and promotion truth | Decapod |

## Non-goals

- Do not implement the agent loop or provider/tool orchestration.
- Do not write a parallel approval, todo, worktree, or proof database.
- Do not present a local optimistic action as an approved or promoted result.
- Do not require a specific backend transport until the Pincher host contract is
  versioned.

## Constraints

- Rust-first, terminal-native local host.
- Pincher owns execution; Decapod owns governance truth.
- Amnion's view state is a projection and cannot grant approval or promotion.

## Acceptance Criteria

- [ ] A human can move between governed intents and identify active custody at
      a glance.
- [ ] A detail view exposes agent/session, todo, worktree, touched files,
      validation, approvals, blockers, proofs, and handoff summary.
- [ ] Quiet mode shows meaningful activity; adjustable verbosity exposes event
      detail, raw logs, validation failures, and proof inspection.
- [ ] Every rendered status retains Pincher/Decapod identifiers and source
      timestamps needed to trace it to authority.
- [ ] Human actions are explicit, reversible where possible, and followed by a
      Decapod result before the projection changes to an authoritative state.
- [ ] The host builds/tests cleanly and `decapod validate` passes.

## Assumptions

- Pincher is the first execution producer and Amnion is its first host.
- The initial implementation can consume serialized Rust/event values locally;
  a long-lived transport is deferred until a concrete need is proven.
- Amnion is intentionally local-first and terminal-native.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `cbb46fea4ed69a2e244419b99c731f321e0d22223264c74afc034828705c58f2`
- Significant implementation surfaces: `.github/` (1 files), `README.md/` (1 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->
