mod influx_client;
mod sensor;

extern crate docopt;
extern crate reqwest;

use docopt::Docopt;

const USAGE: &'static str = "
temperature

Usage:
	temperature (PATH)
	temperature (-h | --help)
	temperature (-v | --version)

Options:
	PATH		The path to the device to read sensor data from.
	-h --help	Show this screen.
	-v --version	Show version.
";

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    let path = args.get_str("PATH");

    // Load the data from the file
    let data: String = match sensor::read_sensor_data(path) {
        Ok(data) => data,
        Err(error) => {
            return eprintln!(
                "Failed to read sensor data from file: {}\n{:?}",
                path, error
            )
        }
    };

    // Now let's parse that data and attempt to extract the temperature
    let s_temp = match sensor::parse_sensor_data(&data) {
        Some(data) => data,
        None => return eprintln!("Could not parse sensor data"),
    };

    // See if we can convert it to a number
    let f_temp = sensor::convert_temperature(s_temp);

    // Print it
    println!("Temperature: {} ℃", f_temp);

    // Test client
    influx_client::add_temperature_to_db(f_temp).unwrap();
}
