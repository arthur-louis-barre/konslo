use axum::extract::{Path, State};
use axum::{http, Json};
use konslo_core::models::check::{Check, CreateCheck};
use crate::error::AppError;
use crate::requests::CreateCheckRequest;
use crate::router::AppState;

pub async fn create_check_handler(
    State(state): State<AppState>,
    Path(habit_id): Path<i32>,
    Json(request): Json<CreateCheckRequest>,
) -> Result<(http::StatusCode, Json<Check>), AppError> {
    let new_check = CreateCheck {
        habit_id,
        value: request.value,
        checked_at: request.checked_at
    };
    let check = state.check_service.create(&new_check).await?;
    Ok((http::StatusCode::CREATED, Json(check)))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum::http;
    use axum_test::TestServer;
    use time::OffsetDateTime;
    use konslo_core::models::check::{Check, CreateCheck};
    use konslo_core::services::check::MockCheckService;
    use konslo_core::services::habit::MockHabitService;
    use crate::router::{get_router, AppState};

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
                Ok(Check {
                    id: 5,
                    habit_id,
                    value,
                    checked_at,
                })
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

        // assert
        assert_eq!(response.status_code(), http::StatusCode::CREATED);
        let body: Check = response.json();

        assert_eq!(body.id, 5);
        assert_eq!(body.habit_id, 25);
        assert_eq!(body.value, 5);
        assert_eq!(body.checked_at, OffsetDateTime::UNIX_EPOCH)
    }
}