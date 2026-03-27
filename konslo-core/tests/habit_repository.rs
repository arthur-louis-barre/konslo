#[cfg(test)]
mod tests {
    use konslo_core::errors::AppError;
    use konslo_core::models::check::{Check, CreateCheck};
    use konslo_core::models::habit::{CreateHabit, GoalPeriod, Habit, HabitWithCheck};
    use konslo_core::repositories::check::{CheckRepository, PostgresCheckRepository};
    use konslo_core::repositories::habit::{HabitRepository, PostgresHabitRepository};
    use sqlx::PgPool;
    use std::error::Error;
    use time::macros::datetime;
    use time::{Duration, OffsetDateTime};

    fn make_create_habit() -> CreateHabit {
        CreateHabit {
            name: "Meditate".to_string(),
            goal_value: 10,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Day,
        }
    }

    #[sqlx::test]
    async fn test_create_habit_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit = make_create_habit();

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
        let new_habit = make_create_habit();

        // act
        repo.create(&new_habit).await?;
        let result = repo.create(&new_habit).await;

        // assert
        assert!(result.is_err(), "result should be Err");
        assert!(matches!(result.unwrap_err(), AppError::Conflict(_)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_habit_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit = make_create_habit();
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
    async fn test_delete_habit_err_not_found(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);

        // act
        let deleted = repo.delete(9999).await?;

        // assert
        assert!(!deleted);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_by_id_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let repo = PostgresHabitRepository::new(pool);
        let new_habit = make_create_habit();

        let created = repo.create(&new_habit).await?;

        // act
        let fetched = repo.get_by_id(created.id).await?;

        // assert
        assert!(fetched.is_some(), "habit should exist after creation");
        assert_eq!(fetched.unwrap(), created);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_by_id_not_found(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let repo = PostgresHabitRepository::new(pool);

        let result = repo.get_by_id(999).await?;

        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all_with_checks_for_ok(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // arrange
        let (reference, habits, checks) = setup_habits_with_checks(&pool).await?;
        let repo = PostgresHabitRepository::new(pool);

        // act
        let habits_with_checks = repo.get_all_with_checks_for(reference).await?;

        // assert
        let expected = vec![
            to_habit_with_check(&habits[0], vec![checks[0].clone()]),
            to_habit_with_check(&habits[1], vec![]),
            to_habit_with_check(&habits[2], vec![checks[2].clone(), checks[3].clone()]),
            to_habit_with_check(&habits[3], vec![checks[5].clone(), checks[6].clone()]),
        ];

        assert_eq!(habits_with_checks, expected);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all_with_checks_for_empty(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let repo = PostgresHabitRepository::new(pool);

        let result = repo.get_all_with_checks_for(OffsetDateTime::now_utc()).await?;

        assert!(result.is_empty());

        Ok(())
    }

    // helper functions

    async fn setup_habits_with_checks(
        pool: &PgPool,
    ) -> Result<(OffsetDateTime, Vec<Habit>, Vec<Check>), Box<dyn Error>> {
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
            name: "Cardio".to_string(),
            goal_value: 180,
            goal_unit: "min".to_string(),
            goal_period: GoalPeriod::Week,
        };
        let new_habit_4 = CreateHabit {
            name: "Read".to_string(),
            goal_value: 4,
            goal_unit: "books".to_string(),
            goal_period: GoalPeriod::Month,
        };

        let habit_created_1 = repo.create(&new_habit_1).await?;
        let habit_created_2 = repo.create(&new_habit_2).await?;
        let habit_created_3 = repo.create(&new_habit_3).await?;
        let habit_created_4 = repo.create(&new_habit_4).await?;

        let repo = PostgresCheckRepository::new(pool.clone());

        let reference = datetime!(2026-03-15 12:55:20 UTC);
        let days_since_monday = reference.weekday().number_days_from_monday();
        let start_of_week = reference - Duration::days(days_since_monday as i64);
        let start_of_month = reference.replace_day(1)?;

        let new_check_1 = CreateCheck {
            habit_id: habit_created_1.id,
            value: 5,
            checked_at: reference,
        };
        let new_check_2 = CreateCheck {
            habit_id: habit_created_1.id,
            value: 5,
            checked_at: reference - Duration::days(1),
        };
        let new_check_3 = CreateCheck {
            habit_id: habit_created_3.id,
            value: 90,
            checked_at: start_of_week,
        };
        let new_check_4 = CreateCheck {
            habit_id: habit_created_3.id,
            value: 20,
            checked_at: start_of_week + Duration::days(1),
        };
        let new_check_5 = CreateCheck {
            habit_id: habit_created_3.id,
            value: 20,
            checked_at: start_of_week - Duration::days(1),
        };
        let new_check_6 = CreateCheck {
            habit_id: habit_created_4.id,
            value: 1,
            checked_at: start_of_month,
        };
        let new_check_7 = CreateCheck {
            habit_id: habit_created_4.id,
            value: 1,
            checked_at: start_of_month + Duration::weeks(1),
        };
        let new_check_8 = CreateCheck {
            habit_id: habit_created_4.id,
            value: 1,
            checked_at: start_of_month - Duration::weeks(1),
        };

        let check_created_1 = repo.create(&new_check_1).await?;
        let check_created_2 = repo.create(&new_check_2).await?;
        let check_created_3 = repo.create(&new_check_3).await?;
        let check_created_4 = repo.create(&new_check_4).await?;
        let check_created_5 = repo.create(&new_check_5).await?;
        let check_created_6 = repo.create(&new_check_6).await?;
        let check_created_7 = repo.create(&new_check_7).await?;
        let check_created_8 = repo.create(&new_check_8).await?;

        Ok((
            reference,
            vec![habit_created_1, habit_created_2, habit_created_3, habit_created_4],
            vec![
                check_created_1,
                check_created_2,
                check_created_3,
                check_created_4,
                check_created_5,
                check_created_6,
                check_created_7,
                check_created_8,
            ],
        ))
    }

    fn to_habit_with_check(habit: &Habit, checks: Vec<Check>) -> HabitWithCheck {
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
}