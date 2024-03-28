use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    layout::{Rect, Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, ListState, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::components::background::set_background;
use crate::components::Component;
use crate::event::Event;
use crate::types::AppResult;


const LIGHT_BACKGROUND: Color = Color::Rgb(52, 52, 60);
const DARK_BACKGROUND: Color = Color::Rgb(34, 34, 40);


pub struct SideBar {
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
        self.current_selected.selected().clone()
    }

    pub fn next(&mut self) {
        if self.list_length > 0 {
            self.current_selected.select(Some(
                match self.current_selected.selected() {
                    Some(index) => (index + 1).rem_euclid(self.list_length),
                    None => 0,
                })
            )
        }
    }

    pub fn previous(&mut self) {
        if self.list_length > 0 {
            self.current_selected.select(Some(
                match self.current_selected.selected() {
                    Some(index) => (index + self.list_length - 1).rem_euclid(self.list_length),
                    None => 0,
                })
            )
        }
    }
}

impl Component for SideBar {
    fn new(_sender: UnboundedSender<Event>) -> AppResult<Self> {
        let mut state = ListState::default();
        state.select(Some(0));

        Ok(SideBar {
            current_selected: state,
            list_length: 5,
        })
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.next();
            }
            KeyCode::BackTab => {
                self.previous();
            }
            _ => {}
        }
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
        let mut stylize = |(index, offset, text): (usize, usize, &&str)| {
            let block: Block = Block::new()
                .borders(Borders::NONE)
                .padding(Padding::new(2, 0, 1, 1));

            // apply offset and run highlight
            match self.current_selected.selected() {
                // highlight
                Some(i) if i + offset == index => {
                    // create an indent effect
                    let indent_layout = Layout::horizontal([
                        Constraint::Length(2),
                        Constraint::Fill(0),
                    ]).split(sidebar_layout[index]);

                    frame.render_widget(
                        Paragraph::new(text.bold().white()).block(block.bg(LIGHT_BACKGROUND)),
                        indent_layout[1],
                    );
                }
                // all other
                _ => {
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
            .map(|(index, text)| (index, 0, text))
            .for_each(&mut stylize);

        // add credit at bottom
        stylize((sidebar_layout.len() - 1, 1, &"  Credits"));
    }
}