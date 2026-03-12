#[cfg(test)]
mod tests {
    use konslo_core::errors::AppError;
    use konslo_core::models::habit::{CreateHabit, GoalPeriod};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};
    use sqlx::PgPool;
    use std::error::Error;


    #[sqlx::test]
    async fn test_create_habit_returns_created_habit(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };

        // act
        let habit = repo.create(&new_habit).await?;

        // assert
        assert!(habit.id > 0);
        assert_eq!(habit.name, "Meditate");
        assert_eq!(habit.goal_value, 10);
        assert_eq!(habit.goal_unit, "min");
        assert_eq!(habit.goal_period, GoalPeriod::Day);

        Ok(())
    }

    #[sqlx::test]
    async fn test_create_habit_err_conflict(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };

        // act
        repo.create(&new_habit).await?;
        let result = repo.create(&new_habit).await;

        // assert
        assert!(result.is_err(), "result should be Err");
        assert!(matches!(result.unwrap_err(), AppError::Conflict(_)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_habit(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };
        let habit = repo.create(&new_habit).await?;

        // act
        let deleted = repo.delete(habit.id).await?;
        let found = repo.get_by_id(habit.id).await?;

        // assert
        assert!(deleted);
        assert!(found.is_none(), "habit should not exist after deletion");

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_habit_not_found(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);

        // act
        let deleted = repo.delete(9999).await?;

        // assert
        assert!(!deleted);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };
        let created = repo.create(&new_habit).await?;

        // act
        let fetched = repo.get_by_id(created.id).await?;

        // assert
        assert!(fetched.is_some(), "habit should exist after creation");
        assert_eq!(fetched.unwrap(), created);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit_1 = CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        };
        let new_habit_2 = CreateHabit {
            name: "Stretch shoulders".to_string(),
            goal_value: 20,
            goal_unit: "reps".to_string(),
            goal_period: GoalPeriod::Day,
        };
        let new_habit_3 = CreateHabit {
            name: "Cardio Zone 2".to_string(),
            goal_value: 180,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Week,
        };
        let created_1 = repo.create(&new_habit_1).await?;
        let created_2 = repo.create(&new_habit_2).await?;
        let created_3 = repo.create(&new_habit_3).await?;

        // act
        let fetched = repo.get_all().await?;

        // assert
        assert_eq!(fetched.len(), 3);
        assert_eq!(fetched[0], created_1);
        assert_eq!(fetched[1], created_2);
        assert_eq!(fetched[2], created_3);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all_empty(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);

        // act
        let fetched = repo.get_all().await?;

        // assert
        assert!(fetched.is_empty());

        Ok(())
    }
}