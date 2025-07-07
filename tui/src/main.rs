use swoop_core::fetch_url;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple fetch to warm up the HTTP client (optional, demonstrates core crate wiring)
    let _ = fetch_url("https://httpbin.org/ip").await?;

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Draw a single frame with a greeting block.
    terminal.draw(|f| {
        let size = f.area();
        let block = Block::default().title("Swoop TUI").borders(Borders::ALL);
        let paragraph = Paragraph::new("Hello, Swoop!").block(block);
        f.render_widget(paragraph, size);
    })?;

    // Wait for a single key press before exiting.
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
