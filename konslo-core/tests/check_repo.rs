#[cfg(test)]
mod tests {
    use konslo_core::errors::AppError;
    use konslo_core::models::check::{NewCheck, Check};
    use konslo_core::models::habit::{NewHabit, GoalPeriod, Habit};
    use konslo_core::repositories::{
        CheckRepository, HabitRepository, PostgresCheckRepository, PostgresHabitRepository,
    };
    use sqlx::PgPool;
    use std::error::Error;
    use std::ops::Add;
    use time::{Duration, OffsetDateTime};
    use time::macros::time;

    async fn seed_habit(pool: &PgPool, create_habit: &NewHabit) -> Result<Habit, Box<dyn Error>> {
        let repo = PostgresHabitRepository::new(pool.clone());
        Ok(repo.create(create_habit).await?)
    }

    async fn seed_check(pool: &PgPool, add_check: &NewCheck) -> Result<Check, Box<dyn Error>> {
        let repo = PostgresCheckRepository::new(pool.clone());
        Ok(repo.upsert(add_check).await?)
    }

    mod upsert {
        use super::*;

        #[sqlx::test]
        async fn test_upsert_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool.clone());
            let s_habit = seed_habit(&pool, &NewHabit::new(
                "Meditate",
                10,
                "min",
                GoalPeriod::Day
            )).await?;

            let yesterday_first = OffsetDateTime::now_utc().add(Duration::days(-1)).replace_time(time!(12:00:00));
            let yesterday_second = yesterday_first.add(Duration::hours(4));
            let add_check_first = NewCheck::new(s_habit.id, 5, yesterday_first);
            let add_check_second = NewCheck::new(s_habit.id, 3, yesterday_second);

            let created_first = repo.upsert(&add_check_first).await?;
            let created_second = repo.upsert(&add_check_second).await?;

            assert!(created_first.id > 0);
            assert_eq!(created_first.habit_id, add_check_first.habit_id);
            assert_eq!(created_first.value, add_check_first.value);
            assert_eq!(created_first.checked_at, add_check_first.checked_at.truncate_to_microsecond());
            assert_eq!(created_second.id, created_first.id);
            assert_eq!(created_second.habit_id, created_first.habit_id);
            assert_eq!(created_second.value, created_first.value + add_check_second.value);
            assert_eq!(created_second.checked_at, created_first.checked_at);

            Ok(())
        }

        #[sqlx::test]
        async fn test_upsert_habit_not_found_returns_not_found_err(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool);

            let result = repo.upsert(&NewCheck::new(999, 5, OffsetDateTime::now_utc())).await;

            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));

            Ok(())
        }
    }

    mod delete_by_habit_for_period {
        use super::*;

        #[sqlx::test]
        async fn test_delete_by_habit_for_period_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool.clone());
            let s_habit = seed_habit(&pool, &NewHabit::new(
                "Meditate",
                10,
                "min",
                GoalPeriod::Day
            )).await?;

            let now = OffsetDateTime::now_utc();
            seed_check(&pool, &NewCheck::new(s_habit.id, 1, now)).await?;
            seed_check(&pool, &NewCheck::new(s_habit.id, 1, now - Duration::days(1))).await?;
            seed_check(&pool, &NewCheck::new(s_habit.id, 1, now - Duration::days(2))).await?;

            let from = (now - Duration::days(3)).date();
            let to = (now - Duration::days(1)).date();

            let deleted = repo.delete_by_habit_for_period(s_habit.id, from, to).await?;

            assert_eq!(deleted, 2);

            Ok(())
        }

        #[sqlx::test]
        async fn test_delete_by_habit_for_period_no_checks_returns_zero(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool.clone());
            let s_habit = seed_habit(&pool, &NewHabit::new("Meditate", 10, "min", GoalPeriod::Day)).await?;

            let now = OffsetDateTime::now_utc();
            let from = (now - Duration::days(3)).date();
            let to = now.date();

            let deleted = repo.delete_by_habit_for_period(s_habit.id, from, to).await?;

            assert_eq!(deleted, 0);

            Ok(())
        }
    }
}
