// Goseli Core - Domain models, types, errors
// No dependencies on other goseli crates

pub mod dto;
pub mod error;
pub mod models;

pub use error::{ApiError, ErrorResponse};

/// Convenience Result alias using ApiError
pub type Result<T> = std::result::Result<T, ApiError>;
