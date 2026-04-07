use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, StatusCode};
use uuid::Uuid;
use crate::jwt::verify_token;
use crate::router::AppState;

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let cookie_header = parts.headers
            .get(header::COOKIE)
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let cookie_str = cookie_header
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let token = cookie_str
            .split(';')
            .map(|s| s.trim())
            .find(|s| s.starts_with("token="))
            .and_then(|s| s.strip_prefix("token="))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        verify_token(token, state.jwt_secret.as_bytes())
            .map(|claims| AuthUser { user_id: claims.user_id})
            .map_err(|_| StatusCode::UNAUTHORIZED)
    }
}