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
};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let config = env::load();
    let state = AppState::new(config.clone());

    let app = app::create_app();

    let addr = format!("{}:{}", config.app_host, config.app_port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}