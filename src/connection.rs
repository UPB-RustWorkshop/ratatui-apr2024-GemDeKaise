use reqwest::header;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Deserialize)]
struct ApiResponse {
    temperature: f64,
}

#[derive(Debug)]
pub struct CityInfo {
    pub(crate) temperature: f64,
}

pub async fn get_temperature(city: String) -> Result<CityInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let data = format!("{{\"city\": \"{}\"}}", city);

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let request = client.request(reqwest::Method::POST, "http://34.116.205.113:3000/cities/current_weather")
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;

    let body: serde_json::Value = serde_json::from_str(&body)?;

    let temperature = body["conditions"]["temp"].as_f64().unwrap();

    Ok(CityInfo {
        temperature: temperature,
    })
}
