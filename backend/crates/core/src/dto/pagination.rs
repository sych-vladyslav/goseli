use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page.max(1) - 1) * self.per_page.clamp(1, 100)
    }

    pub fn limit(&self) -> i64 {
        self.per_page.clamp(1, 100)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub per_page: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn new(params: &PaginationParams, total_items: i64) -> Self {
        let per_page = params.per_page.clamp(1, 100);
        let total_pages = (total_items + per_page - 1) / per_page;
        Self {
            page: params.page.max(1),
            per_page,
            total_items,
            total_pages,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}
