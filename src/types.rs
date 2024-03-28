use std::error;
use std::io::Stderr;
use ratatui::backend::CrosstermBackend;

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;
pub type Backend = CrosstermBackend<Stderr>;
pub type Terminal = ratatui::terminal::Terminal<Backend>;