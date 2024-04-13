use crate::app::{App, AppResult};
use crossterm::event::KeyEvent;
use crate::connection::get_temperature;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // TODO: define actions for quitting the app
        crossterm::event::KeyCode::Char('q') => {
            app.running = false;
        }
        // TODO: define actions for apps functionalities
        crossterm::event::KeyCode::Char('m') => {
            println!("hello rust workshop!");
        }

        crossterm::event::KeyCode::Down => {
            app.next_city();
        }

        crossterm::event::KeyCode::Up => {
            app.previous_city();
        }
        _ => {}
    }
    Ok(())
}
