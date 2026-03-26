use crate::error::AppError;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http;
use time::OffsetDateTime;
use konslo_core::models::habit::CreateHabit;
use crate::requests::{CreateHabitRequest, HabitsQuery};
use crate::responses::{HabitResponse, HabitWithCheckResponse};
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

pub async fn get_all_with_checks_for_handler (
    State(state): State<AppState>,
    Query(params): Query<HabitsQuery>
) -> Result<Json<Vec<HabitWithCheckResponse>>, AppError> {
    let habits_with_checks = state.habit_service
        .get_all_with_checks_for(params.date.unwrap_or_else(OffsetDateTime::now_utc))
        .await?
        .into_iter()
        .map(|h| h.into())
        .collect();

    Ok(Json(habits_with_checks))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use axum_test::TestServer;
    use time::OffsetDateTime;
    use konslo_core::models::check::Check;
    use konslo_core::models::habit::{GoalPeriod, HabitWithCheck};
    use konslo_core::services::check::MockCheckService;
    use konslo_core::services::habit::MockHabitService;
    use crate::responses::{CheckResponse, GoalPeriodDto, HabitWithCheckResponse};
    use crate::router::{get_router, AppState};

    #[tokio::test]
    async fn test_get_all_with_checks_for_handler_ok() {
        // arrange
        let mut mock_habit_service = MockHabitService::new();
        let mock_check_service = MockCheckService::new();

        mock_habit_service
            .expect_get_all_with_checks_for()
            .return_once(|_| Box::pin(async {
                Ok(vec![HabitWithCheck {
                    id: 1,
                    name: "Meditate".to_string(),
                    goal_value: 10,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Day,
                    created_at: OffsetDateTime::UNIX_EPOCH,
                    checks: vec![Check { id: 1, habit_id: 1, value: 5, checked_at: OffsetDateTime::UNIX_EPOCH }],
                }])
            }));

        let server = TestServer::new(get_router(AppState {
            habit_service: Arc::new(mock_habit_service),
            check_service: Arc::new(mock_check_service),
        }));

        // act
        let response = server.get("/habits").await;

        // assert
        assert_eq!(response.status_code(), http::StatusCode::OK);
        let body: Vec<HabitWithCheckResponse> = response.json();
        assert_eq!(body, vec![HabitWithCheckResponse {
            id: 1,
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriodDto::Day,
            created_at: OffsetDateTime::UNIX_EPOCH,
            checks: vec![CheckResponse { id: 1, habit_id: 1, value: 5, checked_at: OffsetDateTime::UNIX_EPOCH }],
        }]);
    }
}