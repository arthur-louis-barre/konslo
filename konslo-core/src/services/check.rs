use crate::errors::AppError;
use crate::models::check::{Check, CreateCheck, UpdateCheck};
use crate::repositories::check::CheckRepository;
use crate::repositories::habit::HabitRepository;
use crate::validation::check::{validate_check_value, validate_checked_at};
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait CheckService: Send + Sync {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError>;
    async fn update(&self, check: &UpdateCheck) -> Result<Option<Check>, AppError>;
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
        Self { check_repo, habit_repo }
    }
}

#[async_trait]
impl CheckService for DefaultCheckService {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError> {
        let habit = self
            .habit_repo
            .get_by_id(new_check.habit_id)
            .await?
            .ok_or(AppError::NotFound(format!("habit {} not found", new_check.habit_id)))?;

        validate_check_value(new_check.value, habit.goal_value)?;
        validate_checked_at(&new_check.checked_at)?;

        let check = self.check_repo.create(new_check).await?;

        Ok(check)
    }

    async fn update(&self, update_check: &UpdateCheck) -> Result<Option<Check>, AppError> {
        let check = self
            .check_repo
            .get_by_id(update_check.id)
            .await?
            .ok_or(AppError::NotFound(format!("check {} not found", update_check.id)))?;

        let habit = self
            .habit_repo
            .get_by_id(check.habit_id)
            .await?
            .ok_or(AppError::NotFound(format!("habit {} not found", check.habit_id)))?;

        validate_check_value(update_check.value, habit.goal_value)?;

        if update_check.value == 0 {
            self.check_repo.delete(update_check.id).await?;
            return Ok(None);
        };

        self.check_repo
            .update(update_check)
            .await?
            .ok_or(AppError::NotFound(format!("check {} not found", update_check.id)))
            .map(Some)
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
    use super::*;
    use crate::models::habit::{GoalPeriod, Habit};
    use crate::repositories::check::MockCheckRepository;
    use crate::repositories::habit::MockHabitRepository;
    use std::sync::Arc;
    use time::{Duration, OffsetDateTime};

    fn make_habit(goal_value: i32) -> Habit {
        Habit {
            id: 1,
            name: "Meditate".to_string(),
            goal_value,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
            created_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    fn make_check(value: i32) -> Check {
        Check {
            id: 1,
            habit_id: 1,
            value,
            checked_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    mod create {
        use super::*;

        #[tokio::test]
        async fn test_ok() {
            let mut habit_repo = MockHabitRepository::new();
            let mut check_repo = MockCheckRepository::new();

            habit_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_habit(10))) }));
            check_repo
                .expect_create()
                .return_once(|_| Box::pin(async { Ok(make_check(5)) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let new_check = CreateCheck {
                habit_id: 1,
                value: 5,
                checked_at: OffsetDateTime::UNIX_EPOCH,
            };

            let result = service.create(&new_check).await;

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), make_check(5));
        }

        #[tokio::test]
        async fn test_habit_not_found_returns_not_found_error() {
            let mut habit_repo = MockHabitRepository::new();
            let check_repo = MockCheckRepository::new();

            habit_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(None) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let new_check = CreateCheck {
                habit_id: 999,
                value: 5,
                checked_at: OffsetDateTime::UNIX_EPOCH,
            };

            let result = service.create(&new_check).await;

            assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
        }

        #[tokio::test]
        async fn test_invalid_value_returns_validation_error() {
            let mut habit_repo = MockHabitRepository::new();
            let check_repo = MockCheckRepository::new();

            habit_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_habit(10))) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let new_check = CreateCheck {
                habit_id: 1,
                value: 999,
                checked_at: OffsetDateTime::UNIX_EPOCH,
            };

            let result = service.create(&new_check).await;

            assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
        }

        #[tokio::test]
        async fn test_invalid_checked_at_returns_validation_error() {
            let mut habit_repo = MockHabitRepository::new();
            let check_repo = MockCheckRepository::new();

            habit_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_habit(10))) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let new_check = CreateCheck {
                habit_id: 1,
                value: 5,
                checked_at: OffsetDateTime::now_utc() + Duration::days(1),
            };

            let result = service.create(&new_check).await;

            assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
        }
    }

    mod update {
        use super::*;

        #[tokio::test]
        async fn test_ok_value_positive_returns_check() {
            let mut habit_repo = MockHabitRepository::new();
            let mut check_repo = MockCheckRepository::new();

            check_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_check(5))) }));
            habit_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_habit(10))) }));
            check_repo
                .expect_update()
                .return_once(|_| Box::pin(async { Ok(Some(make_check(8))) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let update = UpdateCheck { id: 1, value: 8 };

            let result = service.update(&update).await;

            assert_eq!(result.unwrap(), Some(make_check(8)));
        }

        #[tokio::test]
        async fn test_ok_value_zero_deletes_and_returns_none() {
            let mut habit_repo = MockHabitRepository::new();
            let mut check_repo = MockCheckRepository::new();

            check_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_check(5))) }));
            habit_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_habit(10))) }));
            check_repo
                .expect_delete()
                .return_once(|_| Box::pin(async { Ok(true) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let update = UpdateCheck { id: 1, value: 0 };

            let result = service.update(&update).await;

            assert_eq!(result.unwrap(), None);
        }

        #[tokio::test]
        async fn test_check_not_found_returns_not_found_error() {
            let mut check_repo = MockCheckRepository::new();
            let habit_repo = MockHabitRepository::new();

            check_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(None) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let update = UpdateCheck { id: 999, value: 5 };

            let result = service.update(&update).await;

            assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
        }

        #[tokio::test]
        async fn test_invalid_value_returns_validation_error() {
            let mut habit_repo = MockHabitRepository::new();
            let mut check_repo = MockCheckRepository::new();

            check_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_check(5))) }));
            habit_repo
                .expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_habit(10))) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));
            let update = UpdateCheck { id: 1, value: 999 };

            let result = service.update(&update).await;

            assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
        }
    }

    mod delete {
        use super::*;

        #[tokio::test]
        async fn test_ok() {
            let mut check_repo = MockCheckRepository::new();
            let habit_repo = MockHabitRepository::new();

            check_repo.expect_delete().return_once(|_| Box::pin(async { Ok(true) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));

            let result = service.delete(1).await;

            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_check_not_found_returns_not_found_error() {
            let mut check_repo = MockCheckRepository::new();
            let habit_repo = MockHabitRepository::new();

            check_repo
                .expect_delete()
                .return_once(|_| Box::pin(async { Ok(false) }));

            let service = DefaultCheckService::new(Arc::new(check_repo), Arc::new(habit_repo));

            let result = service.delete(99).await;

            assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
        }
    }
}