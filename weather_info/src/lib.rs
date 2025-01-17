use anyhow::{Context, Ok};
use open_meteo_rs::forecast::ForecastResult;
use std::{error::Error, ffi::CString, os::raw::c_char};
use tokio::runtime::Runtime;

extern crate open_meteo_rs;

#[derive(Debug)]
#[repr(C)]
pub struct GPScoordinates {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug)]
#[repr(C)]
pub struct CurrentWeather {
    pub temp_unit: *const c_char,
    pub temp_value: f32,
}

 fn get_forecast_result(gps_coordinates: &GPScoordinates) -> Result<ForecastResult, Box<dyn Error>> {
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();

    (opts.location.lat, opts.location.lng) = (gps_coordinates.latitude, gps_coordinates.longitude);

    opts.current.push("temperature_2m".into());

    //Run async function in a dedicated runtime and block thread until complete
    Runtime::new()
        .unwrap()
        .block_on(client.forecast(opts))
}

fn get_current_temperature(gps_coordinates: GPScoordinates) -> Result<CurrentWeather, anyhow::Error> {
    let forecast_current = 
        get_forecast_result(&gps_coordinates)
            .unwrap()
            .current
            .with_context(|| format!("forecast current failed for gps: {gps_coordinates:#?}"))?;

    let forecast_temp = 
        forecast_current
            .values
            .get("temperature_2m")
            .with_context(|| "get temperature failed")?
        ;

    let temp_unit = CString::new(
        forecast_temp.unit
            .clone()
            .with_context(|| "temperature unit ref failed")?
    )?;

    let temperature = CurrentWeather {
        temp_unit: temp_unit.as_ptr(),
        temp_value: forecast_temp.value.to_string().parse::<f32>()
            .with_context(|| "parsing temperature failed")?,
    };

    //do not free up the string memory
    std::mem::forget(temp_unit);

    Ok(temperature)
}

#[no_mangle]
pub extern "C" fn get_current_temperature_c (gps_coordinates: GPScoordinates) -> CurrentWeather {
    get_current_temperature(gps_coordinates).unwrap()
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    #[test]
    fn it_works() {
        let gps = GPScoordinates {
            latitude: 51.76,
            longitude: 19.65,
        };
        let result =  get_current_temperature(gps).unwrap();
        println!("[result] {:#?}", result);
        let cstr = unsafe {CStr::from_ptr(result.temp_unit).to_str().unwrap()};
        assert!(cstr.contains("°C"), "[temp_unit] = {}", cstr);
        assert!(result.temp_value > -50.0 && result.temp_value < 50.0, "[temp_value] = {}", result.temp_value);
    }
}
