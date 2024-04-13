use futures::sink::Buffer;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};
use crate::app::App;
use crate::connection::get_temperature;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);


    let cities: Vec<ListItem> =
        app.cities.iter().map(|city| ListItem::new(city.clone())).collect();


    frame.render_stateful_widget(
        List::new(cities)
            .block(
                Block::default()
                    .title("Cities")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White)),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray),
            )
            .highlight_symbol(">> "),
        layout[0],
        &mut app.cities_state,
    );

    let weather_info = List::new(app.cities.iter().map(|city| {
        ListItem::new(format!("{}: {}°C", city, app.cities_temperature.get(city).unwrap()))
    }));

    // based on the selected city, display the weather info
    if let Some(selected_city) = app.get_selected_city() {
        let temp = app.cities_temperature.get(selected_city).unwrap();
        let weather_info = ListItem::new(format!("{}: {}°C", selected_city, temp));
        frame.render_widget(
            List::new(vec![weather_info])
                .block(
                    Block::default()
                        .title("Weather Info")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White)),
                )
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::DarkGray),
                )
                .highlight_symbol(">> "),
            layout[1],
        );
    }


}
