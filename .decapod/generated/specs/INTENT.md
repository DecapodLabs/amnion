# Intent

<!-- decapod:declared-capabilities:start -->

## Declared Capability Surfaces

- `event-driven`

<!-- decapod:declared-capabilities:end -->
## Product Outcome
- UI/UX layer. The soft place where intent becomes visible. The terminal cockpit, workspace, conversation surface, intent browser, status view, and human interaction layer.

## What This Project Is
amnion is a cli project built using Rust.
UI/UX layer. The soft place where intent becomes visible. The terminal cockpit, workspace, conversation surface, intent browser, status view, and human interaction layer.

Key operating facts:
- **Primary languages**: Rust
- **Detected surfaces**: cargo, rust

## Product View
```mermaid
flowchart LR
  U[Primary User] --> P[amnion]
  P --> O[User-visible Outcome]
  P --> G[Proof Gates]
  G --> E[Evidence Artifacts]
```

## Inferred Baseline
- Repository: amnion
- Product type: cli
- Primary languages: Rust
- Detected surfaces: cargo, rust

## Scope
| Area | In Scope | Proof Surface |
|---|---|---|
| Core workflow | Define a concrete user-visible workflow | Acceptance criteria + tests |
| Data contracts | Document canonical inputs/outputs | [INTERFACES.md](./INTERFACES.md) and schema checks |
| Delivery quality | Block promotion on broken proof surfaces | [VALIDATION.md](./VALIDATION.md) blocking gates |

## Non-Goals (Falsifiable)
| Non-goal | How to falsify |
|---|---|
| Feature creep beyond the primary outcome | Any PR adds capability not tied to outcome criteria |
| Shipping without evidence | Missing validation artifacts for promoted changes |
| Ambiguous ownership boundaries | Missing owner/system-of-record in interfaces |

## Constraints
- Technical: runtime, dependency, and topology boundaries are explicit.
- Operational: deployment, rollback, and incident ownership are defined.
- Security/compliance: sensitive data handling and authz are mandatory.

## Acceptance Criteria (must be objectively testable)
- [ ] Done means Amnion provides a soft, terminal-native Rust TUI where a human can move between governed intents, see each intent’s calm custody state at a glance, and drill into details only when needed: active agent/session, claimed todos, current worktree, touched files, Decapod validation state, approvals, blockers, proof artifacts, handoff summary, and recent meaningful activity. Amnion must not run the agent loop itself; it renders and controls Pincher-managed execution while treating Decapod as the source of governance truth. The default experience should be quiet and confidence-oriented, with minimal log noise, explicit human-attention states, and adjustable verbosity for deeper event streams, raw logs, validation failures, and proof inspection.
- [ ] Non-functional targets are met (latency, reliability, cost, etc.).
- [ ] Validation gates pass and artifacts are attached.
- [ ] `cargo test` passes for unit/integration coverage
- [ ] `cargo clippy -- -D warnings` passes with no denied lints
- [ ] `cargo fmt --check` passes on the repo

## Epistemic Custody Fields

### Active Assumptions
- [ ] List any assumptions made to proceed.
- [ ] Flag assumptions that require future verification.

### Confidence & Risk Level
- **Confidence**: Low/Medium/High (Rationale: )
- **Risk**: Low/Medium/High (Impact of wrong assumptions: )

### Measured vs Inferred Facts
| Fact | Source (Provenance) | Type (Measured/Inferred) |
|---|---|---|
| | | |

### Unresolved Contradictions
- [ ] List any evidence that conflicts with current assumptions or intent.

### Deferred Questions
- [ ] Questions to be answered later.

### Stop Conditions
- [ ] Explicit conditions under which the agent should stop and ask for help.

### Proof Required Before Completion
- [ ] Specific evidence needed to prove the outcome is met.

## Tradeoffs Register
| Decision | Benefit | Cost | Review Trigger |
|---|---|---|---|
| Simplicity vs extensibility | Faster iteration | Potential rework | Feature set expands |
| Strict gates vs dev speed | Higher confidence | More upfront discipline | Lead time regressions |

## First Implementation Slice
- [ ] Define the smallest user-visible workflow to ship first.
- [ ] Define required data/contracts for that workflow.
- [ ] Define what is intentionally postponed until v2.

## Open Questions (with decision deadlines)
| Question | Owner | Deadline | Decision |
|---|---|---|---|
| Which interfaces are versioned at launch? | TBD | YYYY-MM-DD | |
| Which non-functional target is hardest to hit? | TBD | YYYY-MM-DD | |
