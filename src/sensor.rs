use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub type Temperature = f64;

#[derive(Debug)]
pub enum SensorError {
    CouldNotReadSensorData(ErrorKind),
    CouldNotSubmitToDb,
}

impl From<Error> for SensorError {
    fn from(error: Error) -> SensorError {
        SensorError::CouldNotReadSensorData(error.kind())
    }
}

///
/// Converts temperature read from the device to a more human readable one.
///
pub fn convert_temperature(temp: &str) -> Temperature {
    let f_temp: f64 = temp.parse().unwrap_or(0.0);

    f_temp / 1000.0
}

///
/// Parses data provided from the device
///
pub fn parse_sensor_data(data: &str) -> Option<&str> {
    // Split at the '=' and return the second string slice
    let split_data: Vec<&str> = data.split('=').collect();

    match split_data.get(2) {
        Some(temperature) => Some(temperature.trim()),
        _ => None,
    }
}

///
/// Reads data from the device
///
pub fn read_sensor_data(path: &str) -> Result<String, SensorError> {
    let mut f = File::open(path)?;
    let mut data = String::new();

    f.read_to_string(&mut data)?;

    Ok(data)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_can_convert_a_temperature() {
        // Pretty much just tests the input and output, this should never fail
        assert_eq!(0.0, convert_temperature("0"));
        assert_eq!(0.0, convert_temperature("0000"));
        assert_eq!(10.0, convert_temperature("10000"));
        assert_eq!(12.345, convert_temperature("12345"));

        // Now let's test if we pass an empty string, default to 0
        assert_eq!(0.0, convert_temperature(""));
    }

    #[test]
    fn it_can_parse_data() {
        // create some valid stub data
        let stub_data =
            "98 01 4b 46 7f ff 0c 10 19 : crc=19 YES\n98 01 4b 46 7f ff 0c 10 19 t=25500\n";
        assert_eq!(Some("25500"), parse_sensor_data(stub_data));

        // create some technically valid stub data but one that makes little sense (hopefully)
        let stub_data = "98 01 4b 46 7f ff 0c 10 19 : crc=19 YES\n98 01 4b 46 7f ff 0c 10 19 t=0\n";
        assert_eq!(Some("0"), parse_sensor_data(stub_data));

        // create empty data (there's nothing int the file or it can't be parsed
        let stub_data = "";
        assert_eq!(None, parse_sensor_data(stub_data));
    }

}
