use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Product, ProductImage, ProductStatus, ProductVariant};

#[derive(Debug, Clone, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub store_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub price: i32,
    pub compare_at_price: Option<i32>,
    pub cost_price: Option<i32>,
    pub sku: Option<String>,
    pub stock_quantity: i32,
    pub attributes: serde_json::Value,
    pub status: ProductStatus,
    pub is_featured: bool,
    pub created_at: String,
    pub updated_at: String,
    pub images: Vec<ProductImage>,
    pub variants: Vec<ProductVariant>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            store_id: product.store_id,
            category_id: product.category_id,
            name: product.name,
            slug: product.slug,
            description: product.description,
            short_description: product.short_description,
            price: product.price,
            compare_at_price: product.compare_at_price,
            cost_price: product.cost_price,
            sku: product.sku,
            stock_quantity: product.stock_quantity,
            attributes: product.attributes,
            status: product.status,
            is_featured: product.is_featured,
            created_at: product.created_at.to_string(),
            updated_at: product.updated_at.to_string(),
            images: vec![],
            variants: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProductRequest {
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub price: i32,
    pub compare_at_price: Option<i32>,
    pub cost_price: Option<i32>,
    pub sku: Option<String>,
    pub stock_quantity: Option<i32>,
    pub attributes: Option<serde_json::Value>,
    pub status: Option<ProductStatus>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProductRequest {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub price: Option<i32>,
    pub compare_at_price: Option<i32>,
    pub cost_price: Option<i32>,
    pub sku: Option<String>,
    pub stock_quantity: Option<i32>,
    pub attributes: Option<serde_json::Value>,
    pub status: Option<ProductStatus>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProductListQuery {
    pub category_id: Option<Uuid>,
    pub status: Option<ProductStatus>,
    pub is_featured: Option<bool>,
    pub search: Option<String>,
}
