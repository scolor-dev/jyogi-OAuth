use axum::Router;

use crate::adapter::http::routes;
use crate::adapter::persistence::db;
use crate::config::AppConfig;
use crate::state::AppState;

pub async fn build_app(cfg: AppConfig) -> Router {
    let pool = db::connect(&cfg.database_url).await;

    let state = AppState::new(pool, cfg.jwt.clone());

    Router::new()
        .merge(routes::health_route::router())
        .merge(routes::auth_routes::router())
        .with_state(state)
}