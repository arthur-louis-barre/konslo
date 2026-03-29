use crate::errors::AppError;
use crate::models::check::{Check, CreateCheck, UpdateCheck};
use async_trait::async_trait;
use sqlx::{PgPool, query_file, query_file_as};
use time::{Date, OffsetDateTime};

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait CheckRepository: Send + Sync {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Check>, AppError>;
    async fn get_by_habit(&self, habit_id: i32) -> Result<Vec<Check>, AppError>;
    async fn get_by_habit_and_date(&self, habit_id: i32, date: Date) -> Result<Option<Check>, AppError>;
    async fn update(&self, check: &UpdateCheck) -> Result<Option<Check>, AppError>;
    async fn delete(&self, id: i32) -> Result<u64, AppError>;
    async fn delete_by_habit_for_period(&self, habit_id: i32, from: Date, to: Date) -> Result<u64, AppError>;
    async fn upsert(&self, habit_id: i32, value: i32, checked_at: OffsetDateTime) -> Result<Check, AppError>;
}

pub struct PostgresCheckRepository {
    pool: PgPool,
}

impl PostgresCheckRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresCheckRepository { pool }
    }
}

#[async_trait]
impl CheckRepository for PostgresCheckRepository {
    async fn create(&self, new_check: &CreateCheck) -> Result<Check, AppError> {
        let check = query_file_as!(
            Check,
            "queries/insert_check.sql",
            new_check.habit_id,
            new_check.value,
            new_check.checked_at,
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(check)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Check>, AppError> {
        let check = query_file_as!(Check, "queries/select_check_by_id.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(check)
    }

    async fn get_by_habit(&self, habit_id: i32) -> Result<Vec<Check>, AppError> {
        let checks = query_file_as!(Check, "queries/select_checks_by_habit_id.sql", habit_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(checks)
    }

    async fn get_by_habit_and_date(&self, habit_id: i32, date: Date) -> Result<Option<Check>, AppError> {
        let check = query_file_as!(Check, "queries/select_check_by_habit_and_date.sql", habit_id, date)
            .fetch_optional(&self.pool)
            .await?;

        Ok(check)
    }

    async fn update(&self, check: &UpdateCheck) -> Result<Option<Check>, AppError> {
        let check = query_file_as!(Check, "queries/update_check.sql", check.value, check.id,)
            .fetch_optional(&self.pool)
            .await?;

        Ok(check)
    }

    async fn delete(&self, id: i32) -> Result<u64, AppError> {
        let result = query_file!("queries/delete_check.sql", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    async fn delete_by_habit_for_period(&self, habit_id: i32, from: Date, to: Date) -> Result<u64, AppError> {
        let result = query_file!("queries/delete_check_by_habit_in_period.sql", habit_id, from, to)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    async fn upsert(&self, habit_id: i32, value: i32, checked_at: OffsetDateTime) -> Result<Check, AppError> {
        let check = query_file_as!(
            Check,
            "queries/upsert_check.sql",
            habit_id,
            value,
            checked_at,
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(check)
    }
}