use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub store_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// Lightweight category reference embedded in product responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

impl From<Category> for CategorySummary {
    fn from(c: Category) -> Self {
        Self {
            id: c.id,
            name: c.name,
            slug: c.slug,
        }
    }
}
