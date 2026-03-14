use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;

#[path = "../src/app.rs"]
mod app;
#[path = "../src/state.rs"]
mod state;
#[path = "../src/config/mod.rs"]
mod config;
#[path = "../src/adapter/mod.rs"]
mod adapter;

use config::Config;
use state::AppState;

#[tokio::test]
async fn health_returns_200() {
    let config = Config {
        app_host: "127.0.0.1".to_string(),
        app_port: 3000,
        rust_log: "info".to_string(),
    };

    let state = AppState::new(config);
    let app = app::create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}