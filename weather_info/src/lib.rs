use anyhow::Context;
//use std::result::Result::Ok;
use open_meteo_rs::forecast::ForecastResult;
use std::{error::Error, ffi::CString, os::raw::c_char, ptr::null};
use tokio::runtime::Runtime;

extern crate open_meteo_rs;

#[derive(Debug)]
#[repr(C)]
pub struct GPScoordinates {
    pub longitude: f64,
    pub latitude: f64,
}

/*
fn parse_coordinates (str: *const c_char) -> GPScoordinates {

}
*/

#[derive(Debug)]
#[repr(C)]
pub struct CurrentWeather {
    pub error_flg: bool,
    pub error_msg: *const c_char,
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

fn get_current_temperature(gps_coordinates: GPScoordinates) -> Result<*mut CurrentWeather, Box<dyn Error>> {
    let forecast_current =
        get_forecast_result(&gps_coordinates)?
            .current
            .with_context(|| format!("forecast current failed for gps: {gps_coordinates:#?}"))?
        ;

    let forecast_temp = 
        forecast_current
            .values
            .get("temperature_2m")
            .with_context(|| "get temperature failed")?
        ;

    let temp_unit = CString::new(
        forecast_temp.unit
            .clone()
            .with_context(|| "get temperature unit failed")?
    )?;

    //let temp_unit_ptr = temp_unit.as_ptr();

    Ok(Box::into_raw(Box::new(CurrentWeather{
        //temp_unit: temp_unit.as_ptr(),
        //temp_unit: temp_unit_ptr,
        error_flg: false,
        error_msg: null(),
        temp_unit: temp_unit.into_raw(),
        temp_value: forecast_temp.value.to_string().parse::<f32>()
            .with_context(|| "parsing temperature value failed")?,
    })))

    //do not free up the string memory
    //std::mem::forget(temp_unit_ptr);

}

#[no_mangle]
pub extern "C" fn get_current_temperature_c (gps_coordinates: GPScoordinates) -> *mut CurrentWeather {
    //get_current_temperature(gps_coordinates).unwrap()
    match get_current_temperature(gps_coordinates) {
        Ok(res_ok) => {
            res_ok
        },
        Err(err) => {
            Box::into_raw(Box::new(
            CurrentWeather {
                error_flg : true,
                error_msg : CString::new(format!("{:#?}", err)).unwrap().into_raw(),
                temp_unit : null(),
                temp_value : 0.0,
                } 
            ))
        },
    } 
}

#[no_mangle]
pub extern "C" fn get_current_temperature_c_free (ptr: *mut CurrentWeather) {
    if ptr.is_null() {
        return;
    }
    else {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    #[test]
    fn good_coordinates() {
        let gps = GPScoordinates {
            latitude: 51.76,
            longitude: 19.65,
        };
  //      let result =  get_current_temperature(gps).unwrap();
        let res = get_current_temperature_c(gps); 
        unsafe {
        let result =  Box::from_raw(res);
        println!("[result] {:#?}", result);
        //let cstr = unsafe {CStr::from_ptr(result.temp_unit).to_str().unwrap()};
        assert!(!result.error_flg);
        let cstr = CStr::from_ptr(result.temp_unit).to_str().unwrap();
        assert!(cstr.contains("°C"), "[temp_unit] = {}", cstr);
        assert!(result.temp_value > -50.0 && result.temp_value < 50.0, "[temp_value] = {}", result.temp_value);
        };
    }

    #[test]
    fn wrong_coordinates() {
        let gps = GPScoordinates {
            latitude: 100.0,
            longitude: -100.0,
        };
        let res = get_current_temperature_c(gps); 
        unsafe {
        let result =  Box::from_raw(res);
        println!("[result] {:#?}", result);
        assert!(result.error_flg);
        let cstr = CStr::from_ptr(result.error_msg).to_str().unwrap();
        println!("[error message]:\n{}\n", cstr);
        };
    }

}
