#[cfg(test)]
mod tests {
    use konslo_core::errors::AppError;
    use konslo_core::models::check::{NewCheck, Check};
    use konslo_core::models::habit::{NewHabit, GoalPeriod, Habit, HabitWithCheck};
    use konslo_core::repositories::check::{CheckRepository, PostgresCheckRepository};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};
    use sqlx::PgPool;
    use std::error::Error;
    use time::macros::datetime;
    use time::{Duration, OffsetDateTime};

    async fn seed_habit(pool: &PgPool, create_habit: &NewHabit) -> Result<Habit, Box<dyn Error>> {
        let repo = PostgresHabitRepository::new(pool.clone());
        Ok(repo.create(create_habit).await?)
    }

    async fn seed_check(pool: &PgPool, add_check: &NewCheck) -> Result<Check, Box<dyn Error>> {
        let repo = PostgresCheckRepository::new(pool.clone());
        Ok(repo.upsert(add_check).await?)
    }

    fn to_habit_with_checks(habit: &Habit, checks: Vec<Check>) -> HabitWithCheck {
        HabitWithCheck {
            id: habit.id,
            name: habit.name.clone(),
            goal_value: habit.goal_value,
            goal_unit: habit.goal_unit.clone(),
            goal_period: habit.goal_period,
            created_at: habit.created_at,
            checks,
        }
    }

    mod create {
        use super::*;

        #[sqlx::test]
        async fn test_create_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool);
            let create_habit = NewHabit::new("Meditate", 10, "min", GoalPeriod::Day);

            let created = repo.create(&create_habit).await?;

            assert!(created.id > 0);
            assert_eq!(created.name, "Meditate");
            assert_eq!(created.goal_value, 10);
            assert_eq!(created.goal_unit, "min");
            assert_eq!(created.goal_period, GoalPeriod::Day);

            Ok(())
        }

        #[sqlx::test]
        async fn test_create_duplicate_name_returns_conflict_err(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool);
            let create_habit = NewHabit::new("Meditate", 10, "min", GoalPeriod::Day);

            repo.create(&create_habit).await?;
            let result = repo.create(&create_habit).await;

            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), AppError::Conflict(_)));

            Ok(())
        }
    }

    mod get_by_id {
        use super::*;

        #[sqlx::test]
        async fn test_get_by_id_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());
            let seed = seed_habit(&pool, &NewHabit::new("Meditate", 10, "min", GoalPeriod::Day)).await?;

            let fetched = repo.get_by_id(seed.id).await?;

            assert!(fetched.is_some());
            assert_eq!(fetched.unwrap(), seed);

            Ok(())
        }

        #[sqlx::test]
        async fn test_get_by_id_not_found_returns_none(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool);

            let fetched = repo.get_by_id(999).await?;

            assert!(fetched.is_none());

            Ok(())
        }
    }

    mod get_with_period_checks {
        use super::*;

        #[sqlx::test]
        async fn test_get_with_period_checks_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());
            let s_habit = seed_habit(&pool, &NewHabit::new("Meditate", 10, "min", GoalPeriod::Day)).await?;

            let reference = OffsetDateTime::now_utc();
            let s_check = seed_check(&pool, &NewCheck::new(s_habit.id, 5, reference)).await?;

            let fetched = repo.get_with_period_checks(s_habit.id, reference).await?;

            assert_eq!(fetched, Some(to_habit_with_checks(&s_habit, vec![s_check])));

            Ok(())
        }

        #[sqlx::test]
        async fn test_get_with_period_checks_no_checks_returns_empty_vec(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());
            let seed = seed_habit(&pool, &NewHabit::new("Meditate", 10, "min", GoalPeriod::Day)).await?;

            let fetched = repo.get_with_period_checks(seed.id, OffsetDateTime::now_utc()).await?;

            assert_eq!(fetched, Some(to_habit_with_checks(&seed, vec![])));

            Ok(())
        }

        #[sqlx::test]
        async fn test_get_with_period_checks_not_found_returns_none(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool);

            let fetched = repo.get_with_period_checks(999, OffsetDateTime::now_utc()).await?;

            assert!(fetched.is_none());

            Ok(())
        }
    }

    mod get_all_with_period_checks {
        use super::*;

        #[sqlx::test]
        async fn test_get_all_with_period_checks_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            // 4 habits: daily, daily (no checks), weekly, monthly
            let s_habit_day = seed_habit(&pool, &NewHabit::new("Meditate", 10, "min", GoalPeriod::Day)).await?;
            let s_habit_day_empty =
                seed_habit(&pool, &NewHabit::new("Stretch neck", 20, "reps", GoalPeriod::Day)).await?;
            let s_habit_week = seed_habit(&pool, &NewHabit::new("Cardio", 180, "min", GoalPeriod::Week)).await?;
            let s_habit_month = seed_habit(&pool, &NewHabit::new("Read", 4, "books", GoalPeriod::Month)).await?;

            let reference = datetime!(2026-03-15 12:00:00 UTC);
            let start_of_week = reference - Duration::days(reference.weekday().number_days_from_monday() as i64);
            let start_of_month = reference.replace_day(1)?;

            // daily: check today (in), check yesterday (out)
            let s_check_day = seed_check(&pool, &NewCheck::new(s_habit_day.id, 5, reference)).await?;
            seed_check(&pool, &NewCheck::new(s_habit_day.id, 5, reference - Duration::days(1))).await?;

            // weekly: 2 checks this week (in), 1 check last week (out)
            let s_check_week_1 = seed_check(&pool, &NewCheck::new(s_habit_week.id, 90, start_of_week)).await?;
            let s_check_week_2 = seed_check(
                &pool,
                &NewCheck::new(s_habit_week.id, 20, start_of_week + Duration::days(1)),
            )
            .await?;
            seed_check(
                &pool,
                &NewCheck::new(s_habit_week.id, 20, start_of_week - Duration::days(1)),
            )
            .await?;

            // monthly: 2 checks this month (in), 1 check last month (out)
            let s_check_month_1 = seed_check(&pool, &NewCheck::new(s_habit_month.id, 1, start_of_month)).await?;
            let s_check_month_2 = seed_check(
                &pool,
                &NewCheck::new(s_habit_month.id, 1, start_of_month + Duration::weeks(1)),
            )
            .await?;
            seed_check(
                &pool,
                &NewCheck::new(s_habit_month.id, 1, start_of_month - Duration::weeks(1)),
            )
            .await?;

            let fetched = repo.get_all_with_period_checks(reference).await?;

            let expected = vec![
                to_habit_with_checks(&s_habit_day, vec![s_check_day]),
                to_habit_with_checks(&s_habit_day_empty, vec![]),
                to_habit_with_checks(&s_habit_week, vec![s_check_week_1, s_check_week_2]),
                to_habit_with_checks(&s_habit_month, vec![s_check_month_1, s_check_month_2]),
            ];

            assert_eq!(fetched, expected);

            Ok(())
        }

        #[sqlx::test]
        async fn test_get_all_with_period_checks_no_habits_returns_empty_vec(
            pool: PgPool,
        ) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool);

            let fetched = repo.get_all_with_period_checks(OffsetDateTime::now_utc()).await?;

            assert!(fetched.is_empty());

            Ok(())
        }
    }

    mod delete {
        use super::*;

        #[sqlx::test]
        async fn test_delete_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());
            let seed = seed_habit(&pool, &NewHabit::new("Meditate", 10, "min", GoalPeriod::Day)).await?;

            let deleted = repo.delete(seed.id).await?;
            let fetched = repo.get_by_id(seed.id).await?;

            assert!(deleted);
            assert!(fetched.is_none());

            Ok(())
        }

        #[sqlx::test]
        async fn test_delete_not_found_returns_false(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool);

            let deleted = repo.delete(999).await?;

            assert!(!deleted);

            Ok(())
        }
    }
}
