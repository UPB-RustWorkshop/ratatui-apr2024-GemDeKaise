use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui_templates::connection::get_temperature;


use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[tokio::main]
async fn main() -> AppResult<()> {
    let app = Arc::new(AsyncMutex::new(App::new().await));

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal, EventHandler::new(100));
    tui.init()?;

    let app_clone = Arc::clone(&app);
    tokio::spawn(async move {
        loop {
           if !app_clone.lock().await.cache {
                let next_5_cities = app_clone.lock().await.get_update_cities();
                for city in next_5_cities {
                    let city_info = get_temperature(city.to_string()).await.unwrap();
                    app_clone.lock().await.cities_temperature.insert(city.to_string(), city_info);
                }
           }
        }
    });

    while app.lock().await.running {
        let mut app_lock = app.lock().await;
        tui.draw(&mut *app_lock);

        if let event = tui.events.next().await? {
            match event {
                Event::Key(key_event) => {
                    handle_key_events(key_event, &mut *app_lock)?;
                }
                _ => {}
            }
        }
    }

    tui.exit()?;
    Ok(())
}