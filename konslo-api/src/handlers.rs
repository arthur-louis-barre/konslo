use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use konslo_core::model::Habit;
use konslo_core::service::habit::HabitService;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateHabitRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct DeleteHabitRequest {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct GetHabitRequest {
    pub id: i32,
}

pub async fn get_habit_handler(
    State(habit_service): State<HabitService>,
    Path(id): Path<i32>,
) -> Result<Json<Habit>, AppError> {
    let habit = habit_service.get_by_id(id).await;
    match habit {
        Ok(Some(habit)) => Ok(Json(habit)),
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(e)?,
    }
}

pub async fn get_habits_handler(
    State(habit_service): State<HabitService>,
) -> Result<Json<Vec<Habit>>, AppError> {
    let habits = habit_service.get_all().await?;
    Ok(Json(habits))
}

pub async fn create_habits_handler(
    State(habit_service): State<HabitService>,
    Json(body): Json<CreateHabitRequest>,
) -> Result<Json<Habit>, AppError> {
    let habit_name = body.name.as_str();
    let habit = habit_service.create(habit_name).await?;
    Ok(Json(habit))
}

pub async fn delete_habits_handler(
    State(habit_service): State<HabitService>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let deleted = habit_service.delete(id).await;
    match deleted {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(AppError::NotFound),
        Err(e) => Err(e)?,
    }
}
