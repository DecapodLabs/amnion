# Amnion

Amnion is the human-facing terminal UI/UX for governed agent work. It is the
soft place where Pincher's execution state becomes visible: the cockpit,
workspace view, conversation surface, intent browser, status view, and human
attention flow.

Amnion renders and controls Pincher-managed execution. It does not run the
agent loop, become a second governance store, or infer approval from a local
UI action. Decapod remains the source of truth for sessions, todos, workspaces,
approvals, validation, proofs, and promotion.

## Boundary with Pincher

| Concern | Owner |
| --- | --- |
| Context preparation, provider turns, tool/patch proposals, retries, and loop lifecycle | [Pincher](https://github.com/DecapodLabs/pincher) |
| Durable custody, approvals, validation, proof, and promotion gates | Decapod |
| Intent list, calm status projection, conversation/workspace views, and attention routing | Amnion |

Amnion consumes Pincher's typed state and event stream, preserving run,
session, task, work-unit, workspace, approval, blocker, and proof references.
The UI is a projection: a displayed `ready` state is not authoritative until
the corresponding Decapod evidence says so.

## First experience

The default TUI should let a human:

- move between governed intents and see the current custody state at a glance;
- inspect the active agent/session, claimed todo, worktree, touched files,
  approvals, blockers, validation state, proof artifacts, and handoff summary;
- expand from quiet meaningful activity into detailed events, raw logs,
  validation failures, and proof inspection only when needed;
- take an explicitly labeled human action when Decapod records an approval
  decision or a blocked handoff.

## Development boundary

Amnion owns presentation and interaction policy. Pincher owns execution
semantics. Changes that cross that boundary require a versioned interface,
named producer/consumer ownership, and representative event/projection proof.
