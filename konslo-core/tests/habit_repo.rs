#[cfg(test)]
mod tests {
    use konslo_core::error::AppError;
    use konslo_core::models::check::{Check, NewCheck};
    use konslo_core::models::habit::{GoalPeriod, Habit, HabitWithCheck, NewHabit};
    use konslo_core::models::user::{NewUser, User};
    use konslo_core::repositories::check::{CheckRepository, PostgresCheckRepository};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};
    use konslo_core::repositories::user::{PostgresUserRepository, UserRepository};
    use sqlx::PgPool;
    use std::error::Error;
    use time::macros::datetime;
    use time::{Duration, OffsetDateTime};
    use uuid::Uuid;

    async fn seed_habit(pool: &PgPool, new_habit: &NewHabit) -> Result<Habit, Box<dyn Error>> {
        let repo = PostgresHabitRepository::new(pool.clone());
        Ok(repo.create(new_habit).await?)
    }

    async fn seed_check(pool: &PgPool, new_check: &NewCheck) -> Result<Check, Box<dyn Error>> {
        let repo = PostgresCheckRepository::new(pool.clone());
        Ok(repo.upsert(new_check).await?)
    }

    async fn seed_user(pool: &PgPool) -> Result<User, Box<dyn Error>> {
        let repo = PostgresUserRepository::new(pool.clone());
        Ok(repo
            .create(&NewUser {
                username: "test_user".to_string(),
                password_hash: "test_password_hash".to_string(),
            })
            .await?)
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
        async fn test_create_ok_returns_habit(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            let s_user = seed_user(&pool).await?;
            let new = NewHabit {
                user_id: s_user.id,
                name: "Meditate".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            };

            let created = repo.create(&new).await?;

            assert_eq!(created.name, new.name);
            assert_eq!(created.goal_value, new.goal_value);
            assert_eq!(created.goal_unit, new.goal_unit);
            assert_eq!(created.goal_period, new.goal_period);
            Ok(())
        }

        #[sqlx::test]
        async fn test_create_duplicate_name_returns_conflict_err(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            let s_user = seed_user(&pool).await?;
            let new = NewHabit {
                user_id: s_user.id,
                name: "Meditate".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            };

            repo.create(&new).await?;
            let result = repo.create(&new).await;

            assert!(matches!(result.unwrap_err(), AppError::Conflict(_)));
            Ok(())
        }
    }

    mod get_by_id {
        use super::*;

        #[sqlx::test]
        async fn test_get_by_id_ok_returns_habit(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            let s_user = seed_user(&pool).await?;
            let s_habit = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Meditate".to_string(),
                    goal_value: 10,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Day,
                },
            )
            .await?;

            let fetched = repo.get_by_id(s_habit.id, s_user.id).await?;

            assert_eq!(fetched, Some(s_habit));
            Ok(())
        }

        #[sqlx::test]
        async fn test_get_by_id_not_found_returns_none(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            let fetched = repo.get_by_id(Uuid::new_v4(), Uuid::new_v4()).await?;

            assert!(fetched.is_none());
            Ok(())
        }
    }

    mod get_with_period_checks {
        use super::*;

        #[sqlx::test]
        async fn test_get_with_period_checks_ok_returns_habit_with_check(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());
            let ts = OffsetDateTime::now_utc();

            let s_user = seed_user(&pool).await?;
            let s_habit = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Meditate".to_string(),
                    goal_value: 10,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Day,
                },
            )
            .await?;
            let s_check = seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit.id,
                    value: 5,
                    checked_at: ts,
                },
            )
            .await?;

            let fetched = repo.get_with_period_checks(s_habit.id, s_user.id, ts).await?;

            assert_eq!(fetched, Some(to_habit_with_checks(&s_habit, vec![s_check])));
            Ok(())
        }

        #[sqlx::test]
        async fn test_get_with_period_checks_no_checks_returns_empty_vec(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());
            let ts = OffsetDateTime::now_utc();

            let s_user = seed_user(&pool).await?;
            let s_habit = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Meditate".to_string(),
                    goal_value: 10,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Day,
                },
            )
            .await?;

            let fetched = repo.get_with_period_checks(s_habit.id, s_user.id, ts).await?;

            assert_eq!(fetched, Some(to_habit_with_checks(&s_habit, vec![])));
            Ok(())
        }

        #[sqlx::test]
        async fn test_get_with_period_checks_not_found_returns_none(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            let fetched = repo
                .get_with_period_checks(Uuid::new_v4(), Uuid::new_v4(), OffsetDateTime::now_utc())
                .await?;

            assert!(fetched.is_none());
            Ok(())
        }
    }

    mod get_all_with_period_checks {
        use super::*;

        #[sqlx::test]
        async fn test_get_all_with_period_checks_ok_returns_habits_with_checks(
            pool: PgPool,
        ) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            let s_user = seed_user(&pool).await?;
            let s_habit_day = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Meditate".to_string(),
                    goal_value: 10,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Day,
                },
            )
            .await?;
            let s_habit_day_empty = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Stretch neck".to_string(),
                    goal_value: 20,
                    goal_unit: "reps".to_string(),
                    goal_period: GoalPeriod::Day,
                },
            )
            .await?;
            let s_habit_week = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Cardio".to_string(),
                    goal_value: 180,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Week,
                },
            )
            .await?;
            let s_habit_month = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Read".to_string(),
                    goal_value: 4,
                    goal_unit: "books".to_string(),
                    goal_period: GoalPeriod::Month,
                },
            )
            .await?;

            let reference = datetime!(2026-03-15 12:00:00 UTC);
            let start_of_week = reference - Duration::days(reference.weekday().number_days_from_monday() as i64);
            let start_of_month = reference.replace_day(1)?;

            // daily: check today (in), check yesterday (out)
            let s_check_day = seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_day.id,
                    value: 5,
                    checked_at: reference,
                },
            )
            .await?;
            seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_day.id,
                    value: 5,
                    checked_at: reference - Duration::days(1),
                },
            )
            .await?;

            // weekly: 2 checks this week (in), 1 check last week (out)
            let s_check_week_1 = seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_week.id,
                    value: 90,
                    checked_at: start_of_week,
                },
            )
            .await?;
            let s_check_week_2 = seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_week.id,
                    value: 20,
                    checked_at: start_of_week + Duration::days(1),
                },
            )
            .await?;
            seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_week.id,
                    value: 20,
                    checked_at: start_of_week - Duration::days(1),
                },
            )
            .await?;

            // monthly: 2 checks this month (in), 1 check last month (out)
            let s_check_month_1 = seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_month.id,
                    value: 1,
                    checked_at: start_of_month,
                },
            )
            .await?;
            let s_check_month_2 = seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_month.id,
                    value: 1,
                    checked_at: start_of_month + Duration::weeks(1),
                },
            )
            .await?;
            seed_check(
                &pool,
                &NewCheck {
                    habit_id: s_habit_month.id,
                    value: 1,
                    checked_at: start_of_month - Duration::weeks(1),
                },
            )
            .await?;

            let fetched = repo.get_all_with_period_checks(s_user.id, reference).await?;

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
            let repo = PostgresHabitRepository::new(pool.clone());

            let s_user = seed_user(&pool).await?;

            let fetched = repo
                .get_all_with_period_checks(s_user.id, OffsetDateTime::now_utc())
                .await?;

            assert!(fetched.is_empty());
            Ok(())
        }
    }

    mod delete {
        use super::*;

        #[sqlx::test]
        async fn test_delete_ok_returns_true(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool.clone());

            let s_user = seed_user(&pool).await?;
            let s_habit = seed_habit(
                &pool,
                &NewHabit {
                    user_id: s_user.id,
                    name: "Meditate".to_string(),
                    goal_value: 10,
                    goal_unit: "min".to_string(),
                    goal_period: GoalPeriod::Day,
                },
            )
            .await?;

            let deleted = repo.delete(s_habit.id, s_user.id).await?;
            let fetched = repo.get_by_id(s_habit.id, s_user.id).await?;

            assert!(deleted);
            assert!(fetched.is_none());
            Ok(())
        }

        #[sqlx::test]
        async fn test_delete_not_found_returns_false(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo = PostgresHabitRepository::new(pool);

            let deleted = repo.delete(Uuid::new_v4(), Uuid::new_v4()).await?;

            assert!(!deleted);
            Ok(())
        }
    }
}
