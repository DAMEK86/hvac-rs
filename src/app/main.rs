use tokio::time::{sleep, Duration};

use hvaclib::onewire::Ds18b20;
use hvaclib::config;
use hvaclib::api;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        // set default to info if none is set already
        std::env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    print_version();

    let config = config::read_config();

    let sensor_id = std::env::args().nth(1).expect("sensor ID required");
    tokio::spawn(async move {
        let sensor = Ds18b20::new(sensor_id);
        match sensor {
            Ok(s) => loop {
                    if let Ok(temp) = s.read_temp() {
                        log::info!("Temp: {} °C", temp.to_celsius())
                    }
                    sleep(Duration::from_secs(2)).await;
            },
            Err(e) => log::error!("{:?}", e),
        }
    });

    api::start_server(config).await;
}

fn print_version() {
    log::info!(target: "main", "VERSION is {}", env!("CARGO_PKG_VERSION"));
}