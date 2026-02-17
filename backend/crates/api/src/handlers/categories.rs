use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use goseli_core::{
    dto::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest},
    Result,
};
use goseli_db::categories;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

/// GET /api/v1/categories - List all categories for the store
async fn list_categories(
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Vec<CategoryResponse>>> {
    let store_id = get_default_store_id(&state.pool).await?;
    let cats = categories::list_categories(&state.pool, store_id).await?;
    let data: Vec<CategoryResponse> = cats.into_iter().map(CategoryResponse::from).collect();
    Ok(Json(data))
}

/// GET /api/v1/categories/:id - Get a single category
async fn get_category(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryResponse>> {
    let category = categories::get_category(&state.pool, id).await?;
    Ok(Json(CategoryResponse::from(category)))
}

/// POST /api/v1/categories - Create a new category
async fn create_category(
    State(state): State<Arc<crate::AppState>>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<CategoryResponse>)> {
    req.validate()
        .map_err(|e| goseli_core::error::ApiError::validation(e.to_string()))?;

    let store_id = get_default_store_id(&state.pool).await?;
    let category = categories::create_category(&state.pool, store_id, &req).await?;
    Ok((StatusCode::CREATED, Json(CategoryResponse::from(category))))
}

/// PUT /api/v1/categories/:id - Update a category
async fn update_category(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<Json<CategoryResponse>> {
    req.validate()
        .map_err(|e| goseli_core::error::ApiError::validation(e.to_string()))?;

    let category = categories::update_category(&state.pool, id, &req).await?;
    Ok(Json(CategoryResponse::from(category)))
}

/// DELETE /api/v1/categories/:id - Delete a category
async fn delete_category(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    categories::delete_category(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Helper to get default store ID (temporary until domain-based routing)
async fn get_default_store_id(pool: &PgPool) -> Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as("SELECT id FROM stores LIMIT 1")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// Mount category routes
pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route(
            "/api/v1/categories",
            get(list_categories).post(create_category),
        )
        .route(
            "/api/v1/categories/{id}",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
}
