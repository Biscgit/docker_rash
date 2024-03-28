use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::components::Component;
use crate::event::Event;
use crate::types::AppResult;
use crate::components::background::{set_background, LIGHT_BACKGROUND};

pub struct Page {}

impl Component for Page{
    fn new(sender: UnboundedSender<Event>) -> AppResult<Self> where Self: Sized {
        Ok(Page{

        })
    }

    fn handle_key_events(&mut self, key: KeyEvent) {

    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        set_background(LIGHT_BACKGROUND, frame, area);
    }
}