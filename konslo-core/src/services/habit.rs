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
    async fn get_all(&self) -> Result<Vec<Habit>, AppError>;
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

    async fn get_all(&self) -> Result<Vec<Habit>, AppError> {
        let habits = self.repo.get_all().await?;
        Ok(habits)
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
mod test {
    use super::*;
    use crate::models::habit::GoalPeriod;
    use crate::repositories::habit::MockHabitRepository;
    use time::OffsetDateTime;

    #[tokio::test]
    async fn test_create_returns_created_habit() {
        // arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_create().returning(|new_habit| {
            let name = new_habit.name.clone();
            let goal_value = new_habit.goal_value;
            let goal_unit = new_habit.goal_unit.clone();
            let goal_period = new_habit.goal_period;
            Box::pin(async move {
                Ok(Habit {
                    id: 1,
                    name,
                    goal_value,
                    goal_unit,
                    goal_period,
                    created_at: OffsetDateTime::UNIX_EPOCH,
                })
            })
        });
        let service = DefaultHabitService::new(Arc::new(repo));
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };

        // act
        let habit = service.create(new_habit).await;

        // assert
        let habit = habit.expect("result should be Ok");
        assert_eq!(habit.id, 1);
        assert_eq!(habit.name, "Meditate");
        assert_eq!(habit.goal_value, 10);
        assert_eq!(habit.goal_unit, "min");
        assert_eq!(habit.goal_period, GoalPeriod::Day);
        assert_eq!(habit.created_at, OffsetDateTime::UNIX_EPOCH);
    }

    #[tokio::test]
    async fn test_create_invalid_input_returns_validation_error() {
        // arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_create().times(0);
        let service = DefaultHabitService::new(Arc::new(repo));
        let new_habit = CreateHabit {
            name: "".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };

        // act
        let habit = service.create(new_habit).await;

        // assert
        assert!(habit.is_err());
        assert!(matches!(habit.unwrap_err(), AppError::Validation(_)));
    }

    #[tokio::test]
    async fn test_get_all_with_checks_for_ok() {
        // arrange
        let mut repo = MockHabitRepository::new();
        repo
            .expect_get_all_with_checks_for()
            .return_once(|_| Box::pin(async { Ok(vec![]) }));
        let service = DefaultHabitService::new(Arc::new(repo));

        // act
        let result = service.get_all_with_checks_for(OffsetDateTime::UNIX_EPOCH).await;

        // assert
        assert!(result.is_ok());
    }
}