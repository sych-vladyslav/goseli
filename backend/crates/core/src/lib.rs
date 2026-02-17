// Goseli Core - Domain models, types, JWT, password hashing, errors
// No dependencies on other goseli crates

pub mod models;
pub mod dto;
pub mod error;

pub use error::{ApiError, Result};
