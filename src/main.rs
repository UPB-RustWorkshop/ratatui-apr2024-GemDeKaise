use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // TODO:  the terminal user interface
    let mut tui = Tui::new(terminal, EventHandler::new(10));
    
    // TODO: init the terminal
    tui.init()?;

    // Start the main loop.
    // while app.running {
        // TODO: Render the user interface.

        // TODO: Handle events.
        
    // }

    while app.running {
        // Render the user interface.
        tui.draw(&mut app);

        // Handle events.
        if let event = tui.events.next().await? {

            match event {
                Event::Key(key_event) => {
                    handle_key_events(key_event, &mut app)?;
                }
                _ => {}
            }
        }
    }

    // TODO: Reset the terminal if the app has been terminated
    tui.exit()?;

    Ok(())
}
