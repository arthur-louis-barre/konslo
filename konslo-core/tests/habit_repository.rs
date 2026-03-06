

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use konslo_core::db::habits::{HabitRepository, PostgresHabitRepository};

    #[sqlx::test]
    async fn test_create_habit(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let repo = PostgresHabitRepository::new(pool);

        let habit_name = "Walking 10 minutes";
        let habit = repo.create(habit_name).await?;

        assert!(habit.id > 0);
        assert_eq!(habit.name, "Walking 10 minutes");
        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_habit(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let repo = PostgresHabitRepository::new(pool);
        let habit = repo.create("Meditate").await?;

        // Act
        let deleted = repo.delete(habit.id).await?;

        // Assert
        assert!(deleted);
        let found = repo.get_by_id(habit.id).await?;
        assert!(found.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_habit_not_found(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let repo = PostgresHabitRepository::new(pool);

        // Act
        let deleted = repo.delete(9999).await?;

        // Assert
        assert!(!deleted);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_by_id(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let repo = PostgresHabitRepository::new(pool);
        let created = repo.create("Meditate").await?;

        // Act
        let fetched = repo.get_by_id(created.id).await?;

        // Assert
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched, created);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let repo = PostgresHabitRepository::new(pool);
        let created_1 = repo.create("Mediate").await?;
        let created_2 = repo.create("Walk for 10 minutes").await?;
        let created_3 = repo.create("Write for 10 minutes").await?;

        // Act
        let fetched = repo.get_all().await?;

        // Assert
        assert_eq!(fetched.len(), 3);
        assert_eq!(fetched[0], created_1);
        assert_eq!(fetched[1], created_2);
        assert_eq!(fetched[2], created_3);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all_empty(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let repo = PostgresHabitRepository::new(pool);

        // Act
        let fetched = repo.get_all().await?;

        // Assert
        assert_eq!(fetched.len(), 0);

        Ok(())
    }
}
