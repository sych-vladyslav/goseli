use goseli_core::{models::user::User, Result};
use sqlx::PgPool;
use uuid::Uuid;

/// Create a new user
pub async fn create_user(
    pool: &PgPool,
    store_id: Uuid,
    email: &str,
    password_hash: &str,
    first_name: Option<&str>,
    last_name: Option<&str>,
) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (store_id, email, password_hash, first_name, last_name, role)
        VALUES ($1, $2, $3, $4, $5, 'customer')
        RETURNING *
        "#,
    )
    .bind(store_id)
    .bind(email)
    .bind(password_hash)
    .bind(first_name)
    .bind(last_name)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Find user by email within a store
pub async fn find_user_by_email(pool: &PgPool, store_id: Uuid, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE store_id = $1 AND email = $2"
    )
    .bind(store_id)
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Find user by ID
pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
