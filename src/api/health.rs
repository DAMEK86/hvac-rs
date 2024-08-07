use crate::config::AppConfig;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    version: String,
    config: AppConfig,
}

pub(crate) fn create_router(cfg: AppConfig) -> Router {
    Router::new()
        .route("/health", routing::get(get_config))
        .with_state(cfg)
}

async fn get_config(State(cfg): State<AppConfig>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(&HealthResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
            config: cfg,
        }),
    )
        .into_response()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::read_config;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn get_health() {
        let cfg = read_config();
        let client = TestClient::new(create_router(cfg.clone())).await;
        let response = client.get("/health").send().await;
        assert_eq!(response.status(), StatusCode::OK);
        let resp = response.text().await;
        assert!(resp.contains(&env!("CARGO_PKG_VERSION").to_string()));
    }
}
