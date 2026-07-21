use crate::events::SourceEvent;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("source unavailable: {0}")]
    Unavailable(String),
}

/// The adapter boundary is intentionally small. A future Pincher transport can
/// implement this trait without changing the reducer, app state, or UI.
pub trait EventSourceAdapter {
    fn name(&self) -> &str;
    fn load(&self) -> Result<Vec<SourceEvent>, SourceError>;
}

#[derive(Debug, Clone)]
pub struct FixtureSource {
    pub scenario: crate::fixtures::FixtureScenario,
}

impl FixtureSource {
    pub fn new(scenario: crate::fixtures::FixtureScenario) -> Self {
        Self { scenario }
    }
}

impl EventSourceAdapter for FixtureSource {
    fn name(&self) -> &str {
        "fixture/replay (provisional)"
    }

    fn load(&self) -> Result<Vec<SourceEvent>, SourceError> {
        Ok(crate::fixtures::events_for(self.scenario))
    }
}
