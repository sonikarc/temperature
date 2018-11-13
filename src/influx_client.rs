use reqwest;
use reqwest::Error; 
use sensor::{Temperature, SensorError};


const INFLUX_HOST: &'static str = "localhost";
const INFLUX_PORT: &'static str = "8086";
const INFLUX_DB: &'static str = "write?db=temperature";

const MEASUREMENT: &'static str = "temperature";

const TAG_COLLECTOR_KEY: &'static str = "collector";
const TAG_COLLECTOR_VALUE: &'static str = "raspberrypi-1";

impl From<Error> for SensorError {
	fn from(error: Error) -> SensorError {
		SensorError::CouldNotSubmitToDb
	}
}

pub fn add_temperature_to_db(temp: Temperature) -> Result<(), SensorError> {
	let url = format!("http://{}:{}/{}", INFLUX_HOST, INFLUX_PORT, INFLUX_DB);
	let insert = format!("{},{}={} value={}", MEASUREMENT, TAG_COLLECTOR_KEY, TAG_COLLECTOR_VALUE, temp);

	let client = reqwest::Client::new();
	let response = client.post(&url).body(insert).send()?;

	Ok(())
} 
