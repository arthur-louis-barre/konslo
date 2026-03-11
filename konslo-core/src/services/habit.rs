use crate::errors::AppError;
use crate::models::habit::{Habit, CreateHabit};
use crate::repositories::habits::HabitRepository;
use crate::validation::habit::validate_habit_name;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;


#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait HabitService: Send + Sync {
    async fn create(&self, new_habit: CreateHabit) -> Result<Habit, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError>;
    async fn get_all(&self) -> Result<Vec<Habit>, AppError>;
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

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let deleted = self.repo.delete(id).await?;
        Ok(deleted)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::habits::MockHabitRepository;

    #[tokio::test]
    #[ignore]
    async fn test_create_returns_created_habit() {
        // arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_create().returning(|name| {
            let name = name.to_string();
            Box::pin(async move { Ok(Habit::new(1, &*name)) })
        });
        let service = DefaultHabitService::new(Arc::new(repo));

        // act
        let habit = service.create("Meditate").await;

        // assert
        let habit = habit.expect("result should be Ok");
        assert_eq!(habit.id, 1);
        assert_eq!(habit.name, "Meditate");
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_invalid_input_returns_validation_error() {
        // arrange
        let repo = MockHabitRepository::new();
        let service = DefaultHabitService::new(Arc::new(repo));

        // act
        let habit = service.create("").await;

        // assert
        assert!(habit.is_err());
        assert!(matches!(habit.unwrap_err(), AppError::Validation(_)));
    }
}
