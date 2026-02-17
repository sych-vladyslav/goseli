use goseli_core::{
    dto::{CreateCategoryRequest, UpdateCategoryRequest},
    models::Category,
    Result,
};
use sqlx::PgPool;
use uuid::Uuid;

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// List all categories for a store
pub async fn list_categories(pool: &PgPool, store_id: Uuid) -> Result<Vec<Category>> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE store_id = $1 ORDER BY sort_order, name",
    )
    .bind(store_id)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

/// Get category by ID
pub async fn get_category(pool: &PgPool, id: Uuid) -> Result<Category> {
    let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(category)
}

/// Create a new category
pub async fn create_category(
    pool: &PgPool,
    store_id: Uuid,
    req: &CreateCategoryRequest,
) -> Result<Category> {
    let slug = slugify(&req.name);
    let sort_order = req.sort_order.unwrap_or(0);

    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (store_id, parent_id, name, slug, description, sort_order)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(store_id)
    .bind(req.parent_id)
    .bind(&req.name)
    .bind(&slug)
    .bind(&req.description)
    .bind(sort_order)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

/// Update a category
pub async fn update_category(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateCategoryRequest,
) -> Result<Category> {
    let current = get_category(pool, id).await?;
    let name = req.name.as_ref().unwrap_or(&current.name);
    let slug = if req.name.is_some() {
        slugify(name)
    } else {
        current.slug.clone()
    };

    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories SET
            parent_id = COALESCE($2, parent_id),
            name = $3,
            slug = $4,
            description = COALESCE($5, description),
            sort_order = COALESCE($6, sort_order),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(req.parent_id)
    .bind(name)
    .bind(&slug)
    .bind(&req.description)
    .bind(req.sort_order)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

/// Delete a category
pub async fn delete_category(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
