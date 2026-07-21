use crate::events::{SourceEvent, SourceFreshness, SourceType};
use serde_json::json;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureScenario {
    ActiveHealthy,
    AwaitingApproval,
    Blocked,
    ValidationFailure,
    ProofPending,
    AuthoritativelyReady,
    Failed,
    HandedOff,
    Stale,
    Unavailable,
    UnknownFutureEvent,
    ConflictingEvidence,
}

pub fn all_scenarios() -> [FixtureScenario; 12] {
    [
        FixtureScenario::ActiveHealthy,
        FixtureScenario::AwaitingApproval,
        FixtureScenario::Blocked,
        FixtureScenario::ValidationFailure,
        FixtureScenario::ProofPending,
        FixtureScenario::AuthoritativelyReady,
        FixtureScenario::Failed,
        FixtureScenario::HandedOff,
        FixtureScenario::Stale,
        FixtureScenario::Unavailable,
        FixtureScenario::UnknownFutureEvent,
        FixtureScenario::ConflictingEvidence,
    ]
}

pub fn title(scenario: FixtureScenario) -> &'static str {
    match scenario {
        FixtureScenario::ActiveHealthy => "Refactor workspace custody",
        FixtureScenario::AwaitingApproval => "Approve release boundary",
        FixtureScenario::Blocked => "Resolve missing workspace",
        FixtureScenario::ValidationFailure => "Repair failing validation",
        FixtureScenario::ProofPending => "Collect promotion proof",
        FixtureScenario::AuthoritativelyReady => "Review completed migration",
        FixtureScenario::Failed => "Recover failed handoff",
        FixtureScenario::HandedOff => "Review engineering handoff",
        FixtureScenario::Stale => "Reconnect stale projection",
        FixtureScenario::Unavailable => "Restore authority source",
        FixtureScenario::UnknownFutureEvent => "Inspect future event",
        FixtureScenario::ConflictingEvidence => "Reconcile evidence conflict",
    }
}

pub fn events_for(scenario: FixtureScenario) -> Vec<SourceEvent> {
    let intent = format!("fixture/{scenario:?}").to_lowercase();
    let title = title(scenario);
    let mut events = vec![
        base("001", "intent.created", &intent, SourceType::Fixture)
            .with_payload(json!({"title": title})),
        base("002", "work.started", &intent, SourceType::Pincher)
            .with_run("run-7f3e")
            .with_session("session-01J0-AMNION")
            .with_agent("agent-pincher-executor")
            .with_todo("todo-01J0-REFAC")
            .with_work_unit("workunit-01J0-REFAC")
            .with_payload(json!({
                "workspace": ".decapod/workspaces/agent-pincher-refac",
                "workspace_known": true,
                "touched_files": ["src/adapter.rs", "tests/projection.rs"]
            })),
    ];

    match scenario {
        FixtureScenario::ActiveHealthy => {
            events.push(base("003", "work.progress", &intent, SourceType::Pincher));
        }
        FixtureScenario::AwaitingApproval => {
            events.push(
                base("003", "approval.requested", &intent, SourceType::Pincher).with_payload(
                    json!({
                        "approval_id": "interlock-release-7",
                        "reason": "Promotion changes the protected branch"
                    }),
                ),
            );
        }
        FixtureScenario::Blocked => {
            events.push(
                base("003", "blocked", &intent, SourceType::Pincher).with_payload(json!({
                    "blocker_id": "custody-missing-2",
                    "cause": "workspace path was not returned by the source"
                })),
            );
        }
        FixtureScenario::ValidationFailure => {
            events.push(
                base("003", "validation.failed", &intent, SourceType::Decapod).with_payload(
                    json!({
                        "validation_id": "validation-2026-07-21",
                        "gate": "projection-contract",
                        "errors": ["authoritative proof artifact is missing"]
                    }),
                ),
            );
        }
        FixtureScenario::ProofPending => {
            events.push(
                base("003", "proof.pending", &intent, SourceType::Pincher).with_payload(
                    json!({"proof_id": "proof-7f3e", "criteria": "cargo test + decapod validate"}),
                ),
            );
        }
        FixtureScenario::AuthoritativelyReady => {
            events.push(
                base("003", "validation.passed", &intent, SourceType::Decapod)
                    .with_payload(json!({"validation_id": "validation-7f3e", "gate": "all"})),
            );
            events.push(
                base("004", "proof.verified", &intent, SourceType::Decapod)
                    .with_payload(json!({"proof_id": "proof-7f3e"})),
            );
            events.push(base("005", "ready", &intent, SourceType::Decapod));
        }
        FixtureScenario::Failed => {
            events.push(
                base("003", "failed", &intent, SourceType::Pincher).with_payload(json!({
                    "cause": "provider process exited before the handoff summary was written"
                })),
            );
        }
        FixtureScenario::HandedOff => {
            events.push(
                base("003", "handoff.requested", &intent, SourceType::Pincher)
                    .with_payload(json!({"handoff_id": "handoff-7f3e"})),
            );
            events.push(
                base("004", "handoff.accepted", &intent, SourceType::Decapod)
                    .with_payload(json!({"handoff_id": "handoff-7f3e"})),
            );
        }
        FixtureScenario::Stale => {
            events.push(
                base("003", "source.stale", &intent, SourceType::Pincher)
                    .with_freshness(SourceFreshness::Stale { age_seconds: 317 }),
            );
        }
        FixtureScenario::Unavailable => {
            events.push(
                base("003", "source.unavailable", &intent, SourceType::Decapod).with_freshness(
                    SourceFreshness::Unavailable {
                        reason: "Decapod query timed out".to_string(),
                    },
                ),
            );
        }
        FixtureScenario::UnknownFutureEvent => {
            events.push(
                base("003", "execution.phase.v3", &intent, SourceType::Pincher)
                    .with_payload(json!({"phase": "review", "confidence": 0.99})),
            );
        }
        FixtureScenario::ConflictingEvidence => {
            events.push(
                base("003", "validation.passed", &intent, SourceType::Decapod)
                    .with_payload(json!({"validation_id": "validation-old", "gate": "all"})),
            );
            events.push(
                base("004", "validation.failed", &intent, SourceType::Decapod).with_payload(
                    json!({"validation_id": "validation-new", "errors": ["replay mismatch"]}),
                ),
            );
        }
    }
    events
}

pub fn demo_events() -> Vec<SourceEvent> {
    all_scenarios().into_iter().flat_map(events_for).collect()
}

fn base(id: &str, event_type: &str, intent: &str, source_type: SourceType) -> SourceEvent {
    SourceEvent::new(
        format!("evt-{id}-{intent}"),
        event_type,
        format!("2026-07-21T00:00:{id}Z"),
        source_type,
        intent,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::{Attention, IntentStatus, Reducer};

    #[test]
    fn every_fixture_projects_without_panicking() {
        for scenario in all_scenarios() {
            let state = Reducer::new().reduce(events_for(scenario));
            assert_eq!(state.projections().len(), 1, "{scenario:?}");
        }
    }

    #[test]
    fn fixture_states_cover_attention_and_authority_boundaries() {
        let state = Reducer::new().reduce(demo_events());
        let projections = state.projections();
        assert!(
            projections
                .iter()
                .any(|p| p.status == IntentStatus::ReadyForReview)
        );
        assert!(
            projections
                .iter()
                .any(|p| p.attention == Attention::Approval)
        );
        assert!(
            projections
                .iter()
                .any(|p| p.attention == Attention::SourceConflict)
        );
    }
}
