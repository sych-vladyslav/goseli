use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::get,
};

#[allow(unused_imports)]
use axum::routing::{post, put, delete};
use goseli_core::{
    dto::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest},
    Result,
};
use goseli_db::categories;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::products::AppState;

/// List all categories for the store
async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CategoryResponse>>> {
    let store_id = get_default_store_id(&state.pool).await?;

    let items = categories::list_categories(&state.pool, store_id).await?;

    Ok(Json(items.into_iter().map(CategoryResponse::from).collect()))
}

/// Get a single category
async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryResponse>> {
    let category = categories::get_category(&state.pool, id).await?;

    Ok(Json(CategoryResponse::from(category)))
}

/// Create a new category (admin only)
async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<Json<CategoryResponse>> {
    let store_id = get_default_store_id(&state.pool).await?;

    let category = categories::create_category(&state.pool, store_id, &req).await?;

    Ok(Json(CategoryResponse::from(category)))
}

/// Update a category (admin only)
async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<Json<CategoryResponse>> {
    let category = categories::update_category(&state.pool, id, &req).await?;

    Ok(Json(CategoryResponse::from(category)))
}

/// Delete a category (admin only)
async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    categories::delete_category(&state.pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Helper to get default store ID (temporary until auth is implemented)
async fn get_default_store_id(pool: &PgPool) -> Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as("SELECT id FROM stores LIMIT 1")
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

/// Mount category routes
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/categories", get(list_categories).post(create_category))
        .route("/api/v1/categories/:id", get(get_category).put(update_category).delete(delete_category))
}
