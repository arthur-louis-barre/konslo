use crate::errors::AppError;
use crate::models::check::{AddCheck, Check};
use async_trait::async_trait;
use sqlx::{PgPool, query_file, query_file_as};
use time::Date;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), mockall::automock)]
pub trait CheckRepository: Send + Sync {
    async fn upsert(&self, add_check: &AddCheck) -> Result<Check, AppError>;
    async fn get_activity_dates(&self, from: Date, to: Date) -> Result<Vec<Date>, AppError>;
    async fn delete_by_habit_for_period(&self, habit_id: i32, from: Date, to: Date) -> Result<u64, AppError>;
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
    async fn upsert(&self, add_check: &AddCheck) -> Result<Check, AppError> {
        let check = query_file_as!(
            Check,
            "queries/upsert_check.sql",
            add_check.habit_id,
            add_check.value,
            add_check.checked_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(check)
    }

    async fn get_activity_dates(&self, from: Date, to: Date) -> Result<Vec<Date>, AppError> {
        let dates = query_file!("queries/select_activity_dates.sql", from, to)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .filter_map(|r| r.date)
            .collect();

        Ok(dates)
    }

    async fn delete_by_habit_for_period(&self, habit_id: i32, from: Date, to: Date) -> Result<u64, AppError> {
        let result = query_file!("queries/delete_check_by_habit_in_period.sql", habit_id, from, to)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
