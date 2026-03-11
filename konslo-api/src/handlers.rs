use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http;
use konslo_core::services::habit::HabitService;
use std::sync::Arc;
use konslo_core::models::habit::{CreateHabit, Habit};

pub async fn create_habits_handler(
    State(habit_service): State<Arc<dyn HabitService>>,
    Json(new_habit): Json<CreateHabit>,
) -> Result<(http::StatusCode, Json<Habit>), AppError> {
    let habit = habit_service.create(new_habit).await?;
    Ok((http::StatusCode::CREATED, Json(habit)))
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
) -> Result<http::StatusCode, AppError> {
    let deleted = habit_service.delete(id).await?;
    match deleted {
        true => Ok(http::StatusCode::NO_CONTENT),
        false => Err(AppError::NotFound(format!("no habit with id {id}"))),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::router::get_router;
    use axum_test::TestServer;
    use konslo_core::services::habit::MockHabitService;

    #[tokio::test]
    async fn test_create_habits_handler_returns_habit() {
        // arrange
        let mut mock_service = MockHabitService::new();
        mock_service.expect_create().return_once(|name| {
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
        assert_eq!(response.status_code(), http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_habits_handler_invalid_input_returns_invalid_input_returns_400() {
        let mut mock_service = MockHabitService::new();
        mock_service.expect_create().return_once(|_| {
            Box::pin(async move { Err(CoreError::Validation("habit name is empty".to_string())) })
        });
        let app = get_router(Arc::new(mock_service));
        let server = TestServer::new(app);

        // act
        let response = server
            .post("/habits")
            .json(&serde_json::json!({ "name": "" }))
            .await;

        // assert
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_habits_handler_duplicate_returns_409() {
        let mut mock_service = MockHabitService::new();
        mock_service
            .expect_create()
            .return_once(|_| Box::pin(async move { Err(CoreError::Conflict("".to_string())) }));
        let app = get_router(Arc::new(mock_service));
        let server = TestServer::new(app);

        // act
        let response = server
            .post("/habits")
            .json(&serde_json::json!({ "name": "" }))
            .await;

        // assert
        assert_eq!(response.status_code(), StatusCode::CONFLICT);
    }
}
