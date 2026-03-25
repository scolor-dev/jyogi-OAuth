use api::{
    adapter::logging::init,
    app,
    config::Config,
    state::AppState,
};
use sqlx::postgres::PgPoolOptions;

const DB_MAX_CONNECTIONS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), api::error::AppError> {
    let _ = dotenvy::dotenv();

    let config = Config::from_env()?;

    init::init(&config.rust_log);

    tracing::info!("connecting to database");

    let db = PgPoolOptions::new()
        .max_connections(DB_MAX_CONNECTIONS)
        .connect(&config.database_url)
        .await
        .map_err(|err| {
            tracing::error!("failed to connect to database: {err}");
            err
        })?;

    tracing::info!("connected to database");
    tracing::info!("starting server");

    let addr = config.listen_addr()?;
    let state = AppState::new(db);
    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("server listening on {addr}");

    axum::serve(listener, app).await?;

    Ok(())
}
