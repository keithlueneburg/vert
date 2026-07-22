use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};

use crate::app::App;

/// Rebuilt from scratch every frame. Hold no state here.
pub fn draw(frame: &mut Frame, app: &App) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    frame.render_widget(
        Paragraph::new("vert").bold().centered().block(Block::bordered()),
        header,
    );

    let lines = vec![
        Line::from(format!("counter: {}", app.counter)),
        Line::from(format!("ticks:   {}", app.ticks)),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("state")),
        body,
    );

    frame.render_widget(
        Paragraph::new("j/k or ↑/↓ to count  ·  q to quit").style(Style::new().dim()),
        footer,
    );
}
