use axum::{
    extract::State,
    http::StatusCode,
    Json, Router,
    routing::{get, post},
};
use goseli_auth::{generate_access_token, generate_refresh_token, hash_password, verify_password, validate_token, AuthUser};
use goseli_core::{
    dto::{AuthResponse, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest, TokenPair},
    models::user::{UserProfile, UserRole},
    Result,
};
use goseli_db::{tokens, users};
use sqlx::PgPool;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

/// POST /api/v1/auth/register - Create a new user account
async fn register(
    State(state): State<Arc<crate::AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    // For now, hardcode store_id (will be extracted from domain later)
    let store_id = get_default_store_id(&state.pool).await?;

    // Check if user already exists
    if users::find_user_by_email(&state.pool, store_id, &req.email).await?.is_some() {
        return Err(goseli_core::error::ApiError::bad_request("Email already registered"));
    }

    // Hash password
    let password_hash = hash_password(&req.password)?;

    // Create user
    let user = users::create_user(
        &state.pool,
        store_id,
        &req.email,
        &password_hash,
        req.first_name.as_deref(),
        req.last_name.as_deref(),
    ).await?;

    // Generate tokens
    let access_token = generate_access_token(user.id, user.email.clone(), user.role, store_id)?;
    let refresh_token = generate_refresh_token(user.id, user.email.clone(), user.role, store_id)?;

    // Store refresh token hash
    let token_hash = tokens::hash_token(&refresh_token);
    let expires_at = OffsetDateTime::now_utc() + time::Duration::days(7);
    tokens::create_refresh_token(&state.pool, user.id, &token_hash, expires_at).await?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            user: UserProfile::from(user),
            access_token,
            refresh_token,
        }),
    ))
}

/// POST /api/v1/auth/login - Authenticate and get tokens
async fn login(
    State(state): State<Arc<crate::AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let store_id = get_default_store_id(&state.pool).await?;

    // Find user
    let user = users::find_user_by_email(&state.pool, store_id, &req.email)
        .await?
        .ok_or_else(|| goseli_core::error::ApiError::unauthorized("Invalid credentials"))?;

    // Verify password
    if !verify_password(&req.password, &user.password_hash)? {
        return Err(goseli_core::error::ApiError::unauthorized("Invalid credentials"));
    }

    // Check if user is active
    if !user.is_active {
        return Err(goseli_core::error::ApiError::forbidden("Account is disabled"));
    }

    // Generate tokens
    let access_token = generate_access_token(user.id, user.email.clone(), user.role, store_id)?;
    let refresh_token = generate_refresh_token(user.id, user.email.clone(), user.role, store_id)?;

    // Store refresh token hash
    let token_hash = tokens::hash_token(&refresh_token);
    let expires_at = OffsetDateTime::now_utc() + time::Duration::days(7);
    tokens::create_refresh_token(&state.pool, user.id, &token_hash, expires_at).await?;

    Ok(Json(AuthResponse {
        user: UserProfile::from(user),
        access_token,
        refresh_token,
    }))
}

/// POST /api/v1/auth/refresh - Refresh access token using refresh token
async fn refresh(
    State(state): State<Arc<crate::AppState>>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<TokenPair>> {
    // Validate refresh token
    let claims = validate_token(&req.refresh_token)?;

    // Check if refresh token exists in DB
    let token_hash = tokens::hash_token(&req.refresh_token);
    let (user_id, expires_at) = tokens::find_refresh_token(&state.pool, &token_hash)
        .await?
        .ok_or_else(|| goseli_core::error::ApiError::unauthorized("Invalid refresh token"))?;

    // Check if token is expired
    if expires_at < OffsetDateTime::now_utc() {
        tokens::delete_refresh_token(&state.pool, &token_hash).await?;
        return Err(goseli_core::error::ApiError::unauthorized("Refresh token expired"));
    }

    // Get user
    let user = users::find_user_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| goseli_core::error::ApiError::unauthorized("User not found"))?;

    // Delete old refresh token
    tokens::delete_refresh_token(&state.pool, &token_hash).await?;

    // Generate new tokens
    let access_token = generate_access_token(user.id, user.email.clone(), user.role, claims.store_id)?;
    let new_refresh_token = generate_refresh_token(user.id, user.email.clone(), user.role, claims.store_id)?;

    // Store new refresh token hash
    let new_token_hash = tokens::hash_token(&new_refresh_token);
    let new_expires_at = OffsetDateTime::now_utc() + time::Duration::days(7);
    tokens::create_refresh_token(&state.pool, user.id, &new_token_hash, new_expires_at).await?;

    Ok(Json(TokenPair {
        access_token,
        refresh_token: new_refresh_token,
    }))
}

/// POST /api/v1/auth/logout - Invalidate refresh token
async fn logout(
    State(state): State<Arc<crate::AppState>>,
    Json(req): Json<LogoutRequest>,
) -> Result<StatusCode> {
    let token_hash = tokens::hash_token(&req.refresh_token);
    tokens::delete_refresh_token(&state.pool, &token_hash).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/auth/me - Get current user profile
async fn me(
    auth_user: AuthUser,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<UserProfile>> {
    let user = users::find_user_by_id(&state.pool, auth_user.user_id)
        .await?
        .ok_or_else(|| goseli_core::error::ApiError::not_found("User not found"))?;

    Ok(Json(UserProfile::from(user)))
}

/// Helper to get default store ID (temporary until domain-based routing)
async fn get_default_store_id(pool: &PgPool) -> Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as("SELECT id FROM stores LIMIT 1")
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

/// Mount auth routes
pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/refresh", post(refresh))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/me", get(me))
}
