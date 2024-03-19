use std::error;
use ratatui::backend::Backend;

use crate::tui::Tui;
use crate::event::Event;
use crate::handler::handle_key_events;

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

pub struct App {
    pub is_running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
        }
    }
}

impl App {
    pub async fn new() -> AppResult<Self> {
        Ok(App::default())
    }

    pub async fn run<B: Backend>(&mut self, mut tui: Tui<B>) -> AppResult<Tui<B>>{
        while self.is_running {
            tui.draw(self)?;

            match tui.events.next().await? {
                Event::Tick => self.tick(),
                Event::Key(key_event) => handle_key_events(key_event, self)?,
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            }
        };

        Ok(tui)
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.is_running = false;
    }
}
