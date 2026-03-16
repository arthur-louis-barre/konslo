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

    #[sqlx::test]
    async fn test_create_check_returns_created_check(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let habit = create_test_habit(&pool).await;

        let checked_at = OffsetDateTime::now_utc().truncate_to_microsecond();
        let repo_check = PostgresCheckRepository::new(pool);
        let new_check = CreateCheck {
            habit_id: habit.id,
            value: 10,
            checked_at,
        };

        // act
        let check = repo_check.create(&new_check).await?;

        // assert
        assert!(check.id > 0);
        assert_eq!(check.habit_id, habit.id);
        assert_eq!(check.value, 10);
        assert_eq!(check.checked_at, checked_at);

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
        // println!("{:?}", err);
        assert!(matches!(err, AppError::Database(_)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_create_check_invalid_value(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let habit = create_test_habit(&pool).await;

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
        assert!(matches!(err, AppError::Database(_)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_create_check_invalid_checked_at(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let habit = create_test_habit(&pool).await;

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
        assert!(matches!(err, AppError::Database(_)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_check_returns_true(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let check = create_test_check(&pool).await;
        let repo = PostgresCheckRepository::new(pool);
        let update = UpdateCheck {
            id: check.id,
            value: 5,
        };

        // act
        let updated = repo.update(&update).await?;

        // assert
        assert!(updated);

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
        assert!(!updated);

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_invalid_value(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let check = create_test_check(&pool).await;
        let repo = PostgresCheckRepository::new(pool);
        let update = UpdateCheck {
            id: check.id,
            value: -5,
        };

        // act
        let updated = repo.update(&update).await;

        // assert
        assert!(matches!(updated.unwrap_err(), AppError::Database(_)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_check_returns_true(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let check = create_test_check(&pool).await;
        let repo = PostgresCheckRepository::new(pool);
        let id = check.id;

        // act
        let deleted = repo.delete(id).await?;
        let found = repo.get_by_id(id).await?;

        // assert
        assert!(deleted);
        assert!(found.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_check_not_found(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresCheckRepository::new(pool);
        let id = 5;

        // act
        let deleted = repo.delete(id).await?;

        // assert
        assert!(!deleted);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_by_habit_id_returns_checks(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let habit = create_test_habit(&pool).await;
        let repo = PostgresCheckRepository::new(pool);

        let checked_at_1 = OffsetDateTime::now_utc()
            .add(Duration::days(-2))
            .truncate_to_microsecond();
        let checked_at_2 = OffsetDateTime::now_utc()
            .add(Duration::days(-1))
            .truncate_to_microsecond();
        let checked_at_3 = OffsetDateTime::now_utc().truncate_to_microsecond();

        let new_check_1 = CreateCheck {
            habit_id: habit.id,
            value: 5,
            checked_at: checked_at_1,
        };
        let new_check_2 = CreateCheck {
            habit_id: habit.id,
            value: 2,
            checked_at: checked_at_2,
        };
        let new_check_3 = CreateCheck {
            habit_id: habit.id,
            value: 3,
            checked_at: checked_at_3,
        };

        repo.create(&new_check_1).await?;
        repo.create(&new_check_2).await?;
        repo.create(&new_check_3).await?;

        // act
        let checks = repo.get_by_habit_id(habit.id).await?;

        // assert
        assert_eq!(checks[0].value, new_check_1.value);
        assert_eq!(checks[0].checked_at, checked_at_1);
        assert_eq!(checks[0].habit_id, habit.id);
        assert_eq!(checks[1].value, new_check_2.value);
        assert_eq!(checks[1].checked_at, checked_at_2);
        assert_eq!(checks[1].habit_id, habit.id);
        assert_eq!(checks[2].value, new_check_3.value);
        assert_eq!(checks[2].checked_at, checked_at_3);
        assert_eq!(checks[2].habit_id, habit.id);

        Ok(())
    }

    async fn create_test_habit(pool: &PgPool) -> Habit {
        let repo = PostgresHabitRepository::new(pool.clone());
        repo.create(&CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 5,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        })
        .await
        .unwrap()
    }

    async fn create_test_check(pool: &PgPool) -> Check {
        let habit = create_test_habit(&pool).await;
        let repo = PostgresCheckRepository::new(pool.clone());
        repo.create(&CreateCheck {
            habit_id: habit.id,
            value: 1,
            checked_at: OffsetDateTime::now_utc(),
        })
        .await
        .unwrap()
    }
}
