use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http;
use time::OffsetDateTime;
use konslo_core::models::habit::CreateHabit;
use crate::requests::{AddCheckRequest, CreateHabitRequest, HabitsQuery, ResetChecksQuery};
use crate::responses::{CheckResponse, HabitResponse, HabitWithCheckResponse};
use crate::router::AppState;

pub async fn create_habit_handler (
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
        None => Err(AppError::NotFound(format!("habit with id {} not found", id))),
    }
}

pub async fn delete_habit_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<http::StatusCode, AppError> {
    state.habit_service.delete(id).await?;

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn get_all_habits_with_period_checks_handler (
    State(state): State<AppState>,
    Query(params): Query<HabitsQuery>
) -> Result<Json<Vec<HabitWithCheckResponse>>, AppError> {
    let habits_with_checks = state.habit_service
        .get_all_with_period_checks(params.date.unwrap_or_else(OffsetDateTime::now_utc))
        .await?
        .into_iter()
        .map(|h| h.into())
        .collect();

    Ok(Json(habits_with_checks))
}

pub async fn add_check_handler(
    State(state): State<AppState>,
    Path(habit_id): Path<i32>,
    Json(request): Json<AddCheckRequest>
) -> Result<Json<CheckResponse>, AppError> {
    let check = state.habit_service
        .add_check(habit_id, request.value,request.checked_at.unwrap_or_else(OffsetDateTime::now_utc))
        .await?;

    Ok(Json(check.into()))
}

pub async fn reset_period_checks_handler(
    State(state): State<AppState>,
    Path(habit_id): Path<i32>,
    Query(params): Query<ResetChecksQuery>
) -> Result<http::StatusCode, AppError> {
    state.habit_service
        .reset_period_checks(habit_id, params.date.unwrap_or_else(OffsetDateTime::now_utc))
        .await?;

    Ok(http::StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {}