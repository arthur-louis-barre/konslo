use axum::extract::{Path, State};
use axum::{http, Json};
use konslo_core::models::check::{Check, CreateCheck};
use crate::error::AppError;
use crate::router::AppState;

pub async fn create_check_handler(
    State(state): State<AppState>,
    Path(habit_id): Path<i32>,
    Json(mut new_check): Json<CreateCheck>,
) -> Result<(http::StatusCode, Json<Check>), AppError> {
    unimplemented!()
}