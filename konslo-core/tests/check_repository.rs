#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::ops::Add;
    use sqlx::PgPool;
    use time::{Duration, OffsetDateTime};
    use konslo_core::errors::AppError;
    use konslo_core::models::check::CreateCheck;
    use konslo_core::models::habit::{CreateHabit, GoalPeriod};
    use konslo_core::repositories::check::{CheckRepository, PostgresCheckRepository};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};

    #[sqlx::test]
    async fn test_create_check_returns_created_check(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo_habit = PostgresHabitRepository::new(pool.clone());
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 5,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };
        let habit = repo_habit.create(&new_habit).await?;

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
        let repo_habit = PostgresHabitRepository::new(pool.clone());
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 5,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };
        let habit = repo_habit.create(&new_habit).await?;

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
        let repo_habit = PostgresHabitRepository::new(pool.clone());
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 5,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };
        let habit = repo_habit.create(&new_habit).await?;

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






}
