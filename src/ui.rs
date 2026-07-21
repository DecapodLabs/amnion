use crate::app::{App, Verbosity};
use crate::projection::{Attention, IntentProjection};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

pub fn draw(frame: &mut Frame<'_>, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(2),
        ])
        .split(frame.area());

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " AMNION ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" governed work, quietly visible"),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(header, outer[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(34), Constraint::Min(35)])
        .split(outer[1]);
    draw_intent_list(frame, app, body[0]);
    draw_detail(frame, app, body[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
        Span::raw(" intents   "),
        Span::styled("Enter/d", Style::default().fg(Color::Cyan)),
        Span::raw(" detail   "),
        Span::styled("v", Style::default().fg(Color::Cyan)),
        Span::raw(" verbosity   "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(" quit"),
    ]))
    .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, outer[2]);
}

fn draw_intent_list(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let items = app
        .projections()
        .iter()
        .enumerate()
        .map(|(index, projection)| {
            let marker = if projection.attention.requires_human() {
                "!"
            } else {
                "·"
            };
            let name = projection.title.as_deref().unwrap_or(&projection.intent_id);
            let text = format!(
                "{marker} {}\n  {}",
                sanitize(name),
                projection.status.label()
            );
            let style = if index == app.selected_index() {
                Style::default().fg(Color::Black).bg(Color::Cyan)
            } else if projection.attention.requires_human() {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(text).style(style)
        })
        .collect::<Vec<_>>();
    let list = List::new(items)
        .block(Block::default().title(" Intents ").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(list, area);
}

fn draw_detail(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let Some(projection) = app.selected_projection() else {
        frame.render_widget(
            Paragraph::new("No governed intents available").block(
                Block::default()
                    .title(" Selected intent ")
                    .borders(Borders::ALL),
            ),
            area,
        );
        return;
    };

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(5)])
        .split(area);
    let summary = summary_lines(projection, app.verbosity);
    frame.render_widget(
        Paragraph::new(Text::from(summary))
            .block(
                Block::default()
                    .title(" Custody spine ")
                    .borders(Borders::ALL),
            )
            .wrap(Wrap { trim: true }),
        vertical[0],
    );

    let mut lines = app
        .visible_activity()
        .into_iter()
        .map(|activity| {
            let prefix = if activity.meaningful { "•" } else { "·" };
            Line::from(format!(
                "{prefix} {}  {}",
                activity.timestamp,
                sanitize(&activity.text)
            ))
        })
        .collect::<Vec<_>>();
    if app.detail_open {
        lines.extend(detail_lines(projection, app.verbosity));
    } else if lines.is_empty() {
        lines.push(Line::from(
            "No meaningful activity yet. Press Enter for detail.",
        ));
    }
    let title = format!(
        " Activity · {} · {} ",
        app.verbosity.label(),
        if app.detail_open { "open" } else { "quiet" }
    );
    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(Block::default().title(title).borders(Borders::ALL))
            .wrap(Wrap { trim: true }),
        vertical[1],
    );
}

fn summary_lines(projection: &IntentProjection, verbosity: Verbosity) -> Vec<Line<'static>> {
    let name = projection.title.as_deref().unwrap_or("Untitled intent");
    let todo = projection
        .custody
        .todo
        .as_ref()
        .map(|todo| format!("{} ({:?})", todo.id, todo.state))
        .unwrap_or_else(|| "Unknown".to_string());
    let workspace = projection
        .custody
        .workspace
        .as_ref()
        .map(|workspace| workspace.reference.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    vec![
        Line::from(vec![
            Span::styled(
                sanitize(name),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            status_span(projection),
        ]),
        Line::from(format!("Attention: {}", projection.attention.label())),
        Line::from(format!("Authority: {}", projection.authority.label())),
        Line::from(format!("Freshness: {}", projection.freshness.label())),
        Line::from(format!(
            "Todo: {todo}   Workspace: {}",
            sanitize(&workspace)
        )),
        Line::from(format!(
            "Validation: {:?}   Proof: {:?}",
            projection.custody.validation.state, projection.custody.proof.state
        )),
        if verbosity == Verbosity::Quiet {
            Line::from("Press Enter to inspect custody, evidence, and source events.")
        } else {
            Line::from(format!(
                "Custody: run={:?} session={:?} work-unit={:?}",
                projection.custody.run_id,
                projection.custody.decapod_session_id,
                projection.custody.work_unit_id
            ))
        },
    ]
}

fn status_span(projection: &IntentProjection) -> Span<'static> {
    let color = match projection.attention {
        Attention::None => Color::Green,
        Attention::Approval | Attention::EvidenceRequired | Attention::HandoffDecision => {
            Color::Yellow
        }
        Attention::StaleSource | Attention::AuthorityUnavailable => Color::Magenta,
        _ => Color::Red,
    };
    Span::styled(
        projection.status.label().to_string(),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

fn detail_lines(projection: &IntentProjection, verbosity: Verbosity) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(format!("Intent ID: {}", sanitize(&projection.intent_id))),
        Line::from(format!(
            "Touched files: {}",
            if projection.custody.touched_files.is_empty() {
                "Unknown".to_string()
            } else {
                projection
                    .custody
                    .touched_files
                    .iter()
                    .map(|path| sanitize(path))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        )),
        Line::from(format!(
            "Approvals: {}",
            projection
                .custody
                .approvals
                .iter()
                .map(|approval| format!("{} [{}]", sanitize(&approval.id), approval.state))
                .collect::<Vec<_>>()
                .join(", ")
                .if_empty(|| "None".to_string())
        )),
        Line::from(format!(
            "Blockers: {}",
            projection
                .custody
                .blockers
                .iter()
                .map(|blocker| sanitize(&blocker.cause))
                .collect::<Vec<_>>()
                .join("; ")
                .if_empty(|| "None".to_string())
        )),
        Line::from(format!(
            "Diagnostics: {}",
            projection
                .diagnostics
                .iter()
                .map(|diagnostic| sanitize(diagnostic))
                .collect::<Vec<_>>()
                .join("; ")
                .if_empty(|| "None".to_string())
        )),
        Line::from(format!(
            "Unknown events preserved: {}",
            projection.unknown_events.len()
        )),
    ];
    if matches!(verbosity, Verbosity::Detailed | Verbosity::Debug) {
        lines.extend(projection.events.iter().map(|event| {
            Line::from(format!(
                "event {} · {} · {}",
                sanitize(&event.event_id),
                sanitize(&event.event_type),
                event.source_type.label()
            ))
        }));
    }
    if verbosity == Verbosity::Debug {
        lines.extend(projection.unknown_events.iter().map(|event| {
            Line::from(format!(
                "raw {}: {}",
                sanitize(&event.event_id),
                sanitize(&event.payload.to_string())
            ))
        }));
    }
    lines
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_control() || matches!(character, '\n' | '\t'))
        .collect()
}

trait EmptyFallback {
    fn if_empty(self, fallback: impl FnOnce() -> String) -> String;
}

impl EmptyFallback for String {
    fn if_empty(self, fallback: impl FnOnce() -> String) -> String {
        if self.is_empty() { fallback() } else { self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fixtures, projection::Reducer};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn fixture_scenarios_render_without_panic() {
        for scenario in fixtures::all_scenarios() {
            let state = Reducer::new().reduce(fixtures::events_for(scenario));
            let app = App::new(state.projections());
            let backend = TestBackend::new(100, 40);
            let mut terminal = Terminal::new(backend).expect("terminal");
            terminal.draw(|frame| draw(frame, &app)).expect("draw");
        }
    }

    #[test]
    fn malformed_text_is_stripped_before_rendering() {
        assert_eq!(sanitize("safe\u{1b}[31m text"), "safe[31m text");
    }
}
