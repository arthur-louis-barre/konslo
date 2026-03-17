use crate::errors::AppError;
use crate::models::check::{Check, CreateCheck, UpdateCheck};
use crate::repositories::check::CheckRepository;
use crate::validation::check::{validate_check_value, validate_checked_at};
use async_trait::async_trait;
use std::sync::Arc;

use crate::repositories::habit::HabitRepository;
#[cfg(any(test, feature = "mockable"))]
use mockall::automock;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait CheckService: Send + Sync {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError>;
    async fn update(&self, check: &UpdateCheck) -> Result<(), AppError>;
    async fn get_by_habit_id(&self, habit_id: i32) -> Result<Vec<Check>, AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct DefaultCheckService {
    check_repo: Arc<dyn CheckRepository>,
    habit_repo: Arc<dyn HabitRepository>,
}

impl DefaultCheckService {
    pub fn new(check_repo: Arc<dyn CheckRepository>, habit_repo: Arc<dyn HabitRepository>) -> Self {
        Self {
            check_repo,
            habit_repo,
        }
    }
}

#[async_trait]
impl CheckService for DefaultCheckService {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError> {
        let habit =
            self.habit_repo
                .get_by_id(new_check.habit_id)
                .await?
                .ok_or(AppError::NotFound(format!(
                    "habit {} not found",
                    new_check.habit_id
                )))?;

        validate_check_value(new_check.value, habit.goal_value)?;
        validate_checked_at(&new_check.checked_at)?;

        let check = self.check_repo.create(new_check).await?;

        Ok(check)
    }

    async fn update(&self, update_check: &UpdateCheck) -> Result<(), AppError> {
        let check = self
            .check_repo
            .get_by_id(update_check.id)
            .await?
            .ok_or(AppError::NotFound(format!(
                "check {} not found",
                update_check.id
            )))?;
        let habit = self
            .habit_repo
            .get_by_id(check.habit_id)
            .await?
            .ok_or(AppError::NotFound(format!(
                "habit {} not found",
                check.habit_id
            )))?;

        validate_check_value(update_check.value, habit.goal_value)?;

        self.check_repo.update(update_check).await?;

        Ok(())
    }

    async fn get_by_habit_id(&self, habit_id: i32) -> Result<Vec<Check>, AppError> {
        self.check_repo.get_by_habit_id(habit_id).await
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let deleted = self.check_repo.delete(id).await?;

        match deleted {
            true => Ok(()),
            false => Err(AppError::NotFound(format!("check {} not found", id))),
        }
    }
}

#[cfg(test)]
mod tests {

    // #[tokio::test]
    // async fn test_create_checks_returns_created_check() {
    //     // arrange
    //     let mut repo = MockCheckRepository::new();
    //     repo.expect_create().return_once(|new_check| {
    //         let habit_id = new_check.habit_id;
    //         let value = new_check.value;
    //         let checked_at = new_check.checked_at.clone();
    //         Box::pin(async move {
    //             Ok(Check {
    //                 id: 1,
    //                 habit_id,
    //                 value,
    //                 checked_at,
    //             })
    //         })
    //     });
    //
    //     let service = DefaultCheckService::new(Arc::new(repo));
    //
    //     let new_check = CreateCheck {
    //         habit_id: 25,
    //         value: 3,
    //         checked_at: OffsetDateTime::UNIX_EPOCH,
    //     };
    //
    //     // act
    //     let check = service.create(&new_check).await;
    //
    //     // assert
    //     assert!(check.is_ok());
    //     let check = check.unwrap();
    //     assert_eq!(check.id, 1);
    //     assert_eq!(check.habit_id, 25);
    //     assert_eq!(check.value, 3);
    //     assert_eq!(check.checked_at, OffsetDateTime::UNIX_EPOCH);
    // }

    // #[tokio::test]
    // async fn test_create_checks_invalid_input_returns_validation_error() {
    //     // arrange
    //     let mut repo = MockCheckRepository::new();
    //     repo.expect_create().times(0);
    //     let service = DefaultCheckService::new(Arc::new(repo));
    //     let new_check_invalid = CreateCheck {
    //         habit_id: 1,
    //         value: 0,
    //         checked_at: OffsetDateTime::now_utc(),
    //     };
    //
    //     // act
    //     let check = service.create(&new_check_invalid).await;
    //
    //     // assert
    //     assert!(check.is_err());
    //     assert!(matches!(check.unwrap_err(), AppError::Validation(_)));
    // }
}
