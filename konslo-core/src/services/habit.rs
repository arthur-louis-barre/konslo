use crate::errors::AppError;
use crate::models::habit::{CreateHabit, Habit, HabitWithCheck};
use crate::repositories::habit::HabitRepository;
use crate::validation::habit::validate_habit_name;
use async_trait::async_trait;
use std::sync::Arc;
use time::OffsetDateTime;

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait HabitService: Send + Sync {
    async fn create(&self, new_habit: CreateHabit) -> Result<Habit, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError>;
    async fn get_all_with_checks_for(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

#[derive(Clone)]
pub struct DefaultHabitService {
    repo: Arc<dyn HabitRepository>,
}

impl DefaultHabitService {
    pub fn new(repo: Arc<dyn HabitRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl HabitService for DefaultHabitService {
    async fn create(&self, new_habit: CreateHabit) -> Result<Habit, AppError> {
        validate_habit_name(&new_habit.name)?;
        let habit = self.repo.create(&new_habit).await?;
        Ok(habit)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError> {
        let habit = self.repo.get_by_id(id).await?;
        Ok(habit)
    }

    async fn get_all_with_checks_for(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError> {
        let habits_with_checks = self.repo.get_all_with_checks_for(timestamp).await?;
        Ok(habits_with_checks)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let deleted = self.repo.delete(id).await?;
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::habit::GoalPeriod;
    use crate::repositories::habit::MockHabitRepository;
    use time::OffsetDateTime;

    fn make_habit() -> Habit {
        Habit {
            id: 1,
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
            created_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    mod create {
        use super::*;

        #[tokio::test]
        async fn test_ok() {
            let mut repo = MockHabitRepository::new();
            repo.expect_create()
                .return_once(|_| Box::pin(async { Ok(make_habit()) }));

            let service = DefaultHabitService::new(Arc::new(repo));
            let new_habit = CreateHabit {
                name: "Meditate".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            };

            let result = service.create(new_habit).await;

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), make_habit());
        }

        #[tokio::test]
        async fn test_invalid_name_returns_validation_error() {
            let mut repo = MockHabitRepository::new();
            repo.expect_create().times(0);

            let service = DefaultHabitService::new(Arc::new(repo));
            let new_habit = CreateHabit {
                name: "".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            };

            let result = service.create(new_habit).await;

            assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
        }
    }

    mod get_by_id {
        use super::*;

        #[tokio::test]
        async fn test_ok() {
            let mut repo = MockHabitRepository::new();
            repo.expect_get_by_id()
                .return_once(|_| Box::pin(async { Ok(Some(make_habit())) }));

            let service = DefaultHabitService::new(Arc::new(repo));

            let result = service.get_by_id(1).await;

            assert_eq!(result.unwrap(), Some(make_habit()));
        }

        #[tokio::test]
        async fn test_not_found_returns_none() {
            let mut repo = MockHabitRepository::new();
            repo.expect_get_by_id().return_once(|_| Box::pin(async { Ok(None) }));

            let service = DefaultHabitService::new(Arc::new(repo));

            let result = service.get_by_id(999).await;

            assert_eq!(result.unwrap(), None);
        }
    }

    mod get_all_with_checks_for {
        use super::*;

        #[tokio::test]
        async fn test_ok() {
            let mut repo = MockHabitRepository::new();
            repo.expect_get_all_with_checks_for()
                .return_once(|_| Box::pin(async { Ok(vec![]) }));

            let service = DefaultHabitService::new(Arc::new(repo));

            let result = service.get_all_with_checks_for(OffsetDateTime::UNIX_EPOCH).await;

            assert!(result.is_ok());
        }
    }

    mod delete {
        use super::*;

        #[tokio::test]
        async fn test_ok() {
            let mut repo = MockHabitRepository::new();
            repo.expect_delete().return_once(|_| Box::pin(async { Ok(true) }));

            let service = DefaultHabitService::new(Arc::new(repo));

            let result = service.delete(1).await;

            assert_eq!(result.unwrap(), true);
        }

        #[tokio::test]
        async fn test_not_found_returns_false() {
            let mut repo = MockHabitRepository::new();
            repo.expect_delete().return_once(|_| Box::pin(async { Ok(false) }));

            let service = DefaultHabitService::new(Arc::new(repo));

            let result = service.delete(999).await;

            assert_eq!(result.unwrap(), false);
        }
    }
}
