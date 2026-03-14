use axum::Router;

use crate::{
    adapter::http::routes::health,
    state::AppState,
};

pub fn create_app() -> Router {
    Router::new()
        .merge(health::routes())
}