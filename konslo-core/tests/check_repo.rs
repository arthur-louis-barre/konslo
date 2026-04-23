#[cfg(test)]
mod tests {
    use konslo_core::error::AppError;
    use konslo_core::models::check::{NewCheck};
    use konslo_core::models::habit::{GoalPeriod, Habit, NewHabit};
    use konslo_core::models::user::{NewUser, User};
    use konslo_core::repositories::check::{CheckRepository, PostgresCheckRepository};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};
    use konslo_core::repositories::user::{PostgresUserRepository, UserRepository};
    use sqlx::PgPool;
    use std::error::Error;
    use time::macros::time;
    use time::{Duration, OffsetDateTime};
    use uuid::Uuid;

    async fn seed_user(pool: &PgPool) -> Result<User, Box<dyn Error>> {
        let repo = PostgresUserRepository::new(pool.clone());
        Ok(repo
            .create(&NewUser {
                username: "test_user".to_string(),
                password_hash: "test_password_hash".to_string(),
            })
            .await?)
    }

    async fn seed_habit(pool: &PgPool, new_habit: &NewHabit) -> Result<Habit, Box<dyn Error>> {
        let repo = PostgresHabitRepository::new(pool.clone());
        Ok(repo.create(new_habit).await?)
    }

    mod upsert {
        use super::*;

        #[sqlx::test]
        async fn test_upsert_ok_returns_check(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool.clone());
            let s_user = seed_user(&pool).await?;
            let s_habit = seed_habit(&pool, &NewHabit {
                user_id: s_user.id,
                name: "Meditate".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            }).await?;

            let yesterday = OffsetDateTime::now_utc() - Duration::days(1);
            let new_first = NewCheck { habit_id: s_habit.id, value: 5, checked_at: yesterday.replace_time(time!(12:00:00)) };
            let new_second = NewCheck { habit_id: s_habit.id, value: 3, checked_at: yesterday.replace_time(time!(16:00:00)) };

            let created_first = repo.upsert(&new_first).await?;
            let created_second = repo.upsert(&new_second).await?;

            assert_eq!(created_first.habit_id, new_first.habit_id);
            assert_eq!(created_first.value, new_first.value);
            assert_eq!(created_second.id, created_first.id);
            assert_eq!(created_second.habit_id, created_first.habit_id);
            assert_eq!(created_second.value, new_first.value + new_second.value);
            assert_eq!(created_second.checked_at, created_first.checked_at);

            Ok(())
        }

        #[sqlx::test]
        async fn test_upsert_habit_not_found_returns_not_found_err(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool);

            let result = repo.upsert(&NewCheck {
                habit_id: Uuid::new_v4(),
                value: 5,
                checked_at: OffsetDateTime::now_utc(),
            }).await;

            assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));

            Ok(())
        }
    }

    mod delete_by_habit_for_period {
        use super::*;

        #[sqlx::test]
        async fn test_delete_by_habit_for_period_ok_returns_count(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool.clone());
            let s_user = seed_user(&pool).await?;
            let s_habit = seed_habit(&pool, &NewHabit {
                user_id: s_user.id,
                name: "Meditate".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            }).await?;

            let now = OffsetDateTime::now_utc();
            repo.upsert(&NewCheck { habit_id: s_habit.id, value: 1, checked_at: now }).await?;
            repo.upsert(&NewCheck { habit_id: s_habit.id, value: 1, checked_at: now - Duration::days(1) }).await?;
            repo.upsert(&NewCheck { habit_id: s_habit.id, value: 1, checked_at: now - Duration::days(2) }).await?;

            let from = (now - Duration::days(3)).date();
            let to = (now - Duration::days(1)).date();

            let deleted = repo.delete_by_habit_for_period(s_habit.id, from, to).await?;

            assert_eq!(deleted, 2);

            Ok(())
        }

        #[sqlx::test]
        async fn test_delete_by_habit_for_period_no_checks_returns_zero(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresCheckRepository::new(pool.clone());
            let s_user = seed_user(&pool).await?;
            let s_habit = seed_habit(&pool, &NewHabit {
                user_id: s_user.id,
                name: "Meditate".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            }).await?;

            let now = OffsetDateTime::now_utc();
            let from = (now - Duration::days(3)).date();
            let to = now.date();

            let deleted = repo.delete_by_habit_for_period(s_habit.id, from, to).await?;

            assert_eq!(deleted, 0);

            Ok(())
        }
    }
}
