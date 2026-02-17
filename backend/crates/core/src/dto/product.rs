use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::models::category::CategorySummary;
use crate::models::product::{ProductImage, ProductStatus, ProductVariant};

use super::pagination::PaginatedResponse;

/// Product as returned by the API (enriched with category, images, variants).
#[derive(Debug, Clone, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub price: i32,
    pub compare_at_price: Option<i32>,
    pub status: ProductStatus,
    pub is_featured: bool,
    pub sku: Option<String>,
    pub stock_quantity: i32,
    pub attributes: serde_json::Value,
    pub category: Option<CategorySummary>,
    pub images: Vec<ProductImage>,
    pub variants: Vec<ProductVariant>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

pub type ProductListResponse = PaginatedResponse<ProductResponse>;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(max = 500))]
    pub short_description: Option<String>,
    #[validate(range(min = 0))]
    pub price: i32,
    #[validate(range(min = 0))]
    pub compare_at_price: Option<i32>,
    #[validate(range(min = 0))]
    pub cost_price: Option<i32>,
    #[validate(length(max = 100))]
    pub sku: Option<String>,
    pub category_id: Option<Uuid>,
    #[validate(range(min = 0))]
    pub stock_quantity: Option<i32>,
    pub attributes: Option<serde_json::Value>,
    pub status: Option<ProductStatus>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    #[validate(length(max = 500))]
    pub short_description: Option<String>,
    #[validate(range(min = 0))]
    pub price: Option<i32>,
    #[validate(range(min = 0))]
    pub compare_at_price: Option<i32>,
    #[validate(range(min = 0))]
    pub cost_price: Option<i32>,
    #[validate(length(max = 100))]
    pub sku: Option<String>,
    pub category_id: Option<Uuid>,
    #[validate(range(min = 0))]
    pub stock_quantity: Option<i32>,
    pub attributes: Option<serde_json::Value>,
    pub status: Option<ProductStatus>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVariantRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 100))]
    pub sku: Option<String>,
    #[validate(range(min = 0))]
    pub price: i32,
    #[validate(range(min = 0))]
    pub compare_at_price: Option<i32>,
    #[validate(range(min = 0))]
    pub stock_quantity: Option<i32>,
    pub attributes: Option<serde_json::Value>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateVariantRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 100))]
    pub sku: Option<String>,
    #[validate(range(min = 0))]
    pub price: Option<i32>,
    #[validate(range(min = 0))]
    pub compare_at_price: Option<i32>,
    #[validate(range(min = 0))]
    pub stock_quantity: Option<i32>,
    pub attributes: Option<serde_json::Value>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

/// Query parameters for product listing.
#[derive(Debug, Deserialize)]
pub struct ProductListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<ProductStatus>,
    pub category_id: Option<Uuid>,
    pub sort: Option<ProductSort>,
    pub q: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductSort {
    PriceAsc,
    PriceDesc,
    CreatedAtDesc,
    NameAsc,
}
