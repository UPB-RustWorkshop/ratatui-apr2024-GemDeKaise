use std::collections::HashMap;
use std::error;
use ratatui::widgets::{ListState, StatefulWidget};
use crate::connection::{CityInfo, get_cities, get_temperature};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub cache: bool,
    pub cities: Vec<String>,
    pub cities_temperature: HashMap<String, CityInfo>,
    pub cities_state: ListState
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> Self {
        // let cities = vec!["Bucuresti", "Pitesti"];
        // get the cities from http://34.116.205.113:3000/cities get endpoint
        let cities = get_cities().await.unwrap();
        let mut cities_temperature = HashMap::new();


        let mut state = ListState::default();
        if !cities.is_empty() {
            state.select(Some(0));
        }


        // cache the first 3 cities and the last 3 cities
        for (index, city) in cities.iter().enumerate() {
            if index < 5 || index >= cities.len() - 5 {
                let city_info = get_temperature(city.to_string()).await.unwrap();
                cities_temperature.insert(city.to_string(), city_info);
            }
        }

        Self {
            running: true,
            cities: cities.iter().map(|city| city.to_string()).collect(),
            cities_temperature,
            cities_state: state,
            cache: false
        }
    }

    pub fn get_selected_city(&self) -> Option<&String> {
        self.cities.get(self.cities_state.selected().unwrap())
    }

    pub fn get_update_cities(&self) -> Vec<String> {
        let selected = self.cities_state.selected().unwrap();
        let mut update_cities = vec![];
        for i in selected..selected + 5 {
            if i < self.cities.len() {
                update_cities.push(self.cities[i].clone());
            } else {
                update_cities.push(self.cities[i - self.cities.len()].clone());
            }
        }

        let size = selected as i32 - 5;
        for i in size..selected as i32 {
            if i >= 0 {
                update_cities.push(self.cities[i as usize].clone());
            } else {
                update_cities.push(self.cities[(self.cities.len() as i32 + i) as usize].clone());
            }
        }

        update_cities
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
        self.cache = false;
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
        self.cache = false;
    }
}
