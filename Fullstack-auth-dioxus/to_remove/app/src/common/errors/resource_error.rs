use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceError {
    Unauthorized,
    Database(String),
    UnsupportedScale,
}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceError::Unauthorized => write!(f, "Unauthorized access"),
            ResourceError::Database(msg) => write!(f, "Database error: {msg}"),
            ResourceError::UnsupportedScale => write!(f, "Unsupported scale operation"),
        }
    }
}

impl Error for ResourceError {}
