use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// The producer boundary is provisional until Pincher publishes a compatible
/// host contract. `Fixture` is intentionally distinct from a real producer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Pincher,
    Decapod,
    Fixture,
    Unknown(String),
}

impl SourceType {
    pub fn label(&self) -> &str {
        match self {
            Self::Pincher => "Pincher",
            Self::Decapod => "Decapod",
            Self::Fixture => "fixture",
            Self::Unknown(value) => value.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SourceFreshness {
    Fresh,
    Stale {
        age_seconds: u64,
    },
    Unavailable {
        reason: String,
    },
    #[default]
    Unknown,
}

impl SourceFreshness {
    pub fn label(&self) -> String {
        match self {
            Self::Fresh => "Fresh".to_string(),
            Self::Stale { age_seconds } => format!("Stale ({age_seconds}s)"),
            Self::Unavailable { reason } => format!("Unavailable: {reason}"),
            Self::Unknown => "Unknown freshness".to_string(),
        }
    }
}

/// A transport-neutral event envelope shaped by the currently available
/// Pincher broker types. Its string event type is intentional: unknown future
/// events must survive projection instead of being discarded by deserialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceEvent {
    pub event_id: String,
    pub event_type: String,
    pub source_timestamp: String,
    pub source_type: SourceType,
    #[serde(default)]
    pub freshness: SourceFreshness,
    pub intent_id: Option<String>,
    pub run_id: Option<String>,
    pub decapod_session_id: Option<String>,
    pub agent_id: Option<String>,
    pub todo_id: Option<String>,
    pub work_unit_id: Option<String>,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Debug, Error)]
pub enum EventParseError {
    #[error("invalid source event JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("source event is missing {0}")]
    MissingField(&'static str),
    #[error("source event has invalid {0}")]
    InvalidField(&'static str),
}

impl SourceEvent {
    pub fn new(
        event_id: impl Into<String>,
        event_type: impl Into<String>,
        timestamp: impl Into<String>,
        source_type: SourceType,
        intent_id: impl Into<String>,
    ) -> Self {
        Self {
            event_id: event_id.into(),
            event_type: event_type.into(),
            source_timestamp: timestamp.into(),
            source_type,
            freshness: SourceFreshness::Fresh,
            intent_id: Some(intent_id.into()),
            run_id: None,
            decapod_session_id: None,
            agent_id: None,
            todo_id: None,
            work_unit_id: None,
            payload: Value::Object(serde_json::Map::new()),
        }
    }

    pub fn from_json(input: &str) -> Result<Self, EventParseError> {
        let event: Self = serde_json::from_str(input)?;
        if event.event_id.trim().is_empty() {
            return Err(EventParseError::MissingField("event_id"));
        }
        if event.event_type.trim().is_empty() {
            return Err(EventParseError::MissingField("event_type"));
        }
        if event.source_timestamp.trim().is_empty() {
            return Err(EventParseError::MissingField("source_timestamp"));
        }
        if !timestamp_is_plausible(&event.source_timestamp) {
            return Err(EventParseError::InvalidField("source_timestamp"));
        }
        Ok(event)
    }

    pub fn with_payload(mut self, payload: Value) -> Self {
        self.payload = payload;
        self
    }

    pub fn with_run(mut self, run_id: impl Into<String>) -> Self {
        self.run_id = Some(run_id.into());
        self
    }

    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.decapod_session_id = Some(session_id.into());
        self
    }

    pub fn with_agent(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self
    }

    pub fn with_todo(mut self, todo_id: impl Into<String>) -> Self {
        self.todo_id = Some(todo_id.into());
        self
    }

    pub fn with_work_unit(mut self, work_unit_id: impl Into<String>) -> Self {
        self.work_unit_id = Some(work_unit_id.into());
        self
    }

    pub fn with_freshness(mut self, freshness: SourceFreshness) -> Self {
        self.freshness = freshness;
        self
    }

    pub fn payload_string(&self, key: &str) -> Option<String> {
        self.payload
            .get(key)
            .and_then(Value::as_str)
            .map(str::to_owned)
    }

    pub fn payload_strings(&self, key: &str) -> Vec<String> {
        self.payload
            .get(key)
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    }
}

pub(crate) fn timestamp_is_plausible(timestamp: &str) -> bool {
    let bytes = timestamp.as_bytes();
    bytes.len() >= 20
        && bytes.get(4) == Some(&b'-')
        && bytes.get(7) == Some(&b'-')
        && bytes.get(10) == Some(&b'T')
        && (timestamp.ends_with('Z')
            || timestamp
                .bytes()
                .skip(19)
                .any(|byte| byte == b'+' || byte == b'-'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_external_event_is_rejected_without_guessing_success() {
        let error = SourceEvent::from_json(
            r#"{"event_type":"ready","source_timestamp":"2026-07-21T00:00:00Z"}"#,
        )
        .expect_err("missing event id must fail closed");
        assert!(error.to_string().contains("event_id"));
    }

    #[test]
    fn malformed_timestamp_is_rejected() {
        let input = r#"{"event_id":"e1","event_type":"ready","source_timestamp":"later","source_type":"pincher","intent_id":"i1"}"#;
        assert!(matches!(
            SourceEvent::from_json(input),
            Err(EventParseError::InvalidField("source_timestamp"))
        ));
    }
}
