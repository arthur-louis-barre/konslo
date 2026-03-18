use crate::error::AppError::{BadRequest, Conflict, InternalServerError, NotFound};
use konslo_core::errors::AppError as CoreError;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AppError {
    BadRequest(String),
    Conflict(String),
    InternalServerError,
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            NotFound(msg) => (StatusCode::NOT_FOUND, format!("Not found: {msg}")),
            InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Server error".into()),
            BadRequest(msg) => (StatusCode::BAD_REQUEST, format!("BadRequest: {msg}")),
            Conflict(msg) => (StatusCode::CONFLICT, format!("Conflict: {msg}")),
        };

        (status, message).into_response()
    }
}

impl From<CoreError> for AppError {
    fn from(e: CoreError) -> Self {
        match e {
            CoreError::Validation(msg) => BadRequest(msg),
            CoreError::Database(_) => {
                tracing::error!("Database error: {:?}", e);
                InternalServerError
            },
            CoreError::Conflict(msg) => Conflict(msg),
            CoreError::NotFound(msg) => NotFound(msg),
        }
    }
}
