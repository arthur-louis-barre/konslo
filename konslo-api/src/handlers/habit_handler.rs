use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http;
use konslo_core::models::habit::{CreateHabit, Habit};
use crate::router::AppState;

pub async fn create_habits_handler(
    State(state): State<AppState>,
    Json(new_habit): Json<CreateHabit>,
) -> Result<(http::StatusCode, Json<Habit>), AppError> {
    let habit = state.habit_service.create(new_habit).await?;
    Ok((http::StatusCode::CREATED, Json(habit)))
}

pub async fn get_habit_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Habit>, AppError> {
    let habit = state.habit_service.get_by_id(id).await?;
    match habit {
        Some(habit) => Ok(Json(habit)),
        None => Err(AppError::NotFound(format!("no habit with id {id}"))),
    }
}

pub async fn get_all_habits_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Habit>>, AppError> {
    let habits = state.habit_service.get_all().await?;
    Ok(Json(habits))
}

pub async fn delete_habits_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<http::StatusCode, AppError> {
    let deleted = state.habit_service.delete(id).await?;
    match deleted {
        true => Ok(http::StatusCode::NO_CONTENT),
        false => Err(AppError::NotFound(format!("no habit with id {id}"))),
    }
}