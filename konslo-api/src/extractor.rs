use crate::error::AppError;
use crate::jwt::{Claims, verify_token};
use crate::router::AppState;
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use uuid::Uuid;

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let cookie_header = parts.headers.get(header::COOKIE).ok_or(AppError::Unauthorized)?;

        let cookie_str = cookie_header.to_str().map_err(|_| AppError::Unauthorized)?;

        let token = cookie_str
            .split(';')
            .map(|s| s.trim())
            .find(|s| s.starts_with("token="))
            .and_then(|s| s.strip_prefix("token="))
            .ok_or(AppError::Unauthorized)?;

        verify_token(token, state.jwt_secret.as_bytes()).map(|Claims { user_id, .. }| AuthUser { user_id })
    }
}
