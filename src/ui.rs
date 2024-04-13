use futures::sink::Buffer;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::{prelude::*, widgets::*};
use crate::app::App;
use crate::connection::{CityInfo, get_temperature};





pub fn render_city_info(frame: &mut Frame, layout: Rect, city_info: &CityInfo) {
    let sun = "\n
       \\  |  /
      ---.*.---
   -==( SUNNY )==-
      ---'*'---
       /  |  \\
    ";
    let cloud = "
         .-~~~-.
  .- ~ ~-(       )_ _
 /                    ~ -.
|                          ',
 \\                         .'
   ~- ._ ,. ,.,.,., ,.. -~
    ";
    // Define styles
    let title_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let text_style = Style::default().fg(Color::White);
    let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

    // Create a paragraph for the primary city info
    let temperature_text = format!("Temperature: {:.1}°C\nFeels Like: {:.1}°C\nWeather: {}\n Humidity: {:.0}%\nWind Speed: {:.1} m/s\n {}",
                                   city_info.temperature, city_info.feels_like, city_info.weather, city_info.humidity, city_info.wind_speed, (if city_info.weather == "Clear" { sun } else {cloud}));
    let weather_paragraph = Paragraph::new(temperature_text)
        .block(Block::default().title("Current Weather").borders(Borders::ALL).title_style(title_style))
        .style(text_style);





  // future temperatures is a vec
    let temperatures: Vec<(f64, f64)> = city_info.future_temperatures
        .iter()
        .enumerate()
        .map(|(i, &temp)| (i as f64, temp))
        .collect();

    let dataset = vec![Dataset::default()
        .marker(Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&temperatures)
        .graph_type(GraphType::Line)];

    let chart = Chart::new(dataset)
        .block(Block::default().title("Future Temperatures").borders(Borders::ALL).title_style(title_style))
        .x_axis(Axis::default().
            style(header_style).
            bounds([0.0, 40.0])
            .title("Hours")
            .labels(
                vec![
                    Span::styled("0", header_style),
                    Span::styled("10", header_style),
                    Span::styled("20", header_style),
                    Span::styled("30", header_style),
                    Span::styled("40", header_style),
                ]
            )
        )
        .y_axis(Axis::default()
            .style(header_style)
            .bounds([0.0, 45.0])
            .title("Temperature")
            .labels(
                vec![
                    Span::styled("0", header_style),
                    Span::styled("5", header_style),
                    Span::styled("15", header_style),
                    Span::styled("25", header_style),
                    Span::styled("45", header_style),
                ]
            )
        );


    // convert vec<f64> to [u64]
    let temps = city_info.future_temperatures.iter().map(|&x| x as u64).collect::<Vec<u64>>();


    let sparkline = Sparkline::default()
        .block(Block::default()
            .title("Future Temperatures")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .title_style(title_style)
        )
        .data(&temps)
        .style(Style::default().fg(Color::Cyan))
        .max(45);


    // Split the layout into two sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // For the paragraph
            // for a chart
            Constraint::Percentage(50), // For the table

        ])
        .split(layout);

    let chart_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[0]);


    // Render the widgets
    frame.render_widget(weather_paragraph, chunks[0]);
    frame.render_widget(sparkline, chart_layout[1]);
    frame.render_widget(chart, chunks[1]);
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // split layout[0] intro 90 10 to add help for the commands (key_down and q to close )
    let new_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout[0]);

    // create help text
    let help_text = Paragraph::new("Use arrow keys to navigate, 'q' to quit")
        .block(Block::default().title("Help").borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(help_text, new_layout[1]);

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
        new_layout[0],
        &mut app.cities_state,
    );

    // based on the selected city, display the weather info
    if let Some(selected_city) = app.get_selected_city() {
        let city_info = app.cities_temperature.get(selected_city);

        if let Some(city_info) = city_info {
            render_city_info(frame, layout[1], city_info);
        } else {
            let loading = Paragraph::new("Loading...")
                .block(Block::default().title("Loading").borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            frame.render_widget(loading, layout[1]);
        }
    }


}
