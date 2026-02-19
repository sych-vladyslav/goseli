use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use goseli_auth::AuthUser;
use goseli_core::{
    dto::{AddToCartRequest, CartResponse, UpdateCartItemRequest},
    Result,
};
use goseli_db::cart;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

const SESSION_COOKIE_NAME: &str = "goseli_session";

/// Helper to get or create a session ID from cookies
fn get_or_create_session_id(jar: &CookieJar) -> String {
    jar.get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| Uuid::now_v7().to_string())
}

/// Helper to get default store ID (temporary until domain-based routing)
async fn get_default_store_id(pool: &PgPool) -> Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as("SELECT id FROM stores LIMIT 1")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// GET /api/v1/cart - Get current cart
async fn get_cart(
    State(state): State<Arc<crate::AppState>>,
    auth_user: Option<AuthUser>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<CartResponse>)> {
    let store_id = get_default_store_id(&state.pool).await?;

    let (user_id, session_id) = if let Some(user) = auth_user {
        (Some(user.user_id), None)
    } else {
        let sid = get_or_create_session_id(&jar);
        (None, Some(sid))
    };

    // Get or create cart
    let cart = cart::get_or_create_cart(&state.pool, store_id, user_id, session_id.clone()).await?;

    // Get cart with enriched items
    let cart_response = cart::get_cart_with_items(&state.pool, cart.id).await?;

    // Set session cookie if guest
    let jar = if user_id.is_none() {
        if let Some(sid) = session_id {
            let cookie = format!("{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000", SESSION_COOKIE_NAME, sid);
            jar.add(axum_extra::extract::cookie::Cookie::parse(cookie).unwrap())
        } else {
            jar
        }
    } else {
        jar
    };

    Ok((jar, Json(cart_response)))
}

/// POST /api/v1/cart/items - Add item to cart
async fn add_to_cart(
    State(state): State<Arc<crate::AppState>>,
    auth_user: Option<AuthUser>,
    jar: CookieJar,
    Json(req): Json<AddToCartRequest>,
) -> Result<(CookieJar, Json<CartResponse>)> {
    req.validate()
        .map_err(|e| goseli_core::error::ApiError::validation(e.to_string()))?;

    let store_id = get_default_store_id(&state.pool).await?;

    let (user_id, session_id) = if let Some(user) = auth_user {
        (Some(user.user_id), None)
    } else {
        let sid = get_or_create_session_id(&jar);
        (None, Some(sid))
    };

    // Get or create cart
    let cart = cart::get_or_create_cart(&state.pool, store_id, user_id, session_id.clone()).await?;

    // Add item to cart
    cart::add_item(
        &state.pool,
        cart.id,
        req.product_id,
        req.variant_id,
        req.quantity,
    )
    .await?;

    // Get updated cart with enriched items
    let cart_response = cart::get_cart_with_items(&state.pool, cart.id).await?;

    // Set session cookie if guest
    let jar = if user_id.is_none() {
        if let Some(sid) = session_id {
            let cookie = format!("{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000", SESSION_COOKIE_NAME, sid);
            jar.add(axum_extra::extract::cookie::Cookie::parse(cookie).unwrap())
        } else {
            jar
        }
    } else {
        jar
    };

    Ok((jar, Json(cart_response)))
}

/// PUT /api/v1/cart/items/:id - Update cart item quantity
async fn update_cart_item(
    State(state): State<Arc<crate::AppState>>,
    auth_user: Option<AuthUser>,
    jar: CookieJar,
    Path(item_id): Path<Uuid>,
    Json(req): Json<UpdateCartItemRequest>,
) -> Result<Json<CartResponse>> {
    req.validate()
        .map_err(|e| goseli_core::error::ApiError::validation(e.to_string()))?;

    let store_id = get_default_store_id(&state.pool).await?;

    let (user_id, session_id) = if let Some(user) = auth_user {
        (Some(user.user_id), None)
    } else {
        let sid = get_or_create_session_id(&jar);
        (None, Some(sid))
    };

    // Get cart
    let cart = cart::get_or_create_cart(&state.pool, store_id, user_id, session_id).await?;

    // Update item quantity
    cart::update_item_quantity(&state.pool, item_id, cart.id, req.quantity).await?;

    // Get updated cart with enriched items
    let cart_response = cart::get_cart_with_items(&state.pool, cart.id).await?;

    Ok(Json(cart_response))
}

/// DELETE /api/v1/cart/items/:id - Remove item from cart
async fn remove_cart_item(
    State(state): State<Arc<crate::AppState>>,
    auth_user: Option<AuthUser>,
    jar: CookieJar,
    Path(item_id): Path<Uuid>,
) -> Result<StatusCode> {
    let store_id = get_default_store_id(&state.pool).await?;

    let (user_id, session_id) = if let Some(user) = auth_user {
        (Some(user.user_id), None)
    } else {
        let sid = get_or_create_session_id(&jar);
        (None, Some(sid))
    };

    // Get cart
    let cart = cart::get_or_create_cart(&state.pool, store_id, user_id, session_id).await?;

    // Remove item
    cart::remove_item(&state.pool, item_id, cart.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /api/v1/cart - Clear entire cart
async fn clear_cart(
    State(state): State<Arc<crate::AppState>>,
    auth_user: Option<AuthUser>,
    jar: CookieJar,
) -> Result<StatusCode> {
    let store_id = get_default_store_id(&state.pool).await?;

    let (user_id, session_id) = if let Some(user) = auth_user {
        (Some(user.user_id), None)
    } else {
        let sid = get_or_create_session_id(&jar);
        (None, Some(sid))
    };

    // Get cart
    let cart = cart::get_or_create_cart(&state.pool, store_id, user_id, session_id).await?;

    // Clear cart
    cart::clear_cart(&state.pool, cart.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Mount cart routes
pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route("/api/v1/cart", get(get_cart).delete(clear_cart))
        .route("/api/v1/cart/items", post(add_to_cart))
        .route(
            "/api/v1/cart/items/:id",
            put(update_cart_item).delete(remove_cart_item),
        )
}
