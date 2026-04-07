use crate::error::AppError;
use crate::jwt::generate_token;
use crate::requests::{LoginRequest, RegisterRequest};
use crate::router::AppState;
use axum::Json;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::response::IntoResponse;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.register(&request.email, &request.password).await?;

    let token = generate_token(user.id, state.jwt_secret.as_bytes())?;
    let cookie = format!("token={}; HttpOnly; Path=/; Max-Age=3600", token);

    Response::builder()
        .status(StatusCode::CREATED)
        .header(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
        .body(Body::empty())
        .map_err(|_| AppError::InternalServerError)
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.login(&request.email, &request.password).await?;

    let token = generate_token(user.id, state.jwt_secret.as_bytes())?;
    let cookie = format!("token={}; HttpOnly; Path=/; Max-Age=3600", token);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
        .body(Body::empty())
        .map_err(|_| AppError::InternalServerError)
}

pub async fn logout_handler() -> Result<impl IntoResponse, AppError> {
    let cookie = "token=; HttpOnly; Path=/; Max-Age=0".to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
        .body(Body::empty())
        .map_err(|_| AppError::InternalServerError)
}
