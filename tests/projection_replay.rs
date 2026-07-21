use amnion::fixtures::{self, FixtureScenario};
use amnion::projection::{AuthorityState, IntentStatus, Reducer};

#[test]
fn the_fixture_adapter_exercises_each_contract_state() {
    for scenario in fixtures::all_scenarios() {
        let projection = Reducer::new()
            .reduce(fixtures::events_for(scenario))
            .projections()
            .pop()
            .expect("fixture projection");
        assert!(!projection.intent_id.is_empty());
        assert!(!projection.events.is_empty(), "{scenario:?}");
    }
}

#[test]
fn authoritative_ready_requires_decappod_validation_and_proof() {
    let ready = Reducer::new()
        .reduce(fixtures::events_for(FixtureScenario::AuthoritativelyReady))
        .projections()
        .pop()
        .expect("ready projection");
    assert_eq!(ready.status, IntentStatus::ReadyForReview);
    assert_eq!(ready.authority, AuthorityState::Confirmed);
    assert!(ready.has_authoritative_ready_evidence());
}
