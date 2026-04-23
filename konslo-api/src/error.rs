use crate::error::AppError::{BadRequest, Conflict, ServerError, NotFound, Unauthorized};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use konslo_core::error::AppError as CoreError;

pub enum AppError {
    BadRequest(String),
    Conflict(String),
    ServerError,
    NotFound(String),
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Conflict(msg) => (StatusCode::CONFLICT, format!("Conflict: {msg}")),
            BadRequest(msg) => (StatusCode::BAD_REQUEST, format!("BadRequest: {msg}")),
            ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
            NotFound(msg) => (StatusCode::NOT_FOUND, format!("Not found: {msg}")),
            Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
        };

        (status, message).into_response()
    }
}

impl From<CoreError> for AppError {
    fn from(e: CoreError) -> Self {
        match e {
            CoreError::Conflict(msg) => Conflict(msg),
            CoreError::Database(msg) => {
                tracing::error!("{}", msg);
                ServerError
            }
            CoreError::Internal(msg) => {
                tracing::error!("{}", msg);
                ServerError
            }
            CoreError::NotFound(msg) => NotFound(msg),
            CoreError::Unauthorized => Unauthorized,
            CoreError::Validation(msg) => BadRequest(msg),
        }
    }
}
