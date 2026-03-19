use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http;
use konslo_core::models::habit::CreateHabit;
use crate::requests::CreateHabitRequest;
use crate::responses::HabitResponse;
use crate::router::AppState;

#[axum::debug_handler]
pub async fn create_habits_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateHabitRequest>,
) -> Result<(http::StatusCode, Json<HabitResponse>), AppError> {
    let new_habit = CreateHabit {
        name: request.name,
        goal_value: request.goal_value,
        goal_unit: request.goal_unit,
        goal_period: request.goal_period.into(),
    };
    let habit = state.habit_service.create(new_habit).await?;
    Ok((http::StatusCode::CREATED, Json(habit.into())))
}

pub async fn get_habit_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<HabitResponse>, AppError> {
    let habit = state.habit_service.get_by_id(id).await?;
    match habit {
        Some(habit) => Ok(Json(habit.into())),
        None => Err(AppError::NotFound(format!("no habit with id {id}"))),
    }
}

pub async fn get_all_habits_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<HabitResponse>>, AppError> {
    let habits = state.habit_service
        .get_all()
        .await?
        .into_iter()
        .map(|h| h.into())
        .collect();

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