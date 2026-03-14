mod app;
mod config;
mod state;
mod error;
mod domain;
mod service;
mod adapter;

use crate::{
    config::env,
    state::AppState,
    adapter::logging::init,
};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let config = env::load();

    init::init(&config.rust_log);
    tracing::info!("starting server");

    let state = AppState::new(config.clone());

    let app = app::create_app();

    let addr = format!("{}:{}", config.app_host, config.app_port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    tracing::info!("server listening");

    axum::serve(listener, app).await.unwrap();
}