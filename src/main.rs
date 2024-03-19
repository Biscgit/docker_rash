use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use docker_rash::app::{App, AppResult};
use docker_rash::event::EventHandler;
use docker_rash::tui::Tui;

const TICK_RATE_MS: u64 = 250;


#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize the terminal
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_MS);

    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Create app and start
    let mut app = App::new().await?;
    tui = app.run(tui).await?;

    // Exit after done
    tui.exit()?;
    Ok(())
}
