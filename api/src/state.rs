use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}