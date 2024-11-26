use anyhow::{anyhow, Error};
use chrono::prelude::*;
use serde_json::Value as JsonValue;

fn celsius_to_fahrenheit(x: f64) -> f64 {
    x * (9.0 / 5.0) + 32.0
}

/// To be used in conjunction with [crate::national_weather_service_api::NationalWeatherServiceApi::gridpoints]
///
/// # Returns
/// A [Vec] containing all of the [DateTime]s and temperatures(°F) from the `raw_forecast_data` json
pub fn parse_temperatures(
    raw_forecast_data: JsonValue,
) -> Result<Vec<(DateTime<Utc>, f64)>, Error> {
    use parse_dataset_error_strings::*;

    raw_forecast_data
        .get("properties")
        .ok_or(anyhow!("{PREFIX}{PROPERTIES_MISSING}{SUFFIX}"))?
        .get("temperature")
        .ok_or(anyhow!("{PREFIX}{TEMPERATURE_MISSING}{SUFFIX}"))?
        .get("values")
        .ok_or(anyhow!("{PREFIX}{VALUES_MISSING}{SUFFIX}"))?
        .as_array()
        .ok_or(anyhow!("{PREFIX}{VALUES_NOT_ARRAY}{SUFFIX}"))?
        .into_iter()
        .enumerate()
        .map(|(i, value)| {
            let date_time = value
                .get("validTime")
                .ok_or(anyhow!("{PREFIX}{VALID_TIME_MISSING}{i}{SUFFIX}"))?
                .as_str()
                .ok_or(anyhow!("{PREFIX}{VALID_TIME_NOT_STRING}{i}{SUFFIX}"))?
                .split_once('/')
                .expect("validTime field is a duration. see https://en.wikipedia.org/wiki/ISO_8601#Time_intervals").0
                .parse()?;

            let celsius = value
                .get("value")
                .ok_or(anyhow!("{PREFIX}{VALUE_MISSING}{i}{SUFFIX}"))?
                .as_f64()
                .ok_or(anyhow!("{PREFIX}{VALUE_NOT_NUMBER}{i}{SUFFIX}"))?;
            let fahrenheit = celsius_to_fahrenheit(celsius);

            Ok((date_time, fahrenheit))
        })
        .collect::<Result<Vec<_>, _>>()
}

mod parse_dataset_error_strings {
    pub const PREFIX: &str = "Could not parse dataset";
    pub const SUFFIX: &str =
        "\nDid this JsonValue come form https://api.weather.gov/gridpoints/{wfo}/{x},{y} endpoint?";
    pub const PROPERTIES_MISSING: &str = "\"properties\" field was missing.";
    pub const TEMPERATURE_MISSING: &str = "\"properties\".\"temperature\" field was missing.";
    pub const VALUES_MISSING: &str = "\"properties\".\"temperature\".\"values\" field was missing.";
    pub const VALUES_NOT_ARRAY: &str =
        "\"properties\".\"temperature\".\"values\" was not an array.";
    pub const VALID_TIME_MISSING: &str =
        "in \"properties\".\"temperature\".\"values\" array no \"validTime\" field at index ";
    pub const VALID_TIME_NOT_STRING: &str =
        "in \"properties\".\"temperature\".\"values\" array \"validTime\" was not a string at index ";
    pub const VALUE_MISSING: &str =
        "in \"properties\".\"temperature\".\"values\" array no \"value\" field at index ";
    pub const VALUE_NOT_NUMBER: &str =
        "in \"properties\".\"temperature\".\"values\" array \"value\" was not a number at index ";
}
