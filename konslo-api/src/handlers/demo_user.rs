use crate::auth::jwt::generate_token;
use crate::error::AppError;
use crate::router::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::response::IntoResponse;
use time::{Duration, OffsetDateTime};

pub async fn register_demo_user_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    state
        .demo_user_service
        .delete_all_older_than(OffsetDateTime::now_utc() - Duration::days(1))
        .await?;
    let user = state.demo_user_service.create().await?;

    tracing::info!("demo login");

    let token = generate_token(user.id, state.jwt_secret.as_bytes())?;
    let cookie = format!(
        "token={}; HttpOnly; SameSite=None; Secure; Path=/; Max-Age=3600",
        token
    );

    Response::builder()
        .status(StatusCode::CREATED)
        .header(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie).map_err(|_| AppError::ServerError)?,
        )
        .body(Body::empty())
        .map_err(|_| AppError::ServerError)
}
