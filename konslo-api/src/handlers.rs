use std::sync::Arc;
use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use konslo_core::model::Habit;
use konslo_core::services::habit::{HabitService};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateHabitRequest {
    pub name: String,
}

pub async fn create_habits_handler(
    State(habit_service): State<Arc<dyn HabitService>>,
    Json(body): Json<CreateHabitRequest>,
) -> Result<(StatusCode, Json<Habit>), AppError> {
    let habit_name = body.name.as_str();
    let habit = habit_service.create(habit_name).await?;
    Ok((StatusCode::CREATED, Json(habit)))
}

pub async fn get_habit_handler(
    State(habit_service): State<Arc<dyn HabitService>>,
    Path(id): Path<i32>,
) -> Result<Json<Habit>, AppError> {
    let habit = habit_service.get_by_id(id).await?;
    match habit {
        Some(habit) => Ok(Json(habit)),
        None => Err(AppError::NotFound(format!("no habit with id {id}"))),
    }
}

pub async fn get_all_habits_handler(
    State(habit_service): State<Arc<dyn HabitService>>,
) -> Result<Json<Vec<Habit>>, AppError> {
    let habits = habit_service.get_all().await?;
    Ok(Json(habits))
}

pub async fn delete_habits_handler(
    State(habit_service): State<Arc<dyn HabitService>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let deleted = habit_service.delete(id).await?;
    match deleted {
        true => Ok(StatusCode::NO_CONTENT),
        false => Err(AppError::NotFound(format!("no habit with id {id}"))),
    }
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use konslo_core::services::habit::MockHabitService;
    use crate::router::get_router;
    use super::*;

    #[tokio::test]
    async fn test_create_habits_handler_returns_habit() {
        // arrange
        let mut mock_service = MockHabitService::new();
        mock_service
            .expect_create()
            .return_once(|name| {
                let name = name.to_string();
                Box::pin(async move { Ok(Habit::new(1, &*name)) })
            });
        let app = get_router(Arc::new(mock_service));
        let server = TestServer::new(app);

        // act
        let response = server
            .post("/habits")
            .json(&serde_json::json!({ "name": "Meditate" }))
            .await;

        // assert
        assert_eq!(response.status_code(), StatusCode::CREATED);
    }
}