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
    set_background(Color::Rgb(34, 34, 40), frame, area);
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
    let pages = ["  Statistics", "  Containers", "  Images", "󰋊  Volumes"];
    pages
        .iter()
        .enumerate()
        .for_each(&mut stylize);

    // add credit at bottom
    stylize((sidebar_layout.len() - 1, &"  Credits"));
}










pub fn test_render(app: &mut App, frame: &mut Frame) {
    // let area: Rect = set_background(frame);
    let area: Rect = frame.size();
    let example_vector = vec![
        CTE::example(),
        CTE::example(),
        CTE::example(),
    ];

    let header = Row::new(["A", "B", "C"]);
    let rows = vec![
        Row::new(["1", "2", "3"]),
        Row::new(["1", "2", "3"]),
        Row::new(["1", "2", "3"]),
    ];

    let table = Table::new(
        rows,
        [
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
        ],
    )
        .header(header);

    frame.render_widget(table, area);
}

pub struct ContainerList {
    selected: Option<usize>,
    elements: Vec<ContainerElement>,
}

impl Default for ContainerList {
    fn default() -> Self {
        ContainerList {
            selected: None,
            elements: Vec::with_capacity(16),
        }
    }
}

impl ContainerList {
    pub fn set_index(&mut self, index: Option<usize>) {
        match index {
            Some(i) => {
                let list_length = self.elements.len();
                if list_length > 0 && i < list_length {
                    self.selected = index;
                }
            }
            None => {
                self.selected = None;
            }
        }
    }

    pub fn next(&mut self) {
        if let Some(index) = self.selected {
            if self.elements.len() > 0 {
                self.selected = Some((index + 1).rem_euclid(self.elements.len()));
            }
        }
    }

    pub fn previous(&mut self) {
        if let Some(index) = self.selected {
            if self.elements.len() > 0 {
                self.selected = Some((index - 1).rem_euclid(self.elements.len()));
            }
        }
    }
}

impl Widget for ContainerList {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        todo!("Implement rendering");
    }
}

pub struct ContainerElement {}

pub struct ContainerEntry {}


