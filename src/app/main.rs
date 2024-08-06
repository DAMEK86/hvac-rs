use std::{thread::sleep, time::Duration};

use hvaclib::onewire::Ds18b20;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        // set default to info if none is set already
        std::env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let sensor_id = std::env::args().nth(1).expect("sensor ID required");
    let sensor = Ds18b20::new(sensor_id);
    match sensor {
        Ok(s) => loop {
            if let Ok(temp) = s.read_temp() {
                log::info!("Temp: {} °C", temp.to_celsius())
            }
            sleep(Duration::from_secs(2))
        },
        Err(e) => log::error!("{:?}", e),
    }
}
