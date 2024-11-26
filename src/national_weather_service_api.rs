pub mod grid_point;
pub mod weather_forecast_office_id;

pub use crate::national_weather_service_api::{
    grid_point::GridPoint, weather_forecast_office_id::WeatherForecastOfficeId,
};
use anyhow::{anyhow, Error};
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use std::{thread::sleep, time::Duration};
use ureq::{Agent, AgentBuilder};

const ROOT: &str = "https://api.weather.gov";
const WAIT_PERIOD: Duration = Duration::from_secs(1);

/// https://www.weather.gov/documentation/services-web-api
#[derive(Debug)]
pub struct NationalWeatherServiceApi {
    http_client: Agent,
}
impl NationalWeatherServiceApi {
    pub fn new() -> Result<Self, Error> {
        let http_client = AgentBuilder::new()
            .user_agent(&format!(
                "NationalWeatherServiceApi_Rust_Bindings_{}",
                now_string()
            ))
            .build();

        Ok(Self { http_client })
    }
}
impl NationalWeatherServiceApi {
    /// **GET** [/points/{latitude},{longitude}](https://www.weather.gov/documentation/services-web-api#/default/point)
    ///
    /// Returns metadata about a given latitude/longitude point. Will [sleep] for [WAIT_PERIOD] after request.
    pub fn points(&self, latitude: f64, longitude: f64) -> Result<JsonValue, Error> {
        let endpoint = format!("{ROOT}/points/{latitude:.4},{longitude:.4}");
        let response = self.http_client.get(&endpoint).call()?;

        if !(200..300).contains(&response.status()) {
            return Err(anyhow!("Failed to make request\n{:#?}", response));
        }

        let response_json: JsonValue = response.into_json()?;

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
        let response = self.http_client.get(&endpoint).call()?;

        if !(200..300).contains(&response.status()) {
            return Err(anyhow!("Failed to make request\n{:#?}", response));
        }

        let response_json: JsonValue = response.into_json()?;
        
        sleep(WAIT_PERIOD);

        Ok(response_json)
    }
}

pub fn now_string() -> String {
    DateTime::<Utc>::from(std::time::SystemTime::now()).to_rfc3339()
}
