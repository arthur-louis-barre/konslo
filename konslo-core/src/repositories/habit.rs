use crate::errors::AppError;
use crate::models::check::Check;
use crate::models::habit::{CreateHabit, GoalPeriod, Habit, HabitWithCheck};
use async_trait::async_trait;
use sqlx::{PgPool, query_file, query_file_as};
use std::collections::HashMap;
use time::OffsetDateTime;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), mockall::automock)]
pub trait HabitRepository: Send + Sync {
    async fn create(&self, new_habit: &CreateHabit) -> Result<Habit, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError>;
    async fn get_with_period_checks(&self, id: i32, timestamp: OffsetDateTime) -> Result<Option<HabitWithCheck>, AppError>;
    async fn get_all_with_period_checks(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

pub struct PostgresHabitRepository {
    pool: PgPool,
}

impl PostgresHabitRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresHabitRepository { pool }
    }
}

#[async_trait]
impl HabitRepository for PostgresHabitRepository {
    async fn create(&self, new_habit: &CreateHabit) -> Result<Habit, AppError> {
        let habit = query_file_as!(
            Habit,
            "queries/insert_habit.sql",
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
        let habit = query_file_as!(Habit, "queries/select_habit_by_id.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(habit)
    }

    async fn get_with_period_checks(&self, habit_id: i32, timestamp: OffsetDateTime) -> Result<Option<HabitWithCheck>, AppError> {
        let rows = query_file!("queries/select_habit_with_period_checks.sql", habit_id, timestamp)
            .fetch_all(&self.pool)
            .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let mut habit_with_checks = {
            let row = &rows[0];
            HabitWithCheck {
                id: row.id,
                name: row.name.clone(),
                goal_value: row.goal_value,
                goal_unit: row.goal_unit.clone(),
                goal_period: row.goal_period,
                created_at: row.created_at,
                checks: vec![],
            }
        };

        for row in rows {
            let check_id = row.check_id;
            if let Some(check_id) = check_id {
                habit_with_checks.checks.push(Check {
                    id: check_id,
                    habit_id,
                    value: row.value.unwrap(),
                    checked_at: row.checked_at.unwrap(),
                });
            }
        }

        Ok(Some(habit_with_checks))
    }

    async fn get_all_with_period_checks(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError> {
        let rows = query_file!("queries/select_habits_with_period_checks.sql", timestamp)
            .fetch_all(&self.pool)
            .await?;

        let mut map: HashMap<i32, HabitWithCheck> = HashMap::new();

        for row in rows {
            let id = row.id;
            let check_id = row.check_id;

            map.entry(id).or_insert_with(|| HabitWithCheck {
                id,
                name: row.name,
                goal_value: row.goal_value,
                goal_unit: row.goal_unit,
                goal_period: row.goal_period,
                created_at: row.created_at,
                checks: vec![],
            });

            if let Some(check_id) = check_id {
                map.entry(id).and_modify(|h| {
                    h.checks.push(Check {
                        id: check_id,
                        habit_id: id,
                        value: row.value.unwrap(),
                        checked_at: row.checked_at.unwrap(),
                    })
                });
            }
        }

        let mut habits_with_checks = map.into_values().collect::<Vec<HabitWithCheck>>();
        habits_with_checks.sort_by_key(|h| h.id);

        Ok(habits_with_checks)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let result = query_file!("queries/delete_habit.sql", id).execute(&self.pool).await?;

        Ok(result.rows_affected() == 1)
    }
}
