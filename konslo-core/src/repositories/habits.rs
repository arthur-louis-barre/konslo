use crate::errors::AppError;
use crate::models::habit::{CreateHabit, GoalPeriod, Habit};
use crate::repositories::to_app_error;
use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait HabitRepository: Send + Sync {
    async fn create(&self, new_habit: &CreateHabit) -> Result<Habit, AppError>;
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
    async fn create(&self, new_habit: &CreateHabit) -> Result<Habit, AppError> {
        let habit = query_as!(
            Habit,
            r#"
                INSERT INTO habits (name, goal_value, goal_unit, goal_period)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, goal_value, goal_unit, goal_period as "goal_period: GoalPeriod", created_at
            "#,
            new_habit.name,
            new_habit.goal_value,
            new_habit.goal_unit,
            new_habit.goal_period as GoalPeriod,
        )
            .fetch_one(&self.pool)
            .await
            .map_err(to_app_error)?;

        Ok(habit)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError> {
        let habit = query_as!(
            Habit,
            r#"
                SELECT id, name, goal_value, goal_unit, goal_period as "goal_period: GoalPeriod", created_at
                FROM habits WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(to_app_error)?;

        Ok(habit)
    }

    async fn get_all(&self) -> Result<Vec<Habit>, AppError> {
        let habits = query_as!(
            Habit,
            r#"
                SELECT id, name, goal_value, goal_unit, goal_period as "goal_period: GoalPeriod", created_at
                FROM habits ORDER BY id;
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(to_app_error)?;

        Ok(habits)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let result = query!(r#"DELETE FROM habits WHERE id = $1"#, id)
            .execute(&self.pool)
            .await
            .map_err(to_app_error)?;

        Ok(result.rows_affected() == 1)
    }
}