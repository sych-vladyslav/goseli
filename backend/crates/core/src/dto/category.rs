use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Category;

#[derive(Debug, Clone, Serialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub store_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            store_id: category.store_id,
            parent_id: category.parent_id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            sort_order: category.sort_order,
            created_at: category.created_at.to_string(),
            updated_at: category.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCategoryRequest {
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCategoryRequest {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}
