#[cfg(feature = "ssr")]
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use leptos::prelude::ServerFnError;
use leptos::server_fn::codec::JsonEncoding;
use leptos::server_fn::error::{FromServerFnError, ServerFnErrorErr};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::domain_error::DomainError;
#[cfg(feature = "ssr")]
use super::resource_error::ResourceError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    // 4xx errors
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    UnprocessableEntity = 422,
    // 5xx errors
    InternalServerError = 500,
}

#[derive(Clone, Debug, Error, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppError {
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

    #[error("Bad Request: {0}")]
    BadRequest(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::ValidationFailed(_) | AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Database(_)
            | AppError::Cache(_)
            | AppError::MissingContext(_)
            | AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> ErrorCode {
        match self {
            AppError::NotFound(_) => ErrorCode::NotFound,
            AppError::Database(_) | AppError::Cache(_) | AppError::InternalServerError(_) => {
                ErrorCode::InternalServerError
            }
            AppError::Unauthorized => ErrorCode::Unauthorized,
            AppError::Forbidden(_) => ErrorCode::Forbidden,
            AppError::MissingContext(_) => ErrorCode::InternalServerError,
            AppError::ValidationFailed(_) => ErrorCode::UnprocessableEntity,
            AppError::BadRequest(_) => ErrorCode::BadRequest,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn database_error(message: impl Into<String>) -> Self {
        Self::Database(message.into())
    }

    pub fn unauthorized() -> Self {
        Self::Unauthorized
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden(message.into())
    }

    pub fn validation_failed(message: impl Into<String>) -> Self {
        Self::ValidationFailed(message.into())
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::InternalServerError(value.to_string())
    }
}

impl From<ServerFnError> for AppError {
    fn from(value: ServerFnError) -> Self {
        AppError::InternalServerError(value.to_string())
    }
}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::NotFound(message) => Self::NotFound(message),
            DomainError::Database(message) => Self::Database(message),
            DomainError::Cache(message) => Self::Cache(message),
            DomainError::Unauthorized => Self::Unauthorized,
            DomainError::Forbidden(message) => Self::Forbidden(message),
            DomainError::MissingContext(message) => Self::MissingContext(message),
            DomainError::InternalServerError(message) => Self::InternalServerError(message),
            DomainError::ValidationFailed(message) => Self::ValidationFailed(message),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<ResourceError> for AppError {
    fn from(err: ResourceError) -> Self {
        match err {
            ResourceError::Unauthorized => Self::Unauthorized,
            ResourceError::Database(error) => Self::Database(error),
            ResourceError::UnsupportedScale => {
                Self::BadRequest("Unsupported resource scale".to_string())
            }
        }
    }
}

#[cfg(feature = "ssr")]
impl From<AppError> for ResourceError {
    fn from(value: AppError) -> Self {
        match value {
            AppError::Unauthorized => Self::Unauthorized,
            AppError::Database(err) => Self::Database(err.to_string()),
            AppError::NotFound(_)
            | AppError::Cache(_)
            | AppError::Forbidden(_)
            | AppError::MissingContext(_)
            | AppError::InternalServerError(_)
            | AppError::ValidationFailed(_)
            | AppError::BadRequest(_) => Self::UnsupportedScale,
        }
    }
}

#[cfg(feature = "ssr")]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code(), self.to_string()).into_response()
    }
}
