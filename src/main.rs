use amnion::{app::App, fixtures, projection::Reducer, ui};
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Reducer::new().reduce(fixtures::demo_events());
    let mut app = App::new(state.projections());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, app))?;
        if event::poll(std::time::Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
        {
            app.handle_key(key);
        }
    }
    Ok(())
}
