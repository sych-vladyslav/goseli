use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json, Router,
    routing::get,
};

#[allow(unused_imports)]
use axum::routing::{post, put, delete};
use goseli_core::{
    dto::{CreateProductRequest, PaginatedResponse, PaginationQuery, ProductListQuery, ProductResponse, UpdateProductRequest},
    Result,
};
use goseli_db::products;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

/// List products with pagination and filters
async fn list_products(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationQuery>,
    Query(filters): Query<ProductListQuery>,
) -> Result<Json<PaginatedResponse<ProductResponse>>> {
    // For now, hardcode store_id (will be extracted from auth/domain later)
    let store_id = get_default_store_id(&state.pool).await?;

    let items = products::list_products(
        &state.pool,
        store_id,
        pagination.page,
        pagination.per_page,
        &filters,
    )
    .await?;

    let total = products::count_products(&state.pool, store_id, &filters).await?;

    let response = PaginatedResponse::new(
        items.into_iter().map(ProductResponse::from).collect(),
        total,
        pagination.page,
        pagination.per_page,
    );

    Ok(Json(response))
}

/// Get a single product by slug
async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Json<ProductResponse>> {
    let store_id = get_default_store_id(&state.pool).await?;

    let product = products::get_product_by_slug(&state.pool, store_id, &slug).await?;
    let images = products::get_product_images(&state.pool, product.id).await?;
    let variants = products::get_product_variants(&state.pool, product.id).await?;

    let mut response = ProductResponse::from(product);
    response.images = images;
    response.variants = variants;

    Ok(Json(response))
}

/// Create a new product (admin only)
async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>> {
    let store_id = get_default_store_id(&state.pool).await?;

    let product = products::create_product(&state.pool, store_id, &req).await?;

    Ok(Json(ProductResponse::from(product)))
}

/// Update a product (admin only)
async fn update_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<Json<ProductResponse>> {
    let product = products::update_product(&state.pool, id, &req).await?;

    Ok(Json(ProductResponse::from(product)))
}

/// Delete a product (soft delete, admin only)
async fn delete_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    products::delete_product(&state.pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Helper to get default store ID (temporary until auth is implemented)
async fn get_default_store_id(pool: &PgPool) -> Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as("SELECT id FROM stores LIMIT 1")
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

/// Mount product routes
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/products", get(list_products).post(create_product))
        .route("/api/v1/products/:slug", get(get_product))
        .route("/api/v1/products/:id", put(update_product).delete(delete_product))
}
