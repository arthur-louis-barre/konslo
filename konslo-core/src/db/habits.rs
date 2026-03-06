use crate::model::Habit;
use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};
use crate::db::to_app_error;
use crate::errors::AppError;

#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait HabitRepository: Send + Sync {
    async fn create(&self, name: &str) -> Result<Habit, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError>;
    async fn get_all(&self) -> Result<Vec<Habit>, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

pub struct PostgresHabitRepository {
    pub pool: PgPool,
}

impl PostgresHabitRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresHabitRepository { pool }
    }
}

#[async_trait]
impl HabitRepository for PostgresHabitRepository {
    async fn create(&self, name: &str) -> Result<Habit, AppError> {
        let habit = query_as!(
            Habit,
            "INSERT INTO habits (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await.map_err(to_app_error)?;

        Ok(habit)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError> {
        let habit = query_as!(Habit, "SELECT id, name FROM habits WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await.map_err(to_app_error)?;

        Ok(habit)
    }

    async fn get_all(&self) -> Result<Vec<Habit>, AppError> {
        let habits = query_as!(Habit, "SELECT id, name FROM habits ORDER BY id",)
            .fetch_all(&self.pool)
            .await.map_err(to_app_error)?;

        Ok(habits)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let result = query!("DELETE FROM habits WHERE id = $1", id)
            .execute(&self.pool)
            .await.map_err(to_app_error)?;

        Ok(result.rows_affected() == 1)
    }
}