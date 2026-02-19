use goseli_core::{
    dto::{CartItemResponse, CartResponse},
    models::{Cart, CartItem},
    ApiError, Result,
};
use sqlx::PgPool;
use uuid::Uuid;

/// Helper struct for querying enriched cart items from the database
#[derive(sqlx::FromRow)]
struct CartItemRow {
    id: Uuid,
    product_id: Uuid,
    variant_id: Option<Uuid>,
    product_name: String,
    product_slug: String,
    product_image_url: Option<String>,
    variant_name: Option<String>,
    price: i32,
    quantity: i32,
    subtotal: i32,
}

/// Get or create a cart for a user or session
pub async fn get_or_create_cart(
    pool: &PgPool,
    store_id: Uuid,
    user_id: Option<Uuid>,
    session_id: Option<String>,
) -> Result<Cart> {
    // Try to find existing cart
    let existing_cart = if let Some(uid) = user_id {
        sqlx::query_as::<_, Cart>(
            "SELECT * FROM carts WHERE store_id = $1 AND user_id = $2 LIMIT 1",
        )
        .bind(store_id)
        .bind(uid)
        .fetch_optional(pool)
        .await?
    } else if let Some(ref sid) = session_id {
        sqlx::query_as::<_, Cart>(
            "SELECT * FROM carts WHERE store_id = $1 AND session_id = $2 LIMIT 1",
        )
        .bind(store_id)
        .bind(sid)
        .fetch_optional(pool)
        .await?
    } else {
        None
    };

    if let Some(cart) = existing_cart {
        return Ok(cart);
    }

    // Create new cart
    let cart_id = Uuid::now_v7();
    let cart = sqlx::query_as::<_, Cart>(
        r#"
        INSERT INTO carts (id, store_id, user_id, session_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(cart_id)
    .bind(store_id)
    .bind(user_id)
    .bind(session_id)
    .fetch_one(pool)
    .await?;

    Ok(cart)
}

/// Get cart with enriched items (joined with product data)
pub async fn get_cart_with_items(pool: &PgPool, cart_id: Uuid) -> Result<CartResponse> {
    // Get the cart
    let cart = sqlx::query_as::<_, Cart>("SELECT * FROM carts WHERE id = $1")
        .bind(cart_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::not_found("Cart not found"))?;

    // Get enriched cart items
    let rows = sqlx::query_as::<_, CartItemRow>(
        r#"
        SELECT
            ci.id,
            ci.product_id,
            ci.variant_id,
            p.name as product_name,
            p.slug as product_slug,
            (SELECT url FROM product_images WHERE product_id = p.id AND is_primary = true LIMIT 1) as product_image_url,
            pv.name as variant_name,
            COALESCE(pv.price, p.price) as price,
            ci.quantity,
            (COALESCE(pv.price, p.price) * ci.quantity) as subtotal
        FROM cart_items ci
        INNER JOIN products p ON ci.product_id = p.id
        LEFT JOIN product_variants pv ON ci.variant_id = pv.id
        WHERE ci.cart_id = $1
        ORDER BY ci.created_at ASC
        "#,
    )
    .bind(cart_id)
    .fetch_all(pool)
    .await?;

    // Convert rows to CartItemResponse
    let items: Vec<CartItemResponse> = rows
        .into_iter()
        .map(|row| CartItemResponse {
            id: row.id,
            product_id: row.product_id,
            variant_id: row.variant_id,
            product_name: row.product_name,
            product_slug: row.product_slug,
            product_image_url: row.product_image_url,
            variant_name: row.variant_name,
            price: row.price,
            quantity: row.quantity,
            subtotal: row.subtotal,
        })
        .collect();

    let total: i32 = items.iter().map(|item| item.subtotal).sum();
    let item_count: i32 = items.iter().map(|item| item.quantity).sum();

    Ok(CartResponse {
        id: cart.id,
        items,
        total,
        item_count,
    })
}

/// Add item to cart (UPSERT: if same product+variant exists, add quantity)
pub async fn add_item(
    pool: &PgPool,
    cart_id: Uuid,
    product_id: Uuid,
    variant_id: Option<Uuid>,
    quantity: i32,
) -> Result<CartItem> {
    // First check stock availability
    let (available_stock, product_name) = if let Some(vid) = variant_id {
        sqlx::query_as::<_, (i32, String)>(
            "SELECT pv.stock_quantity, p.name FROM product_variants pv
             INNER JOIN products p ON pv.product_id = p.id
             WHERE pv.id = $1",
        )
        .bind(vid)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::not_found("Product variant not found"))?
    } else {
        sqlx::query_as::<_, (i32, String)>(
            "SELECT stock_quantity, name FROM products WHERE id = $1",
        )
        .bind(product_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::not_found("Product not found"))?
    };

    // Check if item already exists in cart
    let existing_quantity: Option<i32> = sqlx::query_scalar(
        "SELECT quantity FROM cart_items
         WHERE cart_id = $1 AND product_id = $2 AND variant_id IS NOT DISTINCT FROM $3",
    )
    .bind(cart_id)
    .bind(product_id)
    .bind(variant_id)
    .fetch_optional(pool)
    .await?;

    let new_quantity = existing_quantity.unwrap_or(0) + quantity;

    if new_quantity > available_stock {
        return Err(ApiError::bad_request(format!(
            "Not enough stock for {}. Available: {}, Requested: {}",
            product_name, available_stock, new_quantity
        )));
    }

    // UPSERT: insert or update quantity
    let item_id = Uuid::now_v7();
    let item = sqlx::query_as::<_, CartItem>(
        r#"
        INSERT INTO cart_items (id, cart_id, product_id, variant_id, quantity)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (cart_id, product_id, variant_id)
        DO UPDATE SET
            quantity = cart_items.quantity + EXCLUDED.quantity,
            updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(item_id)
    .bind(cart_id)
    .bind(product_id)
    .bind(variant_id)
    .bind(quantity)
    .fetch_one(pool)
    .await?;

    Ok(item)
}

/// Update cart item quantity
pub async fn update_item_quantity(
    pool: &PgPool,
    item_id: Uuid,
    cart_id: Uuid,
    quantity: i32,
) -> Result<CartItem> {
    // Get item to check stock
    let item =
        sqlx::query_as::<_, CartItem>("SELECT * FROM cart_items WHERE id = $1 AND cart_id = $2")
            .bind(item_id)
            .bind(cart_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| ApiError::not_found("Cart item not found"))?;

    // Check stock availability
    let (available_stock, product_name) = if let Some(vid) = item.variant_id {
        sqlx::query_as::<_, (i32, String)>(
            "SELECT pv.stock_quantity, p.name FROM product_variants pv
             INNER JOIN products p ON pv.product_id = p.id
             WHERE pv.id = $1",
        )
        .bind(vid)
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_as::<_, (i32, String)>(
            "SELECT stock_quantity, name FROM products WHERE id = $1",
        )
        .bind(item.product_id)
        .fetch_one(pool)
        .await?
    };

    if quantity > available_stock {
        return Err(ApiError::bad_request(format!(
            "Not enough stock for {}. Available: {}, Requested: {}",
            product_name, available_stock, quantity
        )));
    }

    // Update quantity
    let updated_item = sqlx::query_as::<_, CartItem>(
        "UPDATE cart_items SET quantity = $1, updated_at = NOW()
         WHERE id = $2 AND cart_id = $3
         RETURNING *",
    )
    .bind(quantity)
    .bind(item_id)
    .bind(cart_id)
    .fetch_one(pool)
    .await?;

    Ok(updated_item)
}

/// Remove item from cart
pub async fn remove_item(pool: &PgPool, item_id: Uuid, cart_id: Uuid) -> Result<()> {
    let result = sqlx::query("DELETE FROM cart_items WHERE id = $1 AND cart_id = $2")
        .bind(item_id)
        .bind(cart_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Cart item not found"));
    }

    Ok(())
}

/// Clear all items from cart
pub async fn clear_cart(pool: &PgPool, cart_id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM cart_items WHERE cart_id = $1")
        .bind(cart_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Merge guest cart into user cart on login
pub async fn merge_carts(pool: &PgPool, guest_cart_id: Uuid, user_cart_id: Uuid) -> Result<()> {
    // Get all items from guest cart
    let guest_items = sqlx::query_as::<_, CartItem>("SELECT * FROM cart_items WHERE cart_id = $1")
        .bind(guest_cart_id)
        .fetch_all(pool)
        .await?;

    // For each guest item, add to user cart (UPSERT will handle duplicates)
    for item in guest_items {
        let item_id = Uuid::now_v7();
        sqlx::query(
            r#"
            INSERT INTO cart_items (id, cart_id, product_id, variant_id, quantity)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (cart_id, product_id, variant_id)
            DO UPDATE SET
                quantity = cart_items.quantity + EXCLUDED.quantity,
                updated_at = NOW()
            "#,
        )
        .bind(item_id)
        .bind(user_cart_id)
        .bind(item.product_id)
        .bind(item.variant_id)
        .bind(item.quantity)
        .execute(pool)
        .await?;
    }

    // Delete guest cart
    sqlx::query("DELETE FROM carts WHERE id = $1")
        .bind(guest_cart_id)
        .execute(pool)
        .await?;

    Ok(())
}
