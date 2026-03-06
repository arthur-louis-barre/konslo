use crate::db::habits::HabitRepository;
use crate::model::Habit;
use std::sync::Arc;
use crate::errors::AppError;
use crate::validation::habit::validate_habit_name;

#[derive(Clone)]
pub struct HabitService {
    repo: Arc<dyn HabitRepository>,
}

impl HabitService {
    pub fn new(repo: Arc<dyn HabitRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, name: &str) -> Result<Habit, AppError> {
        validate_habit_name(name)?;
        let habit = self.repo.create(name).await?;
        Ok(habit)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError> {
        let habit = self.repo.get_by_id(id).await?;
        Ok(habit)
    }

    pub async fn get_all(&self) -> Result<Vec<Habit>, AppError> {
        let habits = self.repo.get_all().await?;
        Ok(habits)
    }

    pub async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let deleted = self.repo.delete(id).await?;
        Ok(deleted)
    }
}

#[cfg(test)]
mod test {
    use crate::db::habits::MockHabitRepository;
    use super::*;
    use crate::service::habit::HabitService;

    #[tokio::test]
    async fn test_create_returns_created_habit() {
        // Arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_create().returning(|name| {
            let name = name.to_string();
            Box::pin(async move { Ok(Habit::new(1, &*name)) })
        });
        let service = HabitService::new(Arc::new(repo));

        // Act
        let habit = service.create("Meditate").await;

        // Assert
        let habit = habit.expect("result should be Ok");
        assert_eq!(habit.id, 1);
        assert_eq!(habit.name, "Meditate");
    }

    #[tokio::test]
    async fn test_get_by_id_returns_habit_when_found() {
        // Arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_get_by_id().returning(|id| Box::pin(async move { Ok(Some(Habit::new(id, "Meditate"))) }));
        let service = HabitService::new(Arc::new(repo));

        // Act
        let habit = service.get_by_id(1).await;

        // Assert
        let habit = habit.expect("result should be Ok");
        let habit = habit.expect("option should be Some");
        assert_eq!(habit.id, 1);
        assert_eq!(habit.name, "Meditate");
    }

    #[tokio::test]
    async fn test_get_by_id_returns_none_when_not_found() {
        // Arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_get_by_id().
            return_once(|_| Box::pin(async { Ok(None) }));
        let service = HabitService::new(Arc::new(repo));

        // Act
        let habit = service.get_by_id(1).await;

        // Assert
        let habit = habit.expect("result should be Ok");
        assert!(habit.is_none(), "option should be None");
    }

    #[tokio::test]
    async fn test_delete_returns_true_when_found() {
        // Arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_delete().return_once(|_| Box::pin(async { Ok(true) }));
        let service = HabitService::new(Arc::new(repo));

        // Act
        let deleted = service.delete(1).await;

        // Assert
        let deleted = deleted.expect("result should be Ok");
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_delete_returns_false_when_not_found() {
        // Arrange
        let mut repo = MockHabitRepository::new();
        repo.expect_delete().return_once(|_| Box::pin(async { Ok(false) }));
        let service = HabitService::new(Arc::new(repo));

        // Act
        let deleted = service.delete(1).await;

        // Assert
        let deleted = deleted.expect("result should be Ok");
        assert!(!deleted);
    }
}
