use std::collections::HashMap;
use crate::errors::AppError;
use crate::models::habit::{CreateHabit, GoalPeriod, Habit, HabitWithCheck};
use async_trait::async_trait;
use sqlx::{PgPool, query, query_as, Row};

#[cfg(any(test, feature = "mockable"))]
use mockall::automock;
use crate::models::check::{Check};

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait HabitRepository: Send + Sync {
    async fn create(&self, new_habit: &CreateHabit) -> Result<Habit, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError>;
    async fn get_all(&self) -> Result<Vec<Habit>, AppError>;
    async fn get_all_with_today_checks(&self) -> Result<Vec<HabitWithCheck>, AppError>;
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
                RETURNING habit_id as id, name, goal_value, goal_unit, goal_period as "goal_period: GoalPeriod", created_at;
            "#,
            new_habit.name,
            new_habit.goal_value,
            new_habit.goal_unit,
            new_habit.goal_period as GoalPeriod,
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(habit)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError> {
        let habit = query_as!(
            Habit,
            r#"
                SELECT habit_id as id, name, goal_value, goal_unit, goal_period as "goal_period: GoalPeriod", created_at
                FROM habits WHERE habit_id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(habit)
    }

    async fn get_all(&self) -> Result<Vec<Habit>, AppError> {
        let habits = query_as!(
            Habit,
            r#"
                SELECT habit_id as id, name, goal_value, goal_unit, goal_period as "goal_period: GoalPeriod", created_at
                FROM habits ORDER BY habit_id;
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(habits)
    }

    async fn get_all_with_today_checks(&self) -> Result<Vec<HabitWithCheck>, AppError> {
        let rows = query(
            r#"
                    SELECT
                        h.habit_id, h.name, h.goal_value, h.goal_unit, h.goal_period, h.created_at,
                        c.check_id, c.value, c.checked_at
                    FROM habits h
                        LEFT JOIN checks c ON h.habit_id = c.habit_id AND c.checked_at::DATE = CURRENT_DATE
                    WHERE
                        c.checked_at::DATE = CURRENT_DATE
                        OR c.check_id IS NULL
                    ORDER BY habit_id
                "#
            ).fetch_all(&self.pool)
            .await?;

        let mut map: HashMap<i32, HabitWithCheck> = HashMap::new();

        for row in rows.iter() {
            let habit_id: i32 = row.get("habit_id");
            if !map.contains_key(&habit_id) {
                map.insert(habit_id.clone(), HabitWithCheck {
                    id: habit_id.clone(),
                    name: row.get("name"),
                    goal_value: row.get("goal_value"),
                    goal_unit: row.get("goal_unit"),
                    goal_period: row.get("goal_period"),
                    created_at: row.get("created_at"),
                    checks: vec![],
                });
            }

            let check_id: Option<i32> = row.get("check_id");
            if let Some(check_id) = check_id {
                map.entry(habit_id.clone()).and_modify(|h| h.checks.push(
                    Check {
                        id: check_id.clone(),
                        habit_id: habit_id.clone(),
                        value: row.get("value"),
                        checked_at: row.get("checked_at"),
                    }
                ));
            }
        }

        let mut result: Vec<HabitWithCheck> = map.into_values().collect();
        result.sort_by_key(|h| h.id);

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let result = query!(r#"DELETE FROM habits WHERE habit_id = $1"#, id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() == 1)
    }
}
