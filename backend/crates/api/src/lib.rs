// Goseli API - Axum routes, handlers, middleware
// Depends on: goseli-core, goseli-db, goseli-auth, goseli-storage

pub mod handlers;

// Re-export handlers for convenience
pub use handlers::*;
