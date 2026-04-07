use crate::errors::AppError;
use crate::models::check::{AddCheck, Check};
use crate::models::habit::{CreateHabit, Habit, HabitWithCheck};
use crate::repositories::{CheckRepository, HabitRepository};
use crate::validation::check::{validate_check_checked_at, validate_check_value, validate_date_range, validate_period_cap};
use crate::validation::habit::validate_habit_name;
use async_trait::async_trait;
use std::sync::Arc;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), mockall::automock)]
pub trait HabitService: Send + Sync {
    async fn create(&self, new_habit: CreateHabit) -> Result<Habit, AppError>;
    async fn get_by_id(&self, id: i32, user_id: Uuid) -> Result<Option<Habit>, AppError>;
    async fn delete(&self, id: i32, user_id: Uuid) -> Result<(), AppError>;
    async fn get_all_with_period_checks(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError>;
    async fn add_check(&self, habit_id: i32, value: i32, timestamp: OffsetDateTime) -> Result<Check, AppError>;
    async fn reset_period_checks(&self, habit_id: i32, timestamp: OffsetDateTime) -> Result<(), AppError>;
    async fn get_activity_dates(&self, from: Date, to: Date) -> Result<Vec<Date>, AppError>;
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

    async fn get_by_id(&self, id: i32, user_id: Uuid) -> Result<Option<Habit>, AppError> {
        let habit = self.habit_repo.get_by_id(id, user_id).await?;

        Ok(habit)
    }

    async fn delete(&self, id: i32, user_id: Uuid) -> Result<(), AppError> {
        let deleted = self.habit_repo.delete(id, user_id).await?;

        if deleted {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("habit with id {} not found", id)))
        }
    }

    async fn get_all_with_period_checks(&self, timestamp: OffsetDateTime) -> Result<Vec<HabitWithCheck>, AppError> {
        let habits_with_checks = self.habit_repo.get_all_with_period_checks(timestamp).await?;

        Ok(habits_with_checks)
    }

    async fn add_check(&self, habit_id: i32, value: i32, checked_at: OffsetDateTime) -> Result<Check, AppError> {
        validate_check_value(value)?;
        validate_check_checked_at(&checked_at)?;

        let habit_wc = self
            .habit_repo
            .get_with_period_checks(habit_id, checked_at)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("habit with id {} not found", habit_id)))?;

        validate_period_cap(
            value,
            habit_wc.checks.iter().map(|c| c.value).sum::<i32>(),
            habit_wc.goal_value,
        )?;

        let check = self
            .check_repo
            .upsert(&AddCheck::new(habit_id, value, checked_at))
            .await?;

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

    async fn get_activity_dates(&self, from: Date, to: Date) -> Result<Vec<Date>, AppError> {
        validate_date_range(from, to)?;

        let dates = self.check_repo.get_activity_dates(from, to).await?;

        Ok(dates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::habit::GoalPeriod;
    use crate::repositories::{MockCheckRepository, MockHabitRepository};
    use time::macros::datetime;

    fn make_service(habit_repo: MockHabitRepository, check_repo: MockCheckRepository) -> DefaultHabitService {
        DefaultHabitService::new(Arc::new(habit_repo), Arc::new(check_repo))
    }

    // make a HabitWithCheck with hardcoded created_at 10 feb. 2026
    fn make_habit_with_checks(id: i32, goal_value: i32, checks: Vec<Check>) -> HabitWithCheck {
        HabitWithCheck {
            id,
            name: "Cardio".to_string(),
            goal_value,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Week,
            created_at: datetime!(2026-02-10 00:00 UTC),
            checks,
        }
    }

    #[tokio::test]
    async fn test_delete_ok_returns_ok() {
        let mut habit_repo = MockHabitRepository::new();
        habit_repo
            .expect_delete()
            .return_once(|_| Box::pin(async move { Ok(true) }));

        let service = make_service(habit_repo, MockCheckRepository::new());

        assert!(service.delete(1).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_unknown_habit_returns_not_found_error() {
        let mut habit_repo = MockHabitRepository::new();
        habit_repo
            .expect_delete()
            .return_once(|_| Box::pin(async move { Ok(false) }));

        let service = make_service(habit_repo, MockCheckRepository::new());

        assert!(matches!(service.delete(1).await, Err(AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_add_check_ok_returns_check() {
        let day1 = datetime!(2026-03-02 10:00 UTC); // Monday
        let day2 = datetime!(2026-03-03 10:00 UTC); // Tuesday
        let day3 = datetime!(2026-03-04 10:00 UTC); // Wednesday

        let mut habit_repo = MockHabitRepository::new();
        habit_repo
            .expect_get_with_period_checks()
            .return_once(move |id, _| {
                Box::pin(async move {
                    Ok(Some(make_habit_with_checks(
                        id,
                        180,
                        vec![
                            Check::new(1, id, 60, day1),
                            Check::new(2, id, 90, day2)
                        ]
                    )))
                })
            });

        let mut check_repo = MockCheckRepository::new();
        check_repo.expect_upsert().return_once(|add_check| {
            let habit_id = add_check.habit_id;
            let value = add_check.value;
            let checked_at = add_check.checked_at;
            Box::pin(async move { Ok(Check::new(3, habit_id, value, checked_at)) })
        });

        let service = make_service(habit_repo, check_repo);
        let result = service.add_check(1, 20, day3).await;

        let expected = Check {
            id: 3,
            habit_id: 1,
            value: 20,
            checked_at: day3,
        };
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_add_check_unknown_habit_returns_not_found_error() {
        let mut habit_repo = MockHabitRepository::new();
        habit_repo
            .expect_get_with_period_checks()
            .return_once(move |_, _| Box::pin(async move { Ok(None) }));

        let service = make_service(habit_repo, MockCheckRepository::new());

        let result = service.add_check(10, 888, OffsetDateTime::UNIX_EPOCH).await;

        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_add_check_exceeds_period_cap_returns_validation_error() {
        let day1 = datetime!(2026-03-02 10:00 UTC);
        let checked_at = datetime!(2026-03-03 10:00 UTC);

        let mut habit_repo = MockHabitRepository::new();
        habit_repo
            .expect_get_with_period_checks()
            .return_once(move |id, _| {
                Box::pin(async move {
                    Ok(Some(make_habit_with_checks(
                        id,
                        180,
                        vec![Check::new(1, id, 170, day1)]
                    )))
                })
            });

        let service = make_service(habit_repo, MockCheckRepository::new());
        let result = service.add_check(1, 20, checked_at).await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[tokio::test]
    async fn test_reset_period_checks_unknown_habit_returns_not_found_error() {
        let mut habit_repo = MockHabitRepository::new();
        habit_repo
            .expect_get_with_period_checks()
            .return_once(|_, _| Box::pin(async move { Ok(None) }));

        let service = make_service(habit_repo, MockCheckRepository::new());

        let result = service.reset_period_checks(1, datetime!(2026-03-03 10:00 UTC)).await;

        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_reset_period_checks_ok_returns_ok() {
        let day1 = datetime!(2026-03-02 10:00 UTC); // Monday
        let day2 = datetime!(2026-03-03 10:00 UTC); // Tuesday
        let now = datetime!(2026-03-04 10:00 UTC);

        let mut habit_repo = MockHabitRepository::new();
        habit_repo
            .expect_get_with_period_checks()
            .return_once(move |id, _| {
                Box::pin(async move {
                    Ok(Some(make_habit_with_checks(
                        id,
                        180,
                        vec![
                            Check::new(1, id, 60, day1),
                            Check::new(2, id, 90, day2)
                        ]
                    )))
                })
            });

        let mut check_repo = MockCheckRepository::new();
        check_repo
            .expect_delete_by_habit_for_period()
            .return_once(|_, _, _| Box::pin(async move { Ok(1) }));

        let service = make_service(habit_repo, check_repo);

        assert!(service.reset_period_checks(1, now).await.is_ok());
    }
}
