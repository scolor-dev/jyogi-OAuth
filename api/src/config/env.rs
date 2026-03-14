use super::Config;

pub fn load() -> Config {
    let app_host = std::env::var("APP_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    let app_port = std::env::var("APP_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3000);

    let rust_log = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string());

    Config {
        app_host,
        app_port,
        rust_log,
    }
}