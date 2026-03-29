use crate::errors::AppError;
use crate::models::habit::{CreateHabit, Habit, HabitWithCheck};
use crate::repositories::habit::HabitRepository;
use crate::validation::habit::validate_habit_name;
use async_trait::async_trait;
use std::sync::Arc;
use time::OffsetDateTime;

use crate::models::check::Check;
use crate::repositories::check::CheckRepository;
#[cfg(any(test, feature = "mockable"))]
use mockall::automock;
use crate::validation::check::validate_period_cap;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), automock)]
pub trait HabitService: Send + Sync {
    async fn create(&self, new_habit: CreateHabit) -> Result<Habit, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
    async fn get_all_with_period_checks(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError>;
    async fn add_check(&self, habit_id: i32, value: i32, timestamp: OffsetDateTime) -> Result<Check, AppError>;
    async fn reset_period_checks(&self, habit_id: i32, timestamp: OffsetDateTime) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct DefaultHabitService {
    habit_repo: Arc<dyn HabitRepository>,
    check_repo: Arc<dyn CheckRepository>,
}

impl DefaultHabitService {
    pub fn new(habit_repo: Arc<dyn HabitRepository>, check_repo: Arc<dyn CheckRepository>) -> Self {
        Self { habit_repo, check_repo }
    }
}

#[async_trait]
impl HabitService for DefaultHabitService {
    async fn create(&self, new_habit: CreateHabit) -> Result<Habit, AppError> {
        validate_habit_name(&new_habit.name)?;
        let habit = self.habit_repo.create(&new_habit).await?;
        Ok(habit)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, AppError> {
        let habit = self.habit_repo.get_by_id(id).await?;
        Ok(habit)
    }

    async fn get_all_with_period_checks(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError> {
        let habits_with_checks = self.habit_repo.get_all_with_period_checks(timestamp).await?;
        Ok(habits_with_checks)
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let deleted = self.habit_repo.delete(id).await?;
        if deleted {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("habit with id {} not found", id)))
        }
    }

    async fn add_check(&self, habit_id: i32, value: i32, timestamp: OffsetDateTime) -> Result<Check, AppError> {
        let habit_with_check = self
            .habit_repo
            .get_with_period_checks(habit_id, timestamp)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("habit with id {} not found", habit_id)))?;

        validate_period_cap(&habit_with_check, value)?;

        let check = self.check_repo.upsert(habit_id, value, timestamp).await?;

        Ok(check)
    }

    async fn reset_period_checks(&self, habit_id: i32, timestamp: OffsetDateTime) -> Result<(), AppError> {
        let habit_with_check = self
            .habit_repo
            .get_with_period_checks(habit_id, timestamp)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("habit with id {} not found", habit_id)))?;

        let period_start = habit_with_check.goal_period.get_period_start(timestamp);
        let period_end = habit_with_check.goal_period.get_period_end(timestamp);

        self.check_repo
            .delete_by_habit_for_period(habit_id, period_start, period_end)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}