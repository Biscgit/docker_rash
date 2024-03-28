use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{backend::Backend, Frame};
use ratatui::layout::{Constraint, Layout, Rect};
use tokio::sync::mpsc::UnboundedSender;

use crate::tui::Tui;
use crate::event::Event;
use crate::components::Component;
use crate::types::AppResult;
use crate::components::sidebar::SideBar;


pub enum Action {
    Tick,
    Resume,
    Suspend,
    Refresh,
    Key(KeyEvent),
}


pub struct App {
    sender: UnboundedSender<Event>,
    sidebar_widget: SideBar,
}


impl App {
    pub async fn run<B: Backend>(&mut self, mut tui: Tui<B>) -> AppResult<()> {
        // first init render
        self.sender.send(Event::Render)?;

        // loop until exit
        loop {
            match tui.events.next().await? {
                Event::Quit => {
                    tui.exit()?;
                    return Ok(());
                }
                Event::Render => {
                    let func = |frame: &mut Frame| {
                        self.render(frame, frame.size())
                    };
                    tui.draw(func)?;
                }
                Event::Key(key_event) => {
                    self.handle_key_events(key_event);
                    self.sender.send(Event::Render)?
                }
                Event::Tick => {}
                _ => {}
            }
        }
    }
}

impl Component for App {
    fn new(sender: UnboundedSender<Event>) -> AppResult<Self> {
        Ok(Self {
            sender: sender.clone(),
            sidebar_widget: SideBar::new(sender.clone())?,
        })
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.sender.send(Event::Quit).unwrap();
            }
            _ => {
                self.sidebar_widget.handle_key_events(key);
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let main_layout = Layout::horizontal([
            Constraint::Length(21),
            Constraint::Min(20),
        ]).split(area);

        self.sidebar_widget.render(frame, main_layout[0]);
    }
}


