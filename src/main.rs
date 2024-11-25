mod data_set;
mod national_weather_service_api;
mod plotting;

use crate::{
    data_set::parse_temperatures,
    national_weather_service_api::{GridPoint, NationalWeatherServiceApi},
    plotting::{plot, PlotArgs},
};
use anyhow::Error;
use clap::Parser;
use national_weather_service_api::now_string;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CommandLineArguments {
    latitude: f64,
    longitude: f64,
    image_path: Option<String>,
}

fn main() -> Result<(), Error> {
    // parse command line arguments
    let CommandLineArguments {
        latitude,
        longitude,
        image_path,
    } = CommandLineArguments::parse();

    // create a client for interacting with the national weather service api
    let nws_api = NationalWeatherServiceApi::new()?;

    // make a GET request to the /points endpoint to get meta data about this (lat, long)
    let lat_long_metadata = nws_api.points(latitude, longitude)?;

    // parse the json data into a GridPoint
    let gridpoint = GridPoint::try_from(lat_long_metadata)?;

    // make GET request to the /gridpoints endpoint for raw forecast data
    let raw_forecast_data = nws_api.gridpoints(gridpoint.wfo(), gridpoint.x(), gridpoint.y())?;

    // parse the temperatures from the raw forecast data
    let data_set = parse_temperatures(raw_forecast_data)?;

    // use the default image string if one was not provided
    let image_path = image_path.unwrap_or_else(|| {
        format!(
            "./temperature_chart_{:.4}_{:.4}_{}.png",
            latitude,
            longitude,
            now_string()
        )
    });

    // create the plot
    plot(
        data_set,
        PlotArgs {
            image_path,
            ..Default::default()
        },
    )?;

    Ok(())
}
