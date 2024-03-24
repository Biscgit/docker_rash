use ratatui::{
    layout::Layout,
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph, Gauge, Table, ListItem, List},
    text::Line,
    Frame,
};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Widget;
use ratatui::widgets::{Borders, ListState, Padding, Row};

use crate::app::App;
use crate::docker::models::ContainerEntry as CTE;


/// renders application
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();
    let main_layout = Layout::horizontal([
        Constraint::Length(21),
        Constraint::Min(20),
    ]).split(area);

    render_sidebar(app, frame, main_layout[0]);
    set_background(Color::Rgb(52, 52, 60), frame, main_layout[1]);
}

/// set background
fn set_background(color: Color, frame: &mut Frame, area: Rect) -> Rect {
    let background = Block::default()
        .borders(Borders::NONE)
        .style(Style::new().bg(color));

    frame.render_widget(&background, area);
    background.inner(area)
}

/// renders the sidebar in the current app's state
fn render_sidebar(app: &mut App, frame: &mut Frame, area: Rect) {
    // set background
    set_background(Color::Rgb(32, 32, 38), frame, area);
    let mut state = ListState::default();
    state.select(Some(1));

    // prepare layout for sidebar
    let sidebar_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(0),
        Constraint::Length(3),
    ]).split(area);

    // define stylizing render function
    let mut stylize = |(index, text): (usize, &&str)| {
        match state.selected() {
            // highlight
            Some(i) if i == index => {
                let block = Block::new()
                    .borders(Borders::NONE)
                    .padding(Padding::new(2, 0, 1, 1))
                    .bg(Color::Rgb(52, 52, 60));

                // create an indent effect
                let indent_layout = Layout::horizontal([
                    Constraint::Length(2),
                    Constraint::Fill(0),
                ]).split(sidebar_layout[index]);
                frame.render_widget(
                    Paragraph::new(text.bold().white()).block(block),
                    indent_layout[1],
                );
            }
            // all other
            _ => {
                let block = Block::new()
                    .borders(Borders::NONE)
                    .padding(Padding::new(2, 0, 1, 1));

                frame.render_widget(
                    Paragraph::new(text.gray()).block(block),
                    sidebar_layout[index],
                );
            }
        }
    };

    // render
    let pages = ["  Statistics", "  Containers", "  Images", "󰋊  Volumes"];
    pages
        .iter()
        .enumerate()
        .for_each(&mut stylize);

    // add credit at bottom
    stylize((sidebar_layout.len() - 1, &"  Credits"));
}
