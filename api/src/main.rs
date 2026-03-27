use api::{
    adapter::logging::init,
    app,
    config::Config,
    state::AppState,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), api::errors::app::AppError> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install CryptoProvider");
    
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    init::init(&config.rust_log);

    tracing::info!("connecting to database");

    let db = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .map_err(|err| {
            tracing::error!("failed to connect to database: {err}");
            err
        })?;

    tracing::info!("connected to database");
    tracing::info!("starting server");

    let addr = config.listen_addr()?;
    let state = AppState::new(db, config.jwt_secret);
    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("server listening on {addr}");

    axum::serve(listener, app).await?;

    Ok(())
}
