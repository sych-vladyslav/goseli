use goseli_core::{error::ApiError, models::user::UserRole};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid, // user_id
    pub email: String,
    pub role: String, // UserRole as string
    pub store_id: Uuid,
    pub exp: i64, // expiration (Unix timestamp)
    pub iat: i64, // issued at (Unix timestamp)
}

fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-me-in-production-use-64-char-random-string".to_string())
}

fn get_access_token_expiry() -> i64 {
    std::env::var("JWT_ACCESS_TOKEN_EXPIRY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(900) // 15 minutes
}

fn get_refresh_token_expiry() -> i64 {
    std::env::var("JWT_REFRESH_TOKEN_EXPIRY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(604800) // 7 days
}

/// Generate an access token (short-lived, 15 min)
pub fn generate_access_token(
    user_id: Uuid,
    email: String,
    role: UserRole,
    store_id: Uuid,
) -> Result<String, ApiError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let expiry = now + get_access_token_expiry();

    let claims = Claims {
        sub: user_id,
        email,
        role: role.to_string(),
        store_id,
        exp: expiry,
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .map_err(|e| ApiError::internal(format!("Token generation failed: {}", e)))?;

    Ok(token)
}

/// Generate a refresh token (long-lived, 7 days)
pub fn generate_refresh_token(
    user_id: Uuid,
    email: String,
    role: UserRole,
    store_id: Uuid,
) -> Result<String, ApiError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let expiry = now + get_refresh_token_expiry();

    let claims = Claims {
        sub: user_id,
        email,
        role: role.to_string(),
        store_id,
        exp: expiry,
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .map_err(|e| ApiError::internal(format!("Token generation failed: {}", e)))?;

    Ok(token)
}

/// Validate a JWT token and extract claims
pub fn validate_token(token: &str) -> Result<Claims, ApiError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            ApiError::unauthorized("Token has expired")
        }
        _ => ApiError::unauthorized("Invalid token"),
    })?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let user_id = Uuid::now_v7();
        let email = "test@example.com".to_string();
        let role = UserRole::Customer;
        let store_id = Uuid::now_v7();

        let token = generate_access_token(user_id, email.clone(), role, store_id).unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, "customer");
        assert_eq!(claims.store_id, store_id);
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
