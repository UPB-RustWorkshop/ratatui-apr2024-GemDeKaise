use reqwest::header;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct CityInfo {
    pub temperature: f64,
    pub weather: String,
    pub feels_like: f64,
    pub humidity: f64,
    pub wind_speed: f64,

    pub future_temperatures: Vec<f64>,
}
pub async fn get_cities() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let request = client.request(reqwest::Method::GET, "http://34.116.205.113:3000/cities")
        .header(header::CONTENT_TYPE, "application/json");

    let response = request.send().await?;
    let body = response.text().await?;

    let body: serde_json::Value = serde_json::from_str(&body)?;

    let cities = body["cities"].as_array().unwrap();

    let cities: Vec<String> = cities.iter().map(|city| city.as_str().unwrap().to_string()).collect();

    Ok(cities)
}

pub async fn get_temperature(city: String) -> Result<CityInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let data = format!("{{\"city\": \"{}\"}}", city);

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let request1 = client.request(reqwest::Method::POST, "http://34.116.205.113:3000/cities/current_weather")
        .headers(headers.clone())
        .json(&json);

    let request2 = client.request(reqwest::Method::POST, "http://34.116.205.113:3000/cities/forecast")
        .headers(headers)
        .json(&json);

    let response1 = request1.send().await?;
    let response2 = request2.send().await?;
    let body = response1.text().await?;

    let body: serde_json::Value = serde_json::from_str(&body)?;

    let temperature = body["conditions"]["temp"].as_f64().unwrap();
    let weather = body["weather"]["main_weather"].as_str().unwrap();
    let feels_like = body["conditions"]["feels_like"].as_f64().unwrap();
    let humidity = body["conditions"]["humidity"].as_f64().unwrap();
    let wind_speed = body["conditions"]["wind"]["speed"].as_f64().unwrap();

    let body = response2.json::<serde_json::Value>().await?;

    let forecast = body["forecast"].as_array().unwrap().iter().map(|day| day["conditions"]["temp"].as_f64().unwrap()).collect();

    Ok(CityInfo {
        temperature: temperature,
        weather: weather.to_string(),
        feels_like: feels_like,
        humidity: humidity,
        wind_speed: wind_speed,
        future_temperatures: forecast,
    })
}
