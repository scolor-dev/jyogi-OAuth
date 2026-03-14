pub mod env;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub rust_log: String,
}