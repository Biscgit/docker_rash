pub mod sidebar;
pub mod background;
pub mod page;

use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use ratatui::backend::Backend;
use tokio::sync::mpsc::UnboundedSender;
use crate::app::Action;
use crate::event::Event;
use crate::types::AppResult;

pub trait Component {
    fn new(sender: UnboundedSender<Event>) -> AppResult<Self> where Self: Sized;

    fn handle_events(&mut self, event: Action) {
        match event {
            Action::Key(key_event) => { self.handle_key_events(key_event); },
            _ => {}
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent);

    fn render(&mut self, frame: &mut Frame, area: Rect);
}