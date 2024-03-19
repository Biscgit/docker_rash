use ratatui::{
    layout::{Layout, Alignment},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Gauge},
    Frame,
};
use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::widgets::Borders;

use crate::app::App;

/// renders application
pub fn render(app: &mut App, frame: &mut Frame) {
    let area: Rect = set_background(frame);
}

/// set background
fn set_background(frame: &mut Frame) -> Rect {
    let area = frame.size();
    let background = Block::default()
        .borders(Borders::NONE)
        .style(Style::new().bg(Color::Rgb(32, 32, 38)));

    frame.render_widget(&background, area);
    background.inner(area)
}


