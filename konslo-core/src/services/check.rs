use crate::errors::AppError;
use crate::models::check::{Check, CreateCheck};
use crate::repositories::check::CheckRepository;
use crate::validation::check::validate_check;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait CheckService: Send + Sync {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Check>, AppError>;
    async fn get_all(&self) -> Result<Vec<Check>, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

#[derive(Clone)]
pub struct DefaultCheckService {
    repo: Arc<dyn CheckRepository>,
}

impl DefaultCheckService {
    pub fn new(repo: Arc<dyn CheckRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl CheckService for DefaultCheckService {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError> {
        validate_check(new_check)?;
        let check = self.repo.create(&new_check).await?;
        Ok(check)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Check>, AppError> {
        let check = self.repo.get_by_id(id).await?;
        Ok(check)
    }

    async fn get_all(&self) -> Result<Vec<Check>, AppError> {
        let checks = self.repo.get_all().await?;
        Ok(checks)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let deleted = self.repo.delete(id).await?;
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::check::MockCheckRepository;
    use time::{OffsetDateTime};

    #[tokio::test]
    async fn test_create_checks_returns_created_check() {
        // arrange
        let mut repo = MockCheckRepository::new();
        repo.expect_create().return_once(|new_check| {
            let habit_id = new_check.habit_id;
            let value = new_check.value;
            let checked_at = new_check.checked_at.clone();
            Box::pin(async move {
                Ok(Check {
                    id: 1,
                    habit_id,
                    value,
                    checked_at,
                })
            })
        });

        let service = DefaultCheckService::new(Arc::new(repo));

        let new_check = CreateCheck {
            habit_id: 25,
            value: 3,
            checked_at: OffsetDateTime::UNIX_EPOCH,
        };

        // act
        let check = service.create(&new_check).await;

        // assert
        assert!(check.is_ok());
        let check = check.unwrap();
        assert_eq!(check.id, 1);
        assert_eq!(check.habit_id, 25);
        assert_eq!(check.value, 3);
        assert_eq!(check.checked_at, OffsetDateTime::UNIX_EPOCH);
    }

    #[tokio::test]
    async fn test_create_checks_invalid_input_returns_validation_error() {
        // arrange
        let mut repo = MockCheckRepository::new();
        repo.expect_create().times(0);
        let service = DefaultCheckService::new(Arc::new(repo));
        let new_check_invalid = CreateCheck {
            habit_id: 1,
            value: 0,
            checked_at: OffsetDateTime::now_utc(),
        };

        // act
        let check = service.create(&new_check_invalid).await;

        // assert
        assert!(check.is_err());
        assert!(matches!(check.unwrap_err(), AppError::Validation(_)));
    }
}
