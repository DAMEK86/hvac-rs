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

    let influx_id = cfg.influx.id.to_string();
    let delta_t_sec = cfg.influx.delta_t_sec;

    let mut sensors: Vec<Ds18b20> = Vec::new();
    for measurement_point in &cfg.measurement_points {
        sensors.push(
            Ds18b20::new(
                measurement_point.sensor_id.clone(),
                measurement_point.name.clone(),
            )
            .unwrap(),
        )
    }

    tokio::spawn(async move {
        loop {
            let mut data: Vec<(&str, f64)> = Vec::new();
            for s in &sensors {
                if let Ok(temp) = s.read_temp() {
                    let temp_celsius = temp.to_celsius();
                    data.push((s.name.as_str(), temp_celsius));
                    log::info!("{}: {} °C", s.name.as_str(), temp_celsius)
                }
            }
            let _ = write_data_with_point_name(&influx_client, &influx_id, data).await;
            sleep(Duration::from_secs(delta_t_sec)).await;
        }
    });

    api::start_server(cfg).await;
}

fn print_version() {
    log::info!(target: "main", "VERSION is {}", env!("CARGO_PKG_VERSION"));
}
