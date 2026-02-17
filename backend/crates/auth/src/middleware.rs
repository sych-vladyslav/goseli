use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use goseli_core::error::ApiError;
use uuid::Uuid;

use crate::jwt::{validate_token, Claims};

/// Authenticated user information extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
    pub store_id: Uuid,
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            email: claims.email,
            role: claims.role,
            store_id: claims.store_id,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?;

        // Parse Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header format".to_string(),
            ))?;

        // Validate token and extract claims
        let claims = validate_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

        Ok(AuthUser::from(claims))
    }
}
