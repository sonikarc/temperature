mod sensor;
mod influx_client;

#[macro_use]
extern crate clap;
extern crate reqwest;

use clap::App;


fn main() {
    let yaml = load_yaml!("cli_options.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // unwrap is safe here because PATH is required
    let path = matches.value_of("PATH").unwrap();

    // Load the data from the file
    let data: String = match sensor::read_sensor_data(path) {
        Ok(data) => data,
        Err(error) => return eprintln!("Failed to read sensor data from file: {}\n{:?}", path, error)
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
