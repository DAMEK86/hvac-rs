use crate::config::AppConfig;
use axum::Router;
use tokio::net::TcpListener;

mod health;

pub async fn start_server(cfg: AppConfig) {
    let addr = format!("{}:{}", cfg.http.listen_address, cfg.http.listen_port);
    let app = Router::new().nest("/", health::create_router(cfg.clone()));

    let listener: TcpListener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            log::error!(target: "api", "Unable to create TCP listener: {}", e);
            std::process::exit(1);
        }
    };

    log::info!(target: "api", "Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        log::error!(target: "api", "Unable to start server: {}", e);
        std::process::exit(1);
    });
}
