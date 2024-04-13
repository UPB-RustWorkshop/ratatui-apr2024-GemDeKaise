use std::collections::HashMap;
use std::error;
use ratatui::widgets::{ListState, StatefulWidget};
use crate::connection::get_temperature;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub cities: Vec<String>,
    pub cities_temperature: HashMap<String, f64>,
    pub cities_state: ListState
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> Self {
        let cities = vec!["Bucuresti", "Pitesti"];
        let mut cities_temperature = HashMap::new();

        for (index, city) in cities.iter().enumerate() {
            let city_info = get_temperature(city.to_string()).await.unwrap();
            cities_temperature.insert(city.to_string(), city_info.temperature);
        }

        let mut state = ListState::default();
        if !cities.is_empty() {
            state.select(Some(0));
        }

        Self {
            running: true,
            cities: cities.iter().map(|city| city.to_string()).collect(),
            cities_temperature,
            cities_state: state
        }
    }

    pub fn get_selected_city(&self) -> Option<&String> {
        self.cities.get(self.cities_state.selected().unwrap())
    }

    pub fn next_city(&mut self) {
        let i = match self.cities_state.selected() {
            Some(i) => {
                if i >= self.cities.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.cities_state.select(Some(i));
    }

    pub fn previous_city(&mut self) {
        let i = match self.cities_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.cities.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.cities_state.select(Some(i));
    }
}
