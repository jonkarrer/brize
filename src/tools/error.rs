use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Config(String),
    Database(String),
    Network(String),
    Validation(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Convert from config::ConfigError
impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::Config(err.to_string())
    }
}

// You can add more From implementations for other library errors
impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::Config(format!("Environment variable error: {}", err))
    }
}

// Example for other common errors you might encounter
// impl From<serde_json::Error> for AppError {
//     fn from(err: serde_json::Error) -> Self {
//         AppError::Internal(format!("JSON parsing error: {}", err))
//     }
// }

// Convert from sqlx::Error, which is sea-orm's error
// impl From<sqlx::Error> for AppError {
//     fn from(err: sqlx::Error) -> Self {
//         AppError::Database(err.to_string())
//     }
// }

pub type Result<T> = std::result::Result<T, AppError>;
