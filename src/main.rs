use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use docker_rash::app::App;
use docker_rash::event::EventHandler;
use docker_rash::tui::Tui;
use docker_rash::types::AppResult;
use docker_rash::components::Component;

const TICK_RATE_MS: u64 = 100;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize the terminal
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_MS);
    let sender = events.get_sender();

    let mut tui = Tui::new(terminal, events);
    tui.init()?;


    // Create app and start
    let mut app = App::new(sender)?;
    app.run(tui).await?;

    Ok(())
}
