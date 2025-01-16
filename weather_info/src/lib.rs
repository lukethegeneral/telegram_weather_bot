use anyhow::Context;
use open_meteo_rs::forecast::ForecastResult;
use std::error::Error;

extern crate open_meteo_rs;

#[derive(Debug)]
pub struct GPScoordinates {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug)]
pub struct CurrentTemperature {
    pub unit: String,
    pub value: f32,
}

async fn get_forecast_result(gps_coordinates: &GPScoordinates) -> Result<ForecastResult, Box<dyn Error>> {
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();

    (opts.location.lat, opts.location.lng) = (gps_coordinates.latitude, gps_coordinates.longitude);

    opts.current.push("temperature_2m".into());

    client.forecast(opts).await
}

pub async fn get_current_temperature(gps_coordinates: GPScoordinates) -> Result<CurrentTemperature, Box<dyn Error>> {
    let forecast_current = 
        get_forecast_result(&gps_coordinates)
            .await?
            .current
            .with_context(|| format!("forecast current failed for gps: {gps_coordinates:#?}"))?;

    let forecast_temp = 
        forecast_current
            .values
            .get("temperature_2m")
            .with_context(|| "get temperature failed")?
        ;

    let temperature = CurrentTemperature {
        unit: forecast_temp.unit.clone()
            .with_context(|| "temperature unit cloning failed")?,
        value: forecast_temp.value.to_string().parse::<f32>()
            .with_context(|| "parsing temperature failed")?,
    };

    Ok(temperature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let gps = GPScoordinates {
            latitude: 51.76,
            longitude: 19.65,
        };
        let result =  get_current_temperature(gps).await.unwrap();
        println!("[result] {:#?}", result);
        assert!(true);
    }
}
