use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum DomainError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Database Error: {0}")]
    Database(String),

    #[error("Cache Error: {0}")]
    Cache(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Missing Context: {0}")]
    MissingContext(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Validation Failed: {0}")]
    ValidationFailed(String),
}
