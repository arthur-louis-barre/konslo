use axum::extract::{Path, State};
use axum::{http, Json};
use axum::response::IntoResponse;
use konslo_core::models::check::{CreateCheck, UpdateCheck};
use crate::error::AppError;
use crate::requests::{CreateCheckRequest, UpdateCheckRequest};
use crate::responses::CheckResponse;
use crate::router::AppState;

pub async fn create_check_handler(
    State(state): State<AppState>,
    Path(habit_id): Path<i32>,
    Json(request): Json<CreateCheckRequest>,
) -> Result<(http::StatusCode, Json<CheckResponse>), AppError> {
    let new_check = CreateCheck { habit_id, value: request.value, checked_at: request.checked_at };
    let check: CheckResponse = state.check_service.create(&new_check).await?.into();
    Ok((http::StatusCode::CREATED, Json(check)))
}

pub async fn get_checks_by_habit_handler(
    State(state): State<AppState>,
    Path(habit_id): Path<i32>,
) -> Result<Json<Vec<CheckResponse>>, AppError> {
    let checks = state.check_service
        .get_by_habit_id(habit_id)
        .await?
        .into_iter()
        .map(|c| c.into())
        .collect();

    Ok(Json(checks))
}

pub async fn update_check_handler(
    State(state): State<AppState>,
    Path((_, check_id)): Path<(i32, i32)>,
    Json(request): Json<UpdateCheckRequest>
) -> Result<impl IntoResponse, AppError> {
    let update = UpdateCheck {id: check_id, value: request.value };
    let response = state.check_service.update(&update).await?;
    match response {
        Some(check) => Ok((http::StatusCode::OK, Json(CheckResponse::from(check))).into_response()),
        None => Ok(http::StatusCode::NO_CONTENT.into_response())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum_test::TestServer;
    use time::OffsetDateTime;
    use konslo_core::models::check::Check;
    use konslo_core::services::check::MockCheckService;
    use konslo_core::services::habit::MockHabitService;
    use crate::router::get_router;
    use super::*;

    #[tokio::test]
    async fn test_create_check_handler_ok() {
        // arrange
        let mut mock_check_service = MockCheckService::new();
        let mock_habit_service = MockHabitService::new();

        mock_check_service.expect_create().return_once(|new_check| {
            let habit_id = new_check.habit_id;
            let value = new_check.value;
            let checked_at = new_check.checked_at;
            Box::pin(async move {
                Ok(Check { id: 5, habit_id, value, checked_at, })
            })
        });

        let app = get_router(AppState {
                check_service: Arc::new(mock_check_service),
                habit_service: Arc::new(mock_habit_service),
            }
        );
        let server = TestServer::new(app);

        // act
        let response = server
            .post("/habits/25/checks")
            .json(&serde_json::json!({
                "value": 5,
                "checked_at": "1970-01-01T00:00:00Z",
            }))
            .await;
        let body: CheckResponse = response.json();

        // assert
        let excepted = CheckResponse { id: 5, habit_id: 25, value: 5, checked_at: OffsetDateTime::UNIX_EPOCH };
        assert_eq!(response.status_code(), http::StatusCode::CREATED);
        assert_eq!(body, excepted);
    }

    #[tokio::test]
    async fn test_get_checks_by_habit_handler_ok() {
        // arrange
        let mut mock_check_service = MockCheckService::new();
        let mock_habit_service = MockHabitService::new();

        mock_check_service.expect_get_by_habit_id().return_once(|habit_id| {
            Box::pin(async move { Ok(vec![
                Check { id: 1, habit_id, value: 5, checked_at: OffsetDateTime::UNIX_EPOCH },
                Check { id: 2, habit_id, value: 10, checked_at: OffsetDateTime::UNIX_EPOCH },
            ])})
        });

        let app = get_router(AppState {
            check_service: Arc::new(mock_check_service),
            habit_service: Arc::new(mock_habit_service),
        }
        );
        let server = TestServer::new(app);

        // act
        let response = server.get("/habits/25/checks").await;
        let body: Vec<CheckResponse> = response.json();

        // assert
        let expected = vec![
            CheckResponse { id: 1, habit_id: 25, value: 5, checked_at: OffsetDateTime::UNIX_EPOCH },
            CheckResponse { id: 2, habit_id: 25, value: 10, checked_at: OffsetDateTime::UNIX_EPOCH },
        ];
        assert_eq!(body, expected);
    }

    #[tokio::test]
    async fn test_update_check_handler_ok() {
        let mut mock_check_service = MockCheckService::new();
        let mock_habit_service = MockHabitService::new();

        mock_check_service.expect_update().return_once(|update| {
            let id = update.id;
            let value = update.value;
            Box::pin(async move {
                Ok(Some(Check { id, habit_id: 25, value, checked_at: OffsetDateTime::UNIX_EPOCH }))
            })
        });

        let server = TestServer::new(get_router(AppState {
            check_service: Arc::new(mock_check_service),
            habit_service: Arc::new(mock_habit_service),
        }));

        // act
        let response = server
            .put("/habits/25/checks/5")
            .json(&serde_json::json!({ "value": 10 }))
            .await;

        assert_eq!(response.status_code(), http::StatusCode::OK);
        let body: CheckResponse = response.json();
        assert_eq!(body.value, 10);
    }

    #[tokio::test]
    async fn test_update_check_handler_deleted() {
        let mut mock_check_service = MockCheckService::new();
        let mock_habit_service = MockHabitService::new();

        mock_check_service.expect_update().return_once(|_| {
            Box::pin(async move { Ok(None) })
        });

        let server = TestServer::new(get_router(AppState {
            check_service: Arc::new(mock_check_service),
            habit_service: Arc::new(mock_habit_service),
        }));

        let response = server
            .put("/habits/25/checks/5")
            .json(&serde_json::json!({ "value": 0 }))
            .await;

        assert_eq!(response.status_code(), http::StatusCode::NO_CONTENT);
    }
}