#[cfg(test)]
mod tests {
    use konslo_core::errors::AppError;
    use konslo_core::models::habit::{CreateHabit, GoalPeriod, Habit, HabitWithCheck};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};
    use sqlx::PgPool;
    use std::error::Error;
    use std::ops::Add;
    use time::{Duration, OffsetDateTime};
    use konslo_core::models::check::{Check, CreateCheck};
    use konslo_core::repositories::check::{CheckRepository, PostgresCheckRepository};

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

    #[sqlx::test]
    async fn test_get_today_habits_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let (habits, checks) = setup_habits_with_checks(&pool).await?;
        let repo = PostgresHabitRepository::new(pool);

        // act
        let habits_with_checks = repo.get_all_with_today_checks().await?;

        // assert
        let excepted = vec![
            HabitWithCheck {
                id: habits[0].id,
                name: habits[0].name.clone(),
                goal_value: habits[0].goal_value,
                goal_unit: habits[0].goal_unit.clone(),
                goal_period: habits[0].goal_period,
                created_at: habits[0].created_at,
                checks: vec![checks[0].clone()]
            },
            HabitWithCheck {
                id: habits[1].id,
                name: habits[1].name.clone(),
                goal_value: habits[1].goal_value,
                goal_unit: habits[1].goal_unit.clone(),
                goal_period: habits[1].goal_period,
                created_at: habits[1].created_at,
                checks: vec![]
            },
            HabitWithCheck {
                id: habits[2].id,
                name: habits[2].name.clone(),
                goal_value: habits[2].goal_value,
                goal_unit: habits[2].goal_unit.clone(),
                goal_period: habits[2].goal_period,
                created_at: habits[2].created_at,
                checks: vec![checks[2].clone()]
            }
        ];

        assert_eq!(habits_with_checks, excepted);

        Ok(())
    }

    async fn setup_habits_with_checks(pool: &PgPool) -> Result<(Vec<Habit>, Vec<Check>), Box<dyn Error>> {
        let repo = PostgresHabitRepository::new(pool.clone());
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
            name: "Train cardio zone 2".to_string(),
            goal_value: 180,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Week,
        };

        let habit_created_1 = repo.create(&new_habit_1).await?;
        let habit_created_2 = repo.create(&new_habit_2).await?;
        let habit_created_3 = repo.create(&new_habit_3).await?;

        let repo = PostgresCheckRepository::new(pool.clone());
        let new_check_1 = CreateCheck {
            habit_id: habit_created_1.id,
            value: 5,
            checked_at: OffsetDateTime::now_utc(),
        };
        let new_check_2 = CreateCheck {
            habit_id: habit_created_2.id,
            value: 10,
            checked_at: OffsetDateTime::now_utc().add(Duration::days(-1)),
        };
        let new_check_3 = CreateCheck {
            habit_id: habit_created_3.id,
            value: 90,
            checked_at: OffsetDateTime::now_utc(),
        };
        let new_check_4 = CreateCheck {
            habit_id: habit_created_3.id,
            value: 20,
            checked_at: OffsetDateTime::now_utc().add(Duration::weeks(-1)),
        };

        let check_created_1 = repo.create(&new_check_1).await?;
        let check_created_2 = repo.create(&new_check_2).await?;
        let check_created_3 = repo.create(&new_check_3).await?;
        let check_created_4 = repo.create(&new_check_4).await?;

        Ok((
            vec![habit_created_1, habit_created_2, habit_created_3],
            vec![check_created_1, check_created_2, check_created_3, check_created_4]
        ))
    }

}
