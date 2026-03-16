use crate::errors::AppError;
use crate::models::check::{Check, CreateCheck, UpdateCheck};
use crate::repositories::to_app_error;
use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait CheckRepository: Send + Sync {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError>;
    async fn get_by_habit_id(&self, habit_id: i32) -> Result<Vec<Check>, AppError>;
    async fn update(&self, check: &UpdateCheck) -> Result<bool, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

pub struct PostgresCheckRepository {
    pub pool: PgPool,
}

impl PostgresCheckRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresCheckRepository { pool }
    }
}

#[async_trait]
impl CheckRepository for PostgresCheckRepository {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError> {
        let check = query_as!(
            Check,
            r#"
                INSERT INTO checks(habit_id, value, checked_at)
                VALUES ($1, $2, $3)
                RETURNING check_id as id, habit_id, value, checked_at;
            "#,
            new_check.habit_id,
            new_check.value,
            new_check.checked_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(to_app_error)?;

        Ok(check)
    }

    async fn get_by_habit_id(&self, habit_id: i32) -> Result<Vec<Check>, AppError> {
        let checks = query_as!(
            Check,
            r#"
                SELECT check_id as id, habit_id, value, checked_at
                FROM checks
                WHERE habit_id = $1
                ORDER BY checked_at
            "#,
            habit_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(to_app_error)?;

        Ok(checks)
    }

    async fn update(&self, check: &UpdateCheck) -> Result<bool, AppError> {
        let result = query!(
            r#"
                UPDATE checks
                SET value = $1
                WHERE check_id = $2;
            "#,
            check.value,
            check.id,
        )
        .execute(&self.pool)
        .await
        .map_err(to_app_error)?;

        Ok(result.rows_affected() == 1)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let result = query!(r#"DELETE FROM checks WHERE check_id = $1"#, id)
            .execute(&self.pool)
            .await
            .map_err(to_app_error)?;

        Ok(result.rows_affected() == 1)
    }
}

impl PostgresCheckRepository {
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Check>, AppError> {
        let check = query_as!(
            Check,
            r#"
                SELECT check_id as id, habit_id, value, checked_at
                FROM checks WHERE check_id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(to_app_error)?;

        Ok(check)
    }
}
