use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    layout::{Rect, Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, ListState, Padding, Paragraph},
};
use serde::de::Unexpected::Option;

use crate::action::Action;
use crate::components::methods::set_background;


const LIGHT_BACKGROUND: Color = Color::Rgb(52, 52, 60);
const DARK_BACKGROUND: Color = Color::Rgb(34, 34, 40);


struct SideBar {
    current_selected: ListState,
    list_length: usize,
}

impl Default for SideBar {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        SideBar {
            current_selected: state,
            list_length: 5,
        }
    }
}

impl SideBar {
    pub fn set_index(&mut self, index: Option<usize>) {
        match index {
            Some(i) => {
                if self.list_length > 0 && i < self.list_length {
                    self.current_selected.select(index);
                }
            }
            None => {
                self.current_selected.select(None);
            }
        }
    }

    pub fn get_index(&self) -> Option<usize> {
        self.current_selected.clone()
    }

    pub fn next(&mut self) {
        if let Some(index) = self.current_selected.selected() {
            if self.list_length > 0 {
                self.current_selected.select(
                    Some((index + 1).rem_euclid(self.list_length)
                    ));
            }
        }
    }

    pub fn previous(&mut self) {
        if let Some(index) = self.current_selected.selected() {
            if self.current_selected > 0 {
                self.current_selected.select(
                    Some((index - 1).rem_euclid(self.list_length))
                );
            }
        }
    }
}

impl SideBar {
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Tab => {}
            KeyCode::BackTab => {}
            _ => {}
        }
        Action::Render
    }
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        set_background(DARK_BACKGROUND, frame, area);

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
            match self.current_selected.selected() {
                // highlight
                Some(i) if i == index => {
                    let block = Block::new()
                        .borders(Borders::NONE)
                        .padding(Padding::new(2, 0, 1, 1))
                        .bg(LIGHT_BACKGROUND);

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
}