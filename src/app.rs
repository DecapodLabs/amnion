use crate::projection::{Activity, IntentProjection};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Summary,
    Detailed,
    Debug,
}

impl Verbosity {
    pub fn label(self) -> &'static str {
        match self {
            Self::Quiet => "Quiet",
            Self::Summary => "Summary",
            Self::Detailed => "Detailed",
            Self::Debug => "Debug",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Quiet => Self::Summary,
            Self::Summary => Self::Detailed,
            Self::Detailed => Self::Debug,
            Self::Debug => Self::Quiet,
        }
    }
}

#[derive(Debug)]
pub struct App {
    projections: Vec<IntentProjection>,
    selected: usize,
    pub detail_open: bool,
    pub verbosity: Verbosity,
    pub should_quit: bool,
}

impl App {
    pub fn new(mut projections: Vec<IntentProjection>) -> Self {
        projections.sort_by(|left, right| left.intent_id.cmp(&right.intent_id));
        Self {
            projections,
            selected: 0,
            detail_open: false,
            verbosity: Verbosity::Quiet,
            should_quit: false,
        }
    }

    pub fn projections(&self) -> &[IntentProjection] {
        &self.projections
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn selected_projection(&self) -> Option<&IntentProjection> {
        self.projections.get(self.selected)
    }

    pub fn visible_activity(&self) -> Vec<&Activity> {
        let Some(projection) = self.selected_projection() else {
            return Vec::new();
        };
        projection
            .recent_activity
            .iter()
            .filter(|activity| match self.verbosity {
                Verbosity::Quiet => activity.meaningful,
                Verbosity::Summary => true,
                Verbosity::Detailed | Verbosity::Debug => true,
            })
            .collect()
    }

    pub fn next_intent(&mut self) {
        if !self.projections.is_empty() {
            self.selected = (self.selected + 1) % self.projections.len();
        }
    }

    pub fn previous_intent(&mut self) {
        if !self.projections.is_empty() {
            self.selected = self
                .selected
                .checked_sub(1)
                .unwrap_or(self.projections.len() - 1);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.next_intent(),
            KeyCode::Up | KeyCode::Char('k') => self.previous_intent(),
            KeyCode::Enter | KeyCode::Char('d') => self.detail_open = !self.detail_open,
            KeyCode::Char('v') => self.verbosity = self.verbosity.next(),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fixtures, projection::Reducer};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn controls_change_only_local_view_state() {
        let state = Reducer::new().reduce(fixtures::events_for(
            fixtures::FixtureScenario::ActiveHealthy,
        ));
        let before = state.projections();
        let mut app = App::new(before.clone());
        app.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.verbosity, Verbosity::Summary);
        assert!(app.detail_open);
        assert_eq!(app.selected_projection(), Some(&before[0]));
    }

    #[test]
    fn quiet_mode_keeps_blocking_activity_visible() {
        let state = Reducer::new().reduce(fixtures::events_for(
            fixtures::FixtureScenario::ValidationFailure,
        ));
        let app = App::new(state.projections());
        assert!(
            app.visible_activity()
                .iter()
                .any(|activity| activity.text.contains("failed"))
        );
    }
}
