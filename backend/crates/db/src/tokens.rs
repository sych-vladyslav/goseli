use goseli_core::Result;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// Store a refresh token (as SHA-256 hash)
pub async fn create_refresh_token(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    expires_at: OffsetDateTime,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// Find a refresh token by its hash
pub async fn find_refresh_token(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<(Uuid, OffsetDateTime)>> {
    let result = sqlx::query_as::<_, (Uuid, OffsetDateTime)>(
        "SELECT user_id, expires_at FROM refresh_tokens WHERE token_hash = $1"
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

/// Delete a specific refresh token
pub async fn delete_refresh_token(pool: &PgPool, token_hash: &str) -> Result<()> {
    sqlx::query("DELETE FROM refresh_tokens WHERE token_hash = $1")
        .bind(token_hash)
        .execute(pool)
        .await?;

    Ok(())
}

/// Delete all refresh tokens for a user (logout from all devices)
pub async fn delete_user_refresh_tokens(pool: &PgPool, user_id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Helper to hash a token using SHA-256
pub fn hash_token(token: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}
