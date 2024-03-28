use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders};

pub fn set_background(color: Color, frame: &mut Frame, area: Rect) -> Rect {
    let background = Block::default()
        .borders(Borders::NONE)
        .style(Style::new().bg(color));

    frame.render_widget(&background, area);
    background.inner(area)
}