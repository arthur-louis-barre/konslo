use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http;
use konslo_core::models::habit::{CreateHabit, Habit};
use konslo_core::services::habit::HabitService;
use std::sync::Arc;

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
    use konslo_core::models::habit::GoalPeriod;
    use konslo_core::services::habit::MockHabitService;
    use time::OffsetDateTime;

    #[tokio::test]
    async fn test_create_habits_handler_returns_habit() {
        // arrange
        let mut mock_service = MockHabitService::new();
        mock_service.expect_create().return_once(|_| {
            Box::pin(async move {
                Ok(Habit {
                    id: 25,
                    name: "Meditate".to_string(),
                    goal_value: 10,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Day,
                    created_at: OffsetDateTime::UNIX_EPOCH,
                })
            })
        });

        let app = get_router(Arc::new(mock_service));
        let server = TestServer::new(app);

        // act
        let response = server
            .post("/habits")
            .json(&serde_json::json!({
                "name": "Meditate",
                "goal_value": 10,
                "goal_unit": "min",
                "goal_period": "day"
            }))
            .await;

        // assert
        assert_eq!(response.status_code(), http::StatusCode::CREATED);

        let body: Habit = response.json();
        assert_eq!(body.id, 25);
        assert_eq!(body.name, "Meditate");
        assert_eq!(body.goal_value, 10);
        assert_eq!(body.goal_unit, "min");
        assert_eq!(body.goal_period, GoalPeriod::Day);
    }
}
