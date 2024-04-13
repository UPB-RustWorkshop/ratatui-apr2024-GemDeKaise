use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::Terminal;
use ratatui_templates::connection::get_temperature;


#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new().await;

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal, EventHandler::new(100));
    tui.init()?;

    while app.running {
        // Render the user interface.
        tui.draw(&mut app);

        if app.cache == false {
            for (index, city) in app.cities.iter().enumerate() {
                if index < app.cities_state.selected().unwrap() + 5 || index >= app.cities.len() - 5{
                    if !app.cities_temperature.contains_key(city) {
                        let city_info = get_temperature(city.to_string()).await.unwrap();
                        app.cities_temperature.insert(city.to_string(), city_info);
                    }
                }
            }
            app.cache = true;
        }

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

    tui.exit()?;
    Ok(())
}
