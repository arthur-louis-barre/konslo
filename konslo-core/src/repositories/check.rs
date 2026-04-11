use crate::errors::AppError;
use crate::models::check::{NewCheck, Check};
use async_trait::async_trait;
use sqlx::{PgPool, query_file, query_file_as};
use time::Date;
use uuid::Uuid;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), mockall::automock)]
pub trait CheckRepository: Send + Sync {
    async fn upsert(&self, new_check: &NewCheck) -> Result<Check, AppError>;
    async fn get_activity_dates(&self, user_id: Uuid, from: Date, to: Date) -> Result<Vec<Date>, AppError>;
    async fn delete_by_habit_for_period(&self, habit_id: Uuid, from: Date, to: Date) -> Result<u64, AppError>;
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
    async fn upsert(&self, new_check: &NewCheck) -> Result<Check, AppError> {
        let check = query_file_as!(
            Check,
            "queries/upsert_check.sql",
            new_check.habit_id,
            new_check.value,
            new_check.checked_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(check)
    }

    async fn get_activity_dates(&self, user_id: Uuid, from: Date, to: Date) -> Result<Vec<Date>, AppError> {
        let dates = query_file!("queries/select_activity_dates.sql", user_id, from, to)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .filter_map(|r| r.date)
            .collect();

        Ok(dates)
    }

    async fn delete_by_habit_for_period(&self, habit_id: Uuid, from: Date, to: Date) -> Result<u64, AppError> {
        let result = query_file!("queries/delete_check_by_habit_in_period.sql", habit_id, from, to)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
