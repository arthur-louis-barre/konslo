use crate::error::AppError;
use crate::extractor::AuthUser;
use crate::requests::{ActivityQuery, AddCheckRequest, CreateHabitRequest, HabitsQuery, ResetChecksQuery};
use crate::responses::{ActivityResponse, CheckResponse, HabitResponse, HabitWithCheckResponse};
use crate::router::AppState;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http;
use konslo_core::models::habit::CreateHabit;
use time::OffsetDateTime;

pub async fn create_habit_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<CreateHabitRequest>,
) -> Result<(http::StatusCode, Json<HabitResponse>), AppError> {
    let new_habit = CreateHabit {
        user_id: auth_user.user_id,
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
    auth_user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<HabitResponse>, AppError> {
    let habit = state.habit_service.get_by_id(id, auth_user.user_id).await?;

    match habit {
        Some(habit) => Ok(Json(habit.into())),
        None => Err(AppError::NotFound(format!("habit with id {} not found", id))),
    }
}

pub async fn delete_habit_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i32>,
) -> Result<http::StatusCode, AppError> {
    state.habit_service.delete(id, auth_user.user_id).await?;
    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn get_all_habits_with_period_checks_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<HabitsQuery>,
) -> Result<Json<Vec<HabitWithCheckResponse>>, AppError> {
    let habits_with_checks = state
        .habit_service
        .get_all_with_period_checks(auth_user.user_id, params.date.unwrap_or_else(OffsetDateTime::now_utc))
        .await?
        .into_iter()
        .map(|h| h.into())
        .collect();

    Ok(Json(habits_with_checks))
}

pub async fn add_check_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(habit_id): Path<i32>,
    Json(request): Json<AddCheckRequest>,
) -> Result<Json<CheckResponse>, AppError> {
    let check = state
        .habit_service
        .add_check(
            habit_id,
            auth_user.user_id,
            request.value,
            request.checked_at.unwrap_or_else(OffsetDateTime::now_utc),
        )
        .await?;

    Ok(Json(check.into()))
}

pub async fn reset_period_checks_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(habit_id): Path<i32>,
    Query(params): Query<ResetChecksQuery>,
) -> Result<http::StatusCode, AppError> {
    state
        .habit_service
        .reset_period_checks(habit_id, auth_user.user_id, params.date.unwrap_or_else(OffsetDateTime::now_utc))
        .await?;

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn get_activity_dates_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<ActivityQuery>,
) -> Result<Json<ActivityResponse>, AppError> {
    let dates = state.habit_service.get_activity_dates(auth_user.user_id, params.from, params.to).await?;

    Ok(Json(dates.into()))
}

#[cfg(test)]
mod tests {}
