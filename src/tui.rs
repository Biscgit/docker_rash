use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;

/// Representation of a terminal user interface.
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        Self::init_panic_hook()?;

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    /// initializes a custom panic hook
    pub fn init_panic_hook() -> AppResult<()> {
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    /// Resets the terminal interface.
    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
