// Goseli API - Axum routes, handlers, middleware

pub mod handlers;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: ConnectionManager,
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    database: String,
    redis: String,
}

async fn health_check(State(state): State<Arc<AppState>>) -> (StatusCode, Json<HealthResponse>) {
    let db_ok = sqlx::query("SELECT 1").execute(&state.pool).await.is_ok();

    let redis_ok = {
        let mut conn = state.redis.clone();
        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
            .is_ok()
    };

    let status = if db_ok && redis_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let body = HealthResponse {
        status: if db_ok && redis_ok {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        database: if db_ok {
            "ok".to_string()
        } else {
            "error".to_string()
        },
        redis: if redis_ok {
            "ok".to_string()
        } else {
            "error".to_string()
        },
    };

    (status, Json(body))
}

pub fn build_router(state: Arc<AppState>) -> Router {
    // TODO: Restrict CORS origins for production (use CORS_ALLOWED_ORIGINS env var)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_check))
        .merge(handlers::auth::routes())
        .merge(handlers::products::routes())
        .merge(handlers::categories::routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
