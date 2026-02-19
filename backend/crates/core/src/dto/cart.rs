use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Cart response with enriched items
#[derive(Debug, Clone, Serialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub items: Vec<CartItemResponse>,
    pub total: i32,
    pub item_count: i32,
}

/// Enriched cart item with product details
#[derive(Debug, Clone, Serialize)]
pub struct CartItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub product_name: String,
    pub product_slug: String,
    pub product_image_url: Option<String>,
    pub variant_name: Option<String>,
    pub price: i32,
    pub quantity: i32,
    pub subtotal: i32,
}

/// Add item to cart request
#[derive(Debug, Deserialize, Validate)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

/// Update cart item quantity request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCartItemRequest {
    #[validate(range(min = 1))]
    pub quantity: i32,
}
