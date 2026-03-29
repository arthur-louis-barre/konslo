#[cfg(test)]
mod tests {
    use konslo_core::errors::AppError;
    use konslo_core::models::check::{Check, CreateCheck, UpdateCheck};
    use konslo_core::models::habit::{CreateHabit, GoalPeriod, Habit};
    use konslo_core::repositories::check::{CheckRepository, PostgresCheckRepository};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};
    use sqlx::PgPool;
    use std::error::Error;
    use std::ops::Add;
    use time::{Duration, OffsetDateTime};

    mod create {
        use super::*;

        #[sqlx::test]
        async fn test_create_check_returns_created_check(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let repo_check = PostgresCheckRepository::new(pool.clone());

            let habit = seed_habit(&pool).await;
            let checked_at = OffsetDateTime::now_utc().truncate_to_microsecond();
            let check = CreateCheck::new(habit.id, 10, checked_at);

            let created = repo_check.create(&check).await?;

            // assert
            assert!(created.id > 0);
            assert_eq!(created.habit_id, check.habit_id);
            assert_eq!(created.value, check.value);
            assert_eq!(created.checked_at, check.checked_at);

            Ok(())
        }

        #[sqlx::test]
        async fn test_create_check_invalid_habit_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
            // arrange
            let repo = PostgresCheckRepository::new(pool);
            let new_check = CreateCheck {
                habit_id: 10,
                value: 10,
                checked_at: OffsetDateTime::now_utc(),
            };

            // act
            let result = repo.create(&new_check).await;

            // assert
            let err = result.unwrap_err();
            assert!(matches!(err, AppError::NotFound(_)));

            Ok(())
        }

        #[sqlx::test]
        async fn test_create_check_invalid_value(pool: PgPool) -> Result<(), Box<dyn Error>> {
            // arrange
            let habit = seed_habit(&pool).await;

            let repo_check = PostgresCheckRepository::new(pool);
            let new_check = CreateCheck {
                habit_id: habit.id,
                value: -5,
                checked_at: OffsetDateTime::now_utc(),
            };

            // act
            let result = repo_check.create(&new_check).await;

            // assert
            let err = result.unwrap_err();
            // println!("{:?}", err);
            assert!(matches!(err, AppError::Validation(_)));

            Ok(())
        }

        #[sqlx::test]
        async fn test_create_check_invalid_checked_at(pool: PgPool) -> Result<(), Box<dyn Error>> {
            // arrange
            let habit = seed_habit(&pool).await;

            let repo_check = PostgresCheckRepository::new(pool);
            let new_check = CreateCheck {
                habit_id: habit.id,
                value: 10,
                checked_at: OffsetDateTime::now_utc().add(Duration::seconds(60)),
            };

            // act
            let result = repo_check.create(&new_check).await;

            // assert
            let err = result.unwrap_err();
            // println!("{:?}", err);
            assert!(matches!(err, AppError::Validation(_)));

            Ok(())
        }

        #[sqlx::test]
        async fn test_update_check_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            // arrange
            let check = seed_check(&pool).await;
            let repo = PostgresCheckRepository::new(pool);
            let update = UpdateCheck { id: check.id, value: 5 };

            // act
            let updated = repo.update(&update).await?;

            // assert
            let expected = Check {
                id: check.id,
                habit_id: check.habit_id,
                value: 5,
                checked_at: check.checked_at,
            };

            assert!(updated.is_some());
            assert_eq!(updated.unwrap(), expected);

            Ok(())
        }

        #[sqlx::test]
        async fn test_update_check_not_found(pool: PgPool) -> Result<(), Box<dyn Error>> {
            // arrange
            let repo = PostgresCheckRepository::new(pool);
            let update = UpdateCheck { id: 5, value: 5 };

            // act
            let updated = repo.update(&update).await?;

            // assert
            assert!(updated.is_none());

            Ok(())
        }

        #[sqlx::test]
        async fn test_update_invalid_value(pool: PgPool) -> Result<(), Box<dyn Error>> {
            // arrange
            let check = seed_check(&pool).await;
            let repo = PostgresCheckRepository::new(pool);
            let update = UpdateCheck {
                id: check.id,
                value: -5,
            };

            // act
            let updated = repo.update(&update).await;

            // assert
            assert!(matches!(updated.unwrap_err(), AppError::Validation(_)));

            Ok(())
        }
    }

    mod get {
        use super::*;

        #[sqlx::test]
        async fn get_by_habit_and_date_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let check_repo = PostgresCheckRepository::new(pool.clone());
            let checked_at = OffsetDateTime::now_utc();
            let habit = seed_habit(&pool).await;
            let check = check_repo.create(&CreateCheck::new(habit.id, 1, checked_at)).await?;

            let result = check_repo.get_by_habit_and_date(habit.id, checked_at.date()).await;

            assert!(result.is_ok());
            assert_eq!(result?, Some(check));
            Ok(())
        }

        #[sqlx::test]
        async fn get_by_habit_and_date_not_found(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let check_repo = PostgresCheckRepository::new(pool.clone());
            let checked_at = OffsetDateTime::now_utc();

            let result = check_repo.get_by_habit_and_date(999, checked_at.date()).await;

            assert!(result.is_ok());
            assert!(result?.is_none());
            Ok(())
        }

        #[sqlx::test]
        async fn get_by_habit_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let check_repo = PostgresCheckRepository::new(pool.clone());
            let habit = seed_habit(&pool).await;

            let checked_at_1 = OffsetDateTime::now_utc()
                .add(Duration::days(-2))
                .truncate_to_microsecond();
            let checked_at_2 = OffsetDateTime::now_utc()
                .add(Duration::days(-1))
                .truncate_to_microsecond();
            let checked_at_3 = OffsetDateTime::now_utc()
                .truncate_to_microsecond();

            let check_1 = check_repo.create(&CreateCheck::new(habit.id, 1, checked_at_1)).await?;
            let check_2 = check_repo.create(&CreateCheck::new(habit.id, 2, checked_at_2)).await?;
            let check_3 = check_repo.create(&CreateCheck::new(habit.id, 5, checked_at_3)).await?;

            // act
            let result = check_repo.get_by_habit(habit.id).await;

            // assert
            assert!(result.is_ok());
            assert!(result?.len());
            assert_eq!(result?, vec![check_1, check_2, check_3]);

            Ok(())
        }
    }

    mod delete {
        use super::*;

        #[sqlx::test]
        async fn delete_check_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let check_repo = PostgresCheckRepository::new(pool.clone());

            let check = seed_check(&pool).await;

            let deleted = check_repo.delete(check.id).await;
            let found = check_repo.get_by_id(check.id).await;

            assert_eq!(deleted?, 1);
            assert!(found?.is_none());

            Ok(())
        }

        #[sqlx::test]
        async fn delete_check_not_found_returns_zero(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let check_repo = PostgresCheckRepository::new(pool.clone());

            let deleted = check_repo.delete(999).await;

            assert_eq!(deleted?, 0);

            Ok(())
        }

        #[sqlx::test]
        async fn delete_by_habit_in_period_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let check_repo = PostgresCheckRepository::new(pool.clone());

            let habit = seed_habit(&pool).await;

            let checked_at_1 = OffsetDateTime::now_utc()
                .add(Duration::days(-2))
                .truncate_to_microsecond();
            let checked_at_2 = OffsetDateTime::now_utc()
                .add(Duration::days(-1))
                .truncate_to_microsecond();
            let checked_at_3 = OffsetDateTime::now_utc()
                .truncate_to_microsecond();

            let _check_1 = check_repo.create(&CreateCheck::new(habit.id, 1, checked_at_1)).await?;
            let _check_2 = check_repo.create(&CreateCheck::new(habit.id, 1, checked_at_2)).await?;
            let check_3 = check_repo.create(&CreateCheck::new(habit.id, 1, checked_at_3)).await?;

            let from = OffsetDateTime::now_utc().add(Duration::days(-3)).date();
            let to = OffsetDateTime::now_utc().add(Duration::days(-1)).date();

            let deleted = check_repo.delete_by_habit_for_period(habit.id, from, to).await;
            let checks_after = check_repo.get_by_habit(habit.id).await?;

            assert_eq!(deleted?, 2);
            assert_eq!(checks_after, vec![check_3]);

            Ok(())
        }

        #[sqlx::test]
        async fn delete_by_habit_in_period_empty_returns_zero(pool: PgPool) -> Result<(), Box<dyn Error>> {
            let check_repo = PostgresCheckRepository::new(pool.clone());

            let habit = seed_habit(&pool).await;

            let from = OffsetDateTime::now_utc().add(Duration::days(-3)).date();
            let to = OffsetDateTime::now_utc().add(Duration::days(-1)).date();

            let deleted = check_repo.delete_by_habit_for_period(habit.id, from, to).await;

            assert_eq!(deleted?, 0);

            Ok(())
        }
    }

    fn make_create_habit() -> CreateHabit {
        CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 5,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        }
    }

    fn make_create_check(habit_id: i32) -> CreateCheck {
        CreateCheck {
            habit_id,
            value: 1,
            checked_at: OffsetDateTime::now_utc(),
        }
    }

    async fn seed_habit(pool: &PgPool) -> Habit {
        PostgresHabitRepository::new(pool.clone())
            .create(&make_create_habit())
            .await
            .unwrap()
    }

    async fn seed_check(pool: &PgPool) -> Check {
        let habit = seed_habit(pool).await;
        PostgresCheckRepository::new(pool.clone())
            .create(&make_create_check(habit.id))
            .await
            .unwrap()
    }
}
