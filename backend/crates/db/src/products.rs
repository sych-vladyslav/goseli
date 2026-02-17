use goseli_core::{
    dto::{CreateProductRequest, ProductListParams, UpdateProductRequest},
    models::{Product, ProductImage, ProductVariant},
    Result,
};
use sqlx::PgPool;
use uuid::Uuid;

/// Generate a URL-safe slug from a string
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// List products with pagination and filters
pub async fn list_products(
    pool: &PgPool,
    store_id: Uuid,
    page: i64,
    per_page: i64,
    filters: &ProductListParams,
) -> Result<Vec<Product>> {
    let offset = (page - 1) * per_page;
    let search_pattern = filters.q.as_ref().map(|s| format!("%{}%", s));
    let status_str = filters.status.as_ref().map(|s| s.to_string());

    let mut query = sqlx::QueryBuilder::new(
        "SELECT * FROM products WHERE store_id = "
    );
    query.push_bind(store_id);

    if let Some(category_id) = filters.category_id {
        query.push(" AND category_id = ");
        query.push_bind(category_id);
    }

    if let Some(ref status) = status_str {
        query.push(" AND status = ");
        query.push_bind(status);
    }

    if let Some(ref pattern) = search_pattern {
        query.push(" AND (name ILIKE ");
        query.push_bind(pattern);
        query.push(" OR description ILIKE ");
        query.push_bind(pattern);
        query.push(")");
    }

    query.push(" ORDER BY created_at DESC LIMIT ");
    query.push_bind(per_page);
    query.push(" OFFSET ");
    query.push_bind(offset);

    let products = query
        .build_query_as::<Product>()
        .fetch_all(pool)
        .await?;

    Ok(products)
}

/// Count total products matching filters
pub async fn count_products(
    pool: &PgPool,
    store_id: Uuid,
    filters: &ProductListParams,
) -> Result<i64> {
    let search_pattern = filters.q.as_ref().map(|s| format!("%{}%", s));
    let status_str = filters.status.as_ref().map(|s| s.to_string());

    let mut query = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM products WHERE store_id = "
    );
    query.push_bind(store_id);

    if let Some(category_id) = filters.category_id {
        query.push(" AND category_id = ");
        query.push_bind(category_id);
    }

    if let Some(ref status) = status_str {
        query.push(" AND status = ");
        query.push_bind(status);
    }

    if let Some(ref pattern) = search_pattern {
        query.push(" AND (name ILIKE ");
        query.push_bind(pattern);
        query.push(" OR description ILIKE ");
        query.push_bind(pattern);
        query.push(")");
    }

    let count: (i64,) = query
        .build_query_as()
        .fetch_one(pool)
        .await?;

    Ok(count.0)
}

/// Get product by ID
pub async fn get_product_by_id(pool: &PgPool, id: Uuid) -> Result<Product> {
    let product = sqlx::query_as!(
        Product,
        r#"
        SELECT
            id, store_id, category_id, name, slug, description, short_description,
            price, compare_at_price, cost_price, sku, stock_quantity,
            attributes, status as "status: _", is_featured, created_at, updated_at
        FROM products
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}

/// Get product by slug for a specific store
pub async fn get_product_by_slug(pool: &PgPool, store_id: Uuid, slug: &str) -> Result<Product> {
    let product = sqlx::query_as!(
        Product,
        r#"
        SELECT
            id, store_id, category_id, name, slug, description, short_description,
            price, compare_at_price, cost_price, sku, stock_quantity,
            attributes, status as "status: _", is_featured, created_at, updated_at
        FROM products
        WHERE store_id = $1 AND slug = $2
        "#,
        store_id,
        slug
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}

/// Get product images
pub async fn get_product_images(pool: &PgPool, product_id: Uuid) -> Result<Vec<ProductImage>> {
    let images = sqlx::query_as!(
        ProductImage,
        "SELECT * FROM product_images WHERE product_id = $1 ORDER BY sort_order",
        product_id
    )
    .fetch_all(pool)
    .await?;

    Ok(images)
}

/// Get product variants
pub async fn get_product_variants(pool: &PgPool, product_id: Uuid) -> Result<Vec<ProductVariant>> {
    let variants = sqlx::query_as!(
        ProductVariant,
        "SELECT * FROM product_variants WHERE product_id = $1 ORDER BY sort_order",
        product_id
    )
    .fetch_all(pool)
    .await?;

    Ok(variants)
}

/// Create a new product
pub async fn create_product(
    pool: &PgPool,
    store_id: Uuid,
    req: &CreateProductRequest,
) -> Result<Product> {
    let slug = slugify(&req.name);
    let status_str = req.status.as_ref().map(|s| s.to_string()).unwrap_or_else(|| "draft".to_string());
    let attributes = req.attributes.clone().unwrap_or_else(|| serde_json::json!({}));
    let stock_quantity = req.stock_quantity.unwrap_or(0);
    let is_featured = req.is_featured.unwrap_or(false);

    let product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (
            store_id, category_id, name, slug, description, short_description,
            price, compare_at_price, cost_price, sku, stock_quantity,
            attributes, status, is_featured
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING
            id, store_id, category_id, name, slug, description, short_description,
            price, compare_at_price, cost_price, sku, stock_quantity,
            attributes, status as "status: _", is_featured, created_at, updated_at
        "#,
        store_id,
        req.category_id,
        req.name,
        slug,
        req.description,
        req.short_description,
        req.price,
        req.compare_at_price,
        req.cost_price,
        req.sku,
        stock_quantity,
        attributes,
        status_str,
        is_featured
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}

/// Update a product
pub async fn update_product(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateProductRequest,
) -> Result<Product> {
    // First fetch the current product
    let current = get_product_by_id(pool, id).await?;

    // Build update query dynamically
    let name = req.name.as_ref().unwrap_or(&current.name);
    let slug = if req.name.is_some() {
        slugify(name)
    } else {
        current.slug.clone()
    };
    let status_str = req.status.as_ref().map(|s| s.to_string()).unwrap_or_else(|| current.status.to_string());

    let product = sqlx::query_as!(
        Product,
        r#"
        UPDATE products SET
            category_id = COALESCE($2, category_id),
            name = $3,
            slug = $4,
            description = COALESCE($5, description),
            short_description = COALESCE($6, short_description),
            price = COALESCE($7, price),
            compare_at_price = COALESCE($8, compare_at_price),
            cost_price = COALESCE($9, cost_price),
            sku = COALESCE($10, sku),
            stock_quantity = COALESCE($11, stock_quantity),
            attributes = COALESCE($12, attributes),
            status = $13,
            is_featured = COALESCE($14, is_featured),
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, store_id, category_id, name, slug, description, short_description,
            price, compare_at_price, cost_price, sku, stock_quantity,
            attributes, status as "status: _", is_featured, created_at, updated_at
        "#,
        id,
        req.category_id,
        name,
        slug,
        req.description,
        req.short_description,
        req.price,
        req.compare_at_price,
        req.cost_price,
        req.sku,
        req.stock_quantity,
        req.attributes,
        status_str,
        req.is_featured
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}

/// Soft delete a product (set status to archived)
pub async fn delete_product(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query!(
        "UPDATE products SET status = 'archived', updated_at = NOW() WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
