use std::sync::Arc;
use tokio::time::{sleep, Duration};

use hvaclib::api;
use hvaclib::config;
use hvaclib::influx::{get_infux_db_client, write_data_with_point_name};
use hvaclib::onewire::Ds18b20;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        // set default to info if none is set already
        std::env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    print_version();

    let cfg = config::read_config();

    let influx_client: Arc<influx_db_client::Client> =
        Arc::new(get_infux_db_client(&cfg.influx).unwrap());

    let sensor_id = std::env::args().nth(1).expect("sensor ID required");
    let influx_id = cfg.influx.id.to_string();
    let point_name = cfg.measurement_points[0].clone().name;
    tokio::spawn(async move {
        let sensor = Ds18b20::new(sensor_id);
        match sensor {
            Ok(s) => loop {
                if let Ok(temp) = s.read_temp() {
                    let temp_celsius = temp.to_celsius();
                    let _ = write_data_with_point_name(
                        &influx_client,
                        &influx_id,
                        vec![(point_name.as_str(), temp_celsius.clone())],
                    )
                    .await;
                    log::info!("Temp: {} °C", temp_celsius)
                }
                sleep(Duration::from_secs(2)).await;
            },
            Err(e) => log::error!("{:?}", e),
        }
    });

    api::start_server(cfg).await;
}

fn print_version() {
    log::info!(target: "main", "VERSION is {}", env!("CARGO_PKG_VERSION"));
}
