use std::{error::Error as StdError, fmt, net::AddrParseError};

#[derive(Debug)]
pub enum ConfigError {
    InvalidPort {
        value: String,
        source: std::num::ParseIntError,
    },
    InvalidListenAddress {
        host: String,
        port: u16,
        source: AddrParseError,
    },
    MissingEnvVar {
        name: &'static str,
    },
    InvalidDbMaxConnections {
        value: String,
        source: std::num::ParseIntError,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort { value, .. } => {
                write!(f, "APP_PORT must be a valid u16, got `{value}`")
            }
            Self::InvalidListenAddress { host, port, .. } => {
                write!(f, "invalid listen address `{host}:{port}`")
            }
            Self::MissingEnvVar { name } => {
                write!(f, "required environment variable `{name}` is not set")
            }
            Self::InvalidDbMaxConnections { value, .. } => {
                write!(f, "DB_MAX_CONNECTIONS must be a valid u32, got `{value}`")
            }
        }
    }
}

impl StdError for ConfigError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidPort { source, .. }
            | Self::InvalidDbMaxConnections { source, .. } => Some(source),
            Self::InvalidListenAddress { source, .. } => Some(source),
            Self::MissingEnvVar { .. } => None,
        }
    }
}