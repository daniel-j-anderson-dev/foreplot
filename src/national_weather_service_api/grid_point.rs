use crate::national_weather_service_api::weather_forecast_office_id::WeatherForecastOfficeId;
use anyhow::{anyhow, Error};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPoint {
    id: WeatherForecastOfficeId,
    x: u64,
    y: u64,
}
impl GridPoint {
    pub const fn wfo(&self) -> WeatherForecastOfficeId {
        self.id
    }
    pub const fn x(&self) -> u64 {
        self.x
    }
    pub const fn y(&self) -> u64 {
        self.y
    }
}
impl TryFrom<JsonValue> for GridPoint {
    type Error = Error;
    /// `value` should come from [NationalWeatherServiceApi::points]
    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        use parse_grid_point_error_strings::*;

        let properties = value
            .get("properties")
            .ok_or(anyhow!("{PREFIX}{PROPERTIES_MISSING}{SUFFIX}"))?;

        let id = properties
            .get("gridId")
            .ok_or(anyhow!("{PREFIX}{ID_MISSING}{SUFFIX}"))?
            .as_str()
            .ok_or(anyhow!("{PREFIX}{ID_NOT_STRING}{SUFFIX}"))?
            .parse()?;

        let x = properties
            .get("gridX")
            .ok_or(anyhow!("{PREFIX}{X_MISSING}{SUFFIX}"))?
            .as_u64()
            .ok_or(anyhow!("{PREFIX}{X_NOT_NUMBER}{SUFFIX}"))?;

        let y = properties
            .get("gridY")
            .ok_or(anyhow!("{PREFIX}{Y_MISSING}{SUFFIX}"))?
            .as_u64()
            .ok_or(anyhow!("{PREFIX}{Y_NOT_NUMBER}{SUFFIX}"))?;

        Ok(GridPoint { id, x, y })
    }
}

mod parse_grid_point_error_strings {
    pub const PREFIX: &str = "Could not construct Gridpoint, ";
    pub const SUFFIX: &str = "\nDid this JsonValue come form https://api.weather.gov/points/{latitude},{longitude} endpoint?";
    pub const PROPERTIES_MISSING: &str = "\"properties\" field was missing.";
    pub const ID_MISSING: &str = "\"gridId\" field was missing.";
    pub const ID_NOT_STRING: &str = "\"gridId\" field was missing.";
    pub const X_MISSING: &str = "\"gridX\" field was missing.";
    pub const X_NOT_NUMBER: &str = "\"gridX\" was not a number.";
    pub const Y_MISSING: &str = "\"gridY\" field was missing.";
    pub const Y_NOT_NUMBER: &str = "\"gridY\" was not a number.";
}
