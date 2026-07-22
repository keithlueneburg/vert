use std::time::Duration;

use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::ui;

/// How long the event loop blocks before redrawing anyway.
/// Keeps animations/clock updates alive when the user is idle.
const TICK_RATE: Duration = Duration::from_millis(250);

/// All application state lives here. The UI is a pure function of it.
pub struct App {
    pub running: bool,
    pub ticks: u64,
    pub counter: i64,
}

/// Everything that can change `App`. Input and time both funnel through this.
enum Action {
    Quit,
    Tick,
    Increment,
    Decrement,
    None,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            ticks: 0,
            counter: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| ui::draw(frame, &self))?;
            let action = next_action()?;
            self.update(action);
        }
        Ok(())
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.running = false,
            Action::Tick => self.ticks += 1,
            Action::Increment => self.counter += 1,
            Action::Decrement => self.counter -= 1,
            Action::None => {}
        }
    }
}

fn next_action() -> Result<Action> {
    if !event::poll(TICK_RATE)? {
        return Ok(Action::Tick);
    }
    Ok(match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => map_key(key),
        _ => Action::None,
    })
}

fn map_key(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
        KeyCode::Up | KeyCode::Char('k') => Action::Increment,
        KeyCode::Down | KeyCode::Char('j') => Action::Decrement,
        _ => Action::None,
    }
}
