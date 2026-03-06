use crate::model::Habit;
use async_trait::async_trait;
use sqlx::{Error, PgPool, query, query_as};

#[async_trait]
pub trait HabitRepository: Send + Sync {
    async fn create(&self, name: &str) -> Result<Habit, Error>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, Error>;
    async fn get_all(&self) -> Result<Vec<Habit>, Error>;
    async fn delete(&self, id: i32) -> Result<bool, Error>;
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
    async fn create(&self, name: &str) -> Result<Habit, Error> {
        let habit = query_as!(
            Habit,
            "INSERT INTO habits (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(habit)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, Error> {
        let habit = query_as!(Habit, "SELECT id, name FROM habits WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(habit)
    }

    async fn get_all(&self) -> Result<Vec<Habit>, Error> {
        let habits = query_as!(Habit, "SELECT id, name FROM habits ORDER BY id",)
            .fetch_all(&self.pool)
            .await?;

        Ok(habits)
    }

    async fn delete(&self, id: i32) -> Result<bool, Error> {
        let result = query!("DELETE FROM habits WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() == 1)
    }
}
