mod app;
mod config;
mod state;
mod error;
mod domain;
mod service;
mod adapter;

use crate::{
    adapter::logging::init,
    config::env,
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let config = env::load();

    init::init(&config.rust_log);

    tracing::info!("starting server");

    let state = AppState::new(config.clone());
    let app = app::create_app(state);

    let addr = format!("{}:{}", config.app_host, config.app_port);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("server listening");

    axum::serve(listener, app).await?;

    Ok(())
}