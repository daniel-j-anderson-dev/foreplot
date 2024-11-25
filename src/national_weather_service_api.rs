pub mod grid_point;
pub mod weather_forecast_office_id;

pub use crate::national_weather_service_api::{
    grid_point::GridPoint, weather_forecast_office_id::WeatherForecastOfficeId,
};
use anyhow::{anyhow, Error};
use chrono::{DateTime, Utc};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use serde_json::Value as JsonValue;
use std::{thread::sleep, time::Duration};

const ROOT: &str = "https://api.weather.gov";
const WAIT_PERIOD: Duration = Duration::from_secs(1);

/// https://www.weather.gov/documentation/services-web-api
#[derive(Debug)]
pub struct NationalWeatherServiceApi {
    http_client: Client,
}
impl NationalWeatherServiceApi {
    pub fn new() -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(
                format!("NationalWeatherServiceApi_Rust_Bindings_{}", now_string()).as_str(),
            )?,
        );

        let http_client = Client::builder().default_headers(headers).build()?;

        Ok(Self { http_client })
    }
}
impl NationalWeatherServiceApi {
    /// **GET** [/points/{latitude},{longitude}](https://www.weather.gov/documentation/services-web-api#/default/point)
    ///
    /// Returns metadata about a given latitude/longitude point. Will [sleep] for [WAIT_PERIOD] after request.
    pub fn points(&self, latitude: f64, longitude: f64) -> Result<JsonValue, Error> {
        let endpoint = format!("{ROOT}/points/{latitude:.4},{longitude:.4}");
        let response = self.http_client.get(endpoint).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to make request\n{:#?}", response));
        }

        let response_json: JsonValue = response.json()?;

        sleep(WAIT_PERIOD);

        Ok(response_json)
    }

    /// **GET** [/gridpoints/{wfo}/{x},{y}](https://www.weather.gov/documentation/services-web-api#/default/gridpoint)
    ///
    /// Returns raw numerical forecast data for a 2.5km grid area. Will [sleep] for [WAIT_PERIOD] after request.
    pub fn gridpoints(
        &self,
        wfo: WeatherForecastOfficeId,
        x: u64,
        y: u64,
    ) -> Result<JsonValue, Error> {
        let endpoint = format!("{ROOT}/gridpoints/{wfo}/{x},{y}");
        let response = self.http_client.get(endpoint).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to make request\n{:#?}", response));
        }

        let response_json: JsonValue = response.json()?;
        sleep(WAIT_PERIOD);

        Ok(response_json)
    }
}

pub fn now_string() -> String {
    DateTime::<Utc>::from(std::time::SystemTime::now()).to_rfc3339()
}
