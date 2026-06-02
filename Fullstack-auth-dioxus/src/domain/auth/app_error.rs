use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppError {
    Unauthorized,
    Forbidden(String),
    InternalServerError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {msg}"),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
