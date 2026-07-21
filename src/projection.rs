use crate::events::{SourceEvent, SourceFreshness, SourceType, timestamp_is_plausible};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    #[default]
    Absent,
    Pending,
    Passed,
    Failed,
    Unavailable,
    Conflicting,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub state: EvidenceState,
    pub reference: Option<String>,
    pub details: Vec<String>,
    pub source: Option<SourceType>,
    pub observed_at: Option<String>,
}

impl Default for Evidence {
    fn default() -> Self {
        Self {
            state: EvidenceState::Absent,
            reference: None,
            details: Vec::new(),
            source: None,
            observed_at: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityState {
    NotProvided,
    ObservedOnly,
    Confirmed,
    Unavailable,
    Conflicting,
}

impl AuthorityState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NotProvided => "No authority evidence",
            Self::ObservedOnly => "Observed only",
            Self::Confirmed => "Decapod evidence present",
            Self::Unavailable => "Authority unavailable",
            Self::Conflicting => "Conflicting evidence",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentStatus {
    Unknown,
    Working,
    WaitingApproval,
    Blocked,
    Failed,
    ProofPending,
    WaitingForEvidence,
    ReadyForReview,
    HandedOff,
    Stale,
    Unavailable,
    Conflicting,
}

impl IntentStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Working => "Working",
            Self::WaitingApproval => "Waiting for approval",
            Self::Blocked => "Blocked",
            Self::Failed => "Failed",
            Self::ProofPending => "Proof pending",
            Self::WaitingForEvidence => "Ready observed; evidence missing",
            Self::ReadyForReview => "Ready for review",
            Self::HandedOff => "Handed off",
            Self::Stale => "Stale source",
            Self::Unavailable => "Source unavailable",
            Self::Conflicting => "Conflicting evidence",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Attention {
    None,
    Approval,
    Blocked,
    ValidationFailure,
    ProofFailure,
    HandoffDecision,
    SourceConflict,
    AuthorityUnavailable,
    StaleSource,
    CustodyMissing,
    EvidenceRequired,
    UnsupportedSource,
}

impl Attention {
    pub fn label(&self) -> &'static str {
        match self {
            Self::None => "No action needed",
            Self::Approval => "Needs your decision",
            Self::Blocked => "Blocked",
            Self::ValidationFailure => "Validation failed",
            Self::ProofFailure => "Proof failed",
            Self::HandoffDecision => "Needs handoff decision",
            Self::SourceConflict => "Source conflict",
            Self::AuthorityUnavailable => "Authority unavailable",
            Self::StaleSource => "Source is stale",
            Self::CustodyMissing => "Custody is incomplete",
            Self::EvidenceRequired => "Needs authoritative evidence",
            Self::UnsupportedSource => "Unsupported source event",
        }
    }

    pub fn requires_human(&self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TodoClaimState {
    Unknown,
    Claimed,
    Unclaimed,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoClaim {
    pub id: String,
    pub state: TodoClaimState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceReference {
    pub reference: String,
    pub known: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalReference {
    pub id: String,
    pub state: String,
    pub source: SourceType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blocker {
    pub reference: Option<String>,
    pub cause: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Handoff {
    pub state: String,
    pub reference: Option<String>,
    pub authoritative: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Custody {
    pub run_id: Option<String>,
    pub decapod_session_id: Option<String>,
    pub agent_id: Option<String>,
    pub todo: Option<TodoClaim>,
    pub work_unit_id: Option<String>,
    pub workspace: Option<WorkspaceReference>,
    pub touched_files: Vec<String>,
    pub approvals: Vec<ApprovalReference>,
    pub blockers: Vec<Blocker>,
    pub validation: Evidence,
    pub proof: Evidence,
    pub handoff: Option<Handoff>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedEvent {
    pub event_id: String,
    pub event_type: String,
    pub source_timestamp: String,
    pub source_type: SourceType,
    pub payload: Value,
}

impl From<&SourceEvent> for ObservedEvent {
    fn from(event: &SourceEvent) -> Self {
        Self {
            event_id: event.event_id.clone(),
            event_type: event.event_type.clone(),
            source_timestamp: event.source_timestamp.clone(),
            source_type: event.source_type.clone(),
            payload: event.payload.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    pub event_id: String,
    pub timestamp: String,
    pub text: String,
    pub meaningful: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentProjection {
    pub intent_id: String,
    pub title: Option<String>,
    pub status: IntentStatus,
    pub attention: Attention,
    pub authority: AuthorityState,
    pub freshness: SourceFreshness,
    pub custody: Custody,
    pub events: Vec<ObservedEvent>,
    pub unknown_events: Vec<ObservedEvent>,
    pub diagnostics: Vec<String>,
    pub recent_activity: Vec<Activity>,
}

impl IntentProjection {
    fn new(intent_id: String) -> Self {
        Self {
            intent_id,
            title: None,
            status: IntentStatus::Unknown,
            attention: Attention::None,
            authority: AuthorityState::NotProvided,
            freshness: SourceFreshness::Unknown,
            custody: Custody::default(),
            events: Vec::new(),
            unknown_events: Vec::new(),
            diagnostics: Vec::new(),
            recent_activity: Vec::new(),
        }
    }

    pub fn has_authoritative_ready_evidence(&self) -> bool {
        self.custody.validation.state == EvidenceState::Passed
            && self.custody.proof.state == EvidenceState::Passed
            && self.authority == AuthorityState::Confirmed
            && self.custody.validation.source == Some(SourceType::Decapod)
            && self.custody.proof.source == Some(SourceType::Decapod)
            && self.attention == Attention::None
    }

    pub fn latest_activity(&self) -> Option<&Activity> {
        self.recent_activity.last()
    }
}

#[derive(Debug, Default)]
pub struct ProjectionState {
    intents: BTreeMap<String, IntentProjection>,
    seen_event_ids: BTreeSet<String>,
    pub unscoped_events: Vec<ObservedEvent>,
    pub diagnostics: Vec<String>,
}

impl ProjectionState {
    pub fn projections(&self) -> Vec<IntentProjection> {
        self.intents.values().cloned().collect()
    }

    pub fn projection(&self, intent_id: &str) -> Option<&IntentProjection> {
        self.intents.get(intent_id)
    }

    pub fn seen_event_count(&self) -> usize {
        self.seen_event_ids.len()
    }
}

#[derive(Debug, Default)]
pub struct Reducer {
    state: ProjectionState,
}

impl Reducer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reduce(mut self, events: impl IntoIterator<Item = SourceEvent>) -> ProjectionState {
        let mut ordered: Vec<_> = events.into_iter().collect();
        ordered.sort_by(|left, right| {
            left.source_timestamp
                .cmp(&right.source_timestamp)
                .then_with(|| left.event_id.cmp(&right.event_id))
        });
        for event in ordered {
            self.apply(event);
        }
        self.state
    }

    pub fn apply(&mut self, event: SourceEvent) {
        if !self.state.seen_event_ids.insert(event.event_id.clone()) {
            return;
        }

        let Some(intent_id) = event.intent_id.clone() else {
            self.state.unscoped_events.push(ObservedEvent::from(&event));
            self.state.diagnostics.push(format!(
                "event {} has no intent_id; preserved outside intent projections",
                event.event_id
            ));
            return;
        };

        let projection = self
            .state
            .intents
            .entry(intent_id)
            .or_insert_with_key(|key| IntentProjection::new(key.clone()));

        projection.events.push(ObservedEvent::from(&event));
        projection.freshness = event.freshness.clone();
        copy_custody_ids(projection, &event);

        if !timestamp_is_plausible(&event.source_timestamp) {
            projection.unknown_events.push(ObservedEvent::from(&event));
            projection.attention = Attention::UnsupportedSource;
            projection.diagnostics.push(format!(
                "event {} has malformed source timestamp; state transition ignored",
                event.event_id
            ));
            add_activity(
                projection,
                &event,
                "Malformed source event preserved",
                false,
            );
            return;
        }

        match event.freshness {
            SourceFreshness::Stale { .. } => {
                projection.status = IntentStatus::Stale;
                projection.attention = Attention::StaleSource;
                add_activity(projection, &event, "Source became stale", true);
                return;
            }
            SourceFreshness::Unavailable { ref reason } => {
                projection.status = IntentStatus::Unavailable;
                projection.attention = Attention::AuthorityUnavailable;
                projection.authority = AuthorityState::Unavailable;
                add_activity(
                    projection,
                    &event,
                    &format!("Source unavailable: {reason}"),
                    true,
                );
                return;
            }
            SourceFreshness::Fresh | SourceFreshness::Unknown => {}
        }

        let event_type = event.event_type.as_str();
        let is_decopod = event.source_type == SourceType::Decapod;
        match event_type {
            "intent.created" => {
                projection.title = event.payload_string("title");
                projection.status = IntentStatus::Unknown;
                add_activity(projection, &event, "Intent became visible", true);
            }
            "agent.started" | "work.started" | "work.progress" => {
                projection.status = IntentStatus::Working;
                add_activity(projection, &event, "Working", event_type != "work.progress");
            }
            "todo.claimed" => {
                if let Some(id) = event.todo_id.clone() {
                    projection.custody.todo = Some(TodoClaim {
                        id,
                        state: TodoClaimState::Claimed,
                    });
                }
                projection.status = IntentStatus::Working;
                add_activity(projection, &event, "Todo claimed", true);
            }
            "todo.completed" => {
                if let Some(todo) = projection.custody.todo.as_mut() {
                    todo.state = TodoClaimState::Completed;
                }
                add_activity(projection, &event, "Todo completed", true);
            }
            "approval.requested" => {
                let approval_id = event
                    .payload_string("approval_id")
                    .or_else(|| event.payload_string("interlock_id"));
                if let Some(id) = approval_id {
                    push_approval(
                        projection,
                        ApprovalReference {
                            id,
                            state: "requested".to_string(),
                            source: event.source_type.clone(),
                        },
                    );
                }
                projection.status = IntentStatus::WaitingApproval;
                projection.attention = Attention::Approval;
                add_activity(projection, &event, "Waiting for approval", true);
            }
            "approval.granted" | "approval.denied" => {
                let state = event_type.strip_prefix("approval.").unwrap_or("observed");
                if let Some(approval) = projection.custody.approvals.last_mut() {
                    approval.state = state.to_string();
                }
                add_activity(projection, &event, &format!("Approval {state}"), true);
                if !is_decopod {
                    projection.diagnostics.push(format!(
                        "{} is a local observation and cannot grant authority",
                        event.event_id
                    ));
                } else if state == "granted" {
                    projection.attention = Attention::None;
                }
            }
            "blocked" | "interlock.encountered" => {
                let cause = event
                    .payload_string("reason")
                    .or_else(|| event.payload_string("cause"))
                    .unwrap_or_else(|| "Blocker cause not provided".to_string());
                projection.custody.blockers.push(Blocker {
                    reference: event.payload_string("blocker_id"),
                    cause: cause.clone(),
                });
                projection.status = IntentStatus::Blocked;
                projection.attention = Attention::Blocked;
                add_activity(projection, &event, &format!("Blocked: {cause}"), true);
            }
            "failed" | "work.failed" | "task.failed" => {
                let cause = event
                    .payload_string("cause")
                    .or_else(|| event.payload_string("reason"))
                    .unwrap_or_else(|| "Execution failed".to_string());
                projection.status = IntentStatus::Failed;
                projection.attention = Attention::Blocked;
                projection.custody.blockers.push(Blocker {
                    reference: event.payload_string("failure_id"),
                    cause: cause.clone(),
                });
                add_activity(projection, &event, &format!("Failed: {cause}"), true);
            }
            "validation.passed" | "validation.failed" => {
                let state = if event_type.ends_with("passed") {
                    EvidenceState::Passed
                } else {
                    EvidenceState::Failed
                };
                let details = event.payload_strings("errors");
                if projection.custody.validation.state != EvidenceState::Absent
                    && projection.custody.validation.state != state
                {
                    projection.custody.validation.state = EvidenceState::Conflicting;
                    projection.authority = AuthorityState::Conflicting;
                    projection.status = IntentStatus::Conflicting;
                    projection.attention = Attention::SourceConflict;
                    projection
                        .diagnostics
                        .push("validation evidence conflicts".to_string());
                } else {
                    projection.custody.validation = Evidence {
                        state: state.clone(),
                        reference: event.payload_string("validation_id").or_else(|| {
                            event
                                .payload_string("gate")
                                .map(|gate| format!("gate:{gate}"))
                        }),
                        details,
                        source: Some(event.source_type.clone()),
                        observed_at: Some(event.source_timestamp.clone()),
                    };
                    if is_decopod {
                        projection.authority = AuthorityState::Confirmed;
                    } else {
                        projection.authority = AuthorityState::ObservedOnly;
                    }
                    if state == EvidenceState::Failed {
                        projection.status = IntentStatus::Failed;
                        projection.attention = Attention::ValidationFailure;
                        projection.custody.blockers.push(Blocker {
                            reference: projection.custody.validation.reference.clone(),
                            cause: if projection.custody.validation.details.is_empty() {
                                "Validation failed".to_string()
                            } else {
                                projection.custody.validation.details.join("; ")
                            },
                        });
                    }
                }
                let text = if state == EvidenceState::Passed {
                    "Validation observed"
                } else {
                    "Validation failed"
                };
                add_activity(projection, &event, text, true);
            }
            "proof.pending" => {
                projection.custody.proof = Evidence {
                    state: EvidenceState::Pending,
                    reference: event.payload_string("proof_id"),
                    details: Vec::new(),
                    source: Some(event.source_type.clone()),
                    observed_at: Some(event.source_timestamp.clone()),
                };
                projection.status = IntentStatus::ProofPending;
                projection.attention = Attention::None;
                add_activity(projection, &event, "Proof pending", true);
            }
            "proof.passed" | "proof.verified" | "proof.failed" => {
                let state = if event_type.ends_with("failed") {
                    EvidenceState::Failed
                } else {
                    EvidenceState::Passed
                };
                if projection.custody.proof.state != EvidenceState::Absent
                    && projection.custody.proof.state != EvidenceState::Pending
                    && projection.custody.proof.state != state
                {
                    projection.custody.proof.state = EvidenceState::Conflicting;
                    projection.authority = AuthorityState::Conflicting;
                    projection.status = IntentStatus::Conflicting;
                    projection.attention = Attention::SourceConflict;
                } else {
                    projection.custody.proof = Evidence {
                        state: state.clone(),
                        reference: event.payload_string("proof_id"),
                        details: event.payload_strings("errors"),
                        source: Some(event.source_type.clone()),
                        observed_at: Some(event.source_timestamp.clone()),
                    };
                    projection.authority = if is_decopod {
                        AuthorityState::Confirmed
                    } else {
                        AuthorityState::ObservedOnly
                    };
                    if state == EvidenceState::Failed {
                        projection.status = IntentStatus::Failed;
                        projection.attention = Attention::ProofFailure;
                    }
                }
                add_activity(
                    projection,
                    &event,
                    if state == EvidenceState::Failed {
                        "Proof failed"
                    } else {
                        "Proof verified"
                    },
                    true,
                );
            }
            "ready" => {
                projection.authority = if is_decopod {
                    AuthorityState::Confirmed
                } else {
                    AuthorityState::ObservedOnly
                };
                if projection.has_authoritative_ready_evidence() {
                    projection.status = IntentStatus::ReadyForReview;
                    projection.attention = Attention::None;
                    add_activity(projection, &event, "Ready for review", true);
                } else {
                    projection.status = IntentStatus::WaitingForEvidence;
                    projection.attention = Attention::EvidenceRequired;
                    projection.diagnostics.push(
                        "local ready observation did not assert authoritative completion"
                            .to_string(),
                    );
                    add_activity(projection, &event, "Ready observed; evidence missing", true);
                }
            }
            "handoff.requested" | "handoff.accepted" | "handoff.rejected" => {
                let state = event_type.strip_prefix("handoff.").unwrap_or("observed");
                let authoritative = is_decopod && state != "requested";
                projection.custody.handoff = Some(Handoff {
                    state: state.to_string(),
                    reference: event.payload_string("handoff_id"),
                    authoritative,
                });
                projection.status = IntentStatus::HandedOff;
                projection.attention = if state == "requested" || !authoritative {
                    Attention::HandoffDecision
                } else {
                    Attention::None
                };
                add_activity(projection, &event, "Handoff requires review", true);
            }
            "custody.invalid" => {
                projection.status = IntentStatus::Blocked;
                projection.attention = Attention::CustodyMissing;
                projection.custody.blockers.push(Blocker {
                    reference: event.payload_string("custody_id"),
                    cause: event
                        .payload_string("reason")
                        .unwrap_or_else(|| "Custody is invalid".to_string()),
                });
                add_activity(projection, &event, "Custody is incomplete", true);
            }
            "source.unavailable" => {
                projection.status = IntentStatus::Unavailable;
                projection.attention = Attention::AuthorityUnavailable;
                projection.authority = AuthorityState::Unavailable;
                add_activity(projection, &event, "Source unavailable", true);
            }
            "provider.token" | "tool.started" | "tool.finished" | "command.output" => {
                add_activity(projection, &event, "Execution activity", false);
            }
            _ => {
                let observed = ObservedEvent::from(&event);
                projection.unknown_events.push(observed);
                if projection.status == IntentStatus::Unknown {
                    projection.attention = Attention::UnsupportedSource;
                }
                projection.diagnostics.push(format!(
                    "unknown event type preserved: {}",
                    event.event_type
                ));
                add_activity(projection, &event, "Unknown source event preserved", false);
            }
        }
        refresh_derived_state(projection);
    }

    pub fn state(&self) -> &ProjectionState {
        &self.state
    }
}

fn copy_custody_ids(projection: &mut IntentProjection, event: &SourceEvent) {
    if event.run_id.is_some() {
        projection.custody.run_id = event.run_id.clone();
    }
    if event.decapod_session_id.is_some() {
        projection.custody.decapod_session_id = event.decapod_session_id.clone();
    }
    if event.agent_id.is_some() {
        projection.custody.agent_id = event.agent_id.clone();
    }
    if event.todo_id.is_some() && projection.custody.todo.is_none() {
        projection.custody.todo = event.todo_id.clone().map(|id| TodoClaim {
            id,
            state: TodoClaimState::Unknown,
        });
    }
    if event.work_unit_id.is_some() {
        projection.custody.work_unit_id = event.work_unit_id.clone();
    }
    if let Some(workspace) = event.payload_string("workspace") {
        projection.custody.workspace = Some(WorkspaceReference {
            reference: workspace,
            known: event
                .payload
                .get("workspace_known")
                .and_then(Value::as_bool)
                .unwrap_or(true),
        });
    }
    let touched = event.payload_strings("touched_files");
    for path in touched {
        if !projection.custody.touched_files.contains(&path) {
            projection.custody.touched_files.push(path);
        }
    }
}

fn push_approval(projection: &mut IntentProjection, approval: ApprovalReference) {
    if !projection
        .custody
        .approvals
        .iter()
        .any(|existing| existing.id == approval.id)
    {
        projection.custody.approvals.push(approval);
    }
}

fn add_activity(
    projection: &mut IntentProjection,
    event: &SourceEvent,
    text: &str,
    meaningful: bool,
) {
    projection.recent_activity.push(Activity {
        event_id: event.event_id.clone(),
        timestamp: event.source_timestamp.clone(),
        text: text.to_string(),
        meaningful,
    });
    if projection.recent_activity.len() > 20 {
        let excess = projection.recent_activity.len() - 20;
        projection.recent_activity.drain(0..excess);
    }
}

fn refresh_derived_state(projection: &mut IntentProjection) {
    if matches!(projection.freshness, SourceFreshness::Stale { .. }) {
        projection.status = IntentStatus::Stale;
        projection.attention = Attention::StaleSource;
        return;
    }
    if matches!(projection.freshness, SourceFreshness::Unavailable { .. }) {
        projection.status = IntentStatus::Unavailable;
        projection.attention = Attention::AuthorityUnavailable;
        return;
    }
    if projection.custody.validation.state == EvidenceState::Conflicting
        || projection.custody.proof.state == EvidenceState::Conflicting
    {
        projection.status = IntentStatus::Conflicting;
        projection.attention = Attention::SourceConflict;
        projection.authority = AuthorityState::Conflicting;
        return;
    }
    if projection.custody.validation.state == EvidenceState::Failed {
        projection.status = IntentStatus::Failed;
        projection.attention = Attention::ValidationFailure;
        return;
    }
    if projection.custody.proof.state == EvidenceState::Failed {
        projection.status = IntentStatus::Failed;
        projection.attention = Attention::ProofFailure;
        return;
    }
    if projection.custody.validation.state == EvidenceState::Passed
        && projection.custody.proof.state == EvidenceState::Passed
        && projection.authority == AuthorityState::Confirmed
    {
        projection.status = IntentStatus::ReadyForReview;
        projection.attention = Attention::None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::SourceEvent;
    use serde_json::json;

    fn event(id: &str, kind: &str, source: SourceType) -> SourceEvent {
        SourceEvent::new(id, kind, "2026-07-21T00:00:00Z", source, "intent/test")
    }

    #[test]
    fn local_ready_does_not_become_authoritative() {
        let state = Reducer::new().reduce([event("1", "ready", SourceType::Pincher)]);
        let intent = state.projection("intent/test").expect("projection");
        assert_eq!(intent.status, IntentStatus::WaitingForEvidence);
        assert_eq!(intent.authority, AuthorityState::ObservedOnly);
        assert!(!intent.has_authoritative_ready_evidence());
    }

    #[test]
    fn conflicting_validation_is_visible() {
        let mut passed = event("1", "validation.passed", SourceType::Decapod);
        passed.payload = json!({"validation_id":"v1"});
        let mut failed = event("2", "validation.failed", SourceType::Decapod);
        failed.payload = json!({"errors":["gate failed"]});
        let state = Reducer::new().reduce([passed, failed]);
        let intent = state.projection("intent/test").expect("projection");
        assert_eq!(intent.status, IntentStatus::Conflicting);
        assert_eq!(intent.attention, Attention::SourceConflict);
    }

    #[test]
    fn replay_is_deterministic_and_duplicate_events_are_idempotent() {
        let first = event("2", "work.started", SourceType::Pincher).with_run("run-2");
        let second = event("1", "work.progress", SourceType::Pincher);
        let ordered = Reducer::new().reduce([first.clone(), second.clone()]);
        let replayed = Reducer::new().reduce([second, first.clone(), first]);
        assert_eq!(ordered.projections(), replayed.projections());
        assert_eq!(replayed.seen_event_count(), 2);
    }

    #[test]
    fn unknown_events_are_preserved_without_becoming_success() {
        let state = Reducer::new().reduce([event("1", "future.phase.v4", SourceType::Pincher)]);
        let intent = state.projection("intent/test").expect("projection");
        assert_eq!(intent.unknown_events.len(), 1);
        assert_ne!(intent.status, IntentStatus::ReadyForReview);
    }

    #[test]
    fn stale_and_unavailable_sources_remain_visible() {
        let stale = event("1", "source.stale", SourceType::Pincher)
            .with_freshness(SourceFreshness::Stale { age_seconds: 9 });
        let unavailable = event("2", "source.unavailable", SourceType::Decapod).with_freshness(
            SourceFreshness::Unavailable {
                reason: "timeout".to_string(),
            },
        );
        let stale_state = Reducer::new().reduce([stale]);
        let unavailable_state = Reducer::new().reduce([unavailable]);
        assert_eq!(
            stale_state
                .projection("intent/test")
                .expect("projection")
                .status,
            IntentStatus::Stale
        );
        assert_eq!(
            unavailable_state
                .projection("intent/test")
                .expect("projection")
                .authority,
            AuthorityState::Unavailable
        );
    }

    #[test]
    fn custody_ids_survive_projection() {
        let event = event("1", "work.started", SourceType::Pincher)
            .with_run("run-1")
            .with_session("session-1")
            .with_agent("agent-1")
            .with_todo("todo-1")
            .with_work_unit("workunit-1");
        let state = Reducer::new().reduce([event]);
        let custody = &state.projection("intent/test").expect("projection").custody;
        assert_eq!(custody.run_id.as_deref(), Some("run-1"));
        assert_eq!(custody.decapod_session_id.as_deref(), Some("session-1"));
        assert_eq!(custody.agent_id.as_deref(), Some("agent-1"));
        assert_eq!(
            custody.todo.as_ref().map(|todo| todo.id.as_str()),
            Some("todo-1")
        );
        assert_eq!(custody.work_unit_id.as_deref(), Some("workunit-1"));
    }

    #[test]
    fn failure_causes_remain_inspectable() {
        let mut failed = event("1", "failed", SourceType::Pincher);
        failed.payload = json!({"cause":"provider exited"});
        let state = Reducer::new().reduce([failed]);
        let intent = state.projection("intent/test").expect("projection");
        assert_eq!(intent.status, IntentStatus::Failed);
        assert!(
            intent
                .custody
                .blockers
                .iter()
                .any(|blocker| blocker.cause == "provider exited")
        );
    }
}
