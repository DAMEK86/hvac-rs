use influx_db_client::{point, Client, Point, Precision};
use log::info;

pub fn get_infux_db_client(
    cfg: &crate::config::InfluxDB,
) -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::new(
        format!("{}:{}", cfg.url, cfg.port).parse().unwrap(),
        cfg.db.as_str(),
    )
    .set_authentication(cfg.user.as_str(), cfg.password.as_str());
    Ok(client)
}

pub async fn write_data_with_point_name(
    client: &Client,
    influx_id: &str,
    data: Vec<(&str, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut point = point!(influx_id);
    for &(key, value) in data.iter() {
        point = point.add_field(key, value);
    }
    let field_count = point.fields.len();
    if !point.fields.is_empty() {
        client
            .write_point(point, Some(Precision::Seconds), None)
            .await?;
    }
    info!("{field_count} fields written to point {influx_id}");
    Ok(())
}
