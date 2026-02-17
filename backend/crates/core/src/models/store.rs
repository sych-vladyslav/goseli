use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub config: serde_json::Value,
    pub theme: String,
    pub currency: String,
    pub domain: Option<String>,
    pub is_active: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// Runtime store configuration, loaded from DB at container startup.
/// Stored as JSONB in stores.config, augmented with top-level fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreConfig {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub currency: String,
    pub theme: String,
    pub product_schema: Option<serde_json::Value>,
    pub checkout_flow: Option<Vec<String>>,
    pub features: Option<serde_json::Value>,
}
