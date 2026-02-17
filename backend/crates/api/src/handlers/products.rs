use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use goseli_core::{
    dto::{
        CreateProductRequest, PaginatedResponse, PaginationMeta, PaginationParams,
        ProductListParams, ProductResponse, UpdateProductRequest,
    },
    Result,
};
use goseli_db::products;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

/// GET /api/v1/products - List products with pagination and filters
async fn list_products(
    State(state): State<Arc<crate::AppState>>,
    Query(params): Query<ProductListParams>,
) -> Result<Json<PaginatedResponse<ProductResponse>>> {
    let store_id = get_default_store_id(&state.pool).await?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let pagination = PaginationParams { page, per_page };

    let items = products::list_products(&state.pool, store_id, page, per_page, &params).await?;
    let total = products::count_products(&state.pool, store_id, &params).await?;

    let data: Vec<ProductResponse> = items.into_iter().map(ProductResponse::from).collect();
    let response = PaginatedResponse {
        data,
        pagination: PaginationMeta::new(&pagination, total),
    };

    Ok(Json(response))
}

/// GET /api/v1/products/:id - Get a single product with images and variants
async fn get_product(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductResponse>> {
    let product = products::get_product_by_id(&state.pool, id).await?;
    let images = products::get_product_images(&state.pool, product.id).await?;
    let variants = products::get_product_variants(&state.pool, product.id).await?;

    let mut response = ProductResponse::from(product);
    response.images = images;
    response.variants = variants;

    Ok(Json(response))
}

/// POST /api/v1/products - Create a new product
async fn create_product(
    State(state): State<Arc<crate::AppState>>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<ProductResponse>)> {
    req.validate()
        .map_err(|e| goseli_core::error::ApiError::validation(e.to_string()))?;

    let store_id = get_default_store_id(&state.pool).await?;
    let product = products::create_product(&state.pool, store_id, &req).await?;

    Ok((StatusCode::CREATED, Json(ProductResponse::from(product))))
}

/// PUT /api/v1/products/:id - Update a product
async fn update_product(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<Json<ProductResponse>> {
    req.validate()
        .map_err(|e| goseli_core::error::ApiError::validation(e.to_string()))?;

    let product = products::update_product(&state.pool, id, &req).await?;

    Ok(Json(ProductResponse::from(product)))
}

/// DELETE /api/v1/products/:id - Soft delete a product (archive)
async fn delete_product(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    products::delete_product(&state.pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Helper to get default store ID (temporary until domain-based routing)
async fn get_default_store_id(pool: &PgPool) -> Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as("SELECT id FROM stores LIMIT 1")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// Mount product routes
pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route("/api/v1/products", get(list_products).post(create_product))
        .route(
            "/api/v1/products/{id}",
            get(get_product).put(update_product).delete(delete_product),
        )
}
