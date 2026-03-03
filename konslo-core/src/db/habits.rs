use async_trait::async_trait;
use sqlx::{query, query_as, Error, PgPool};
use crate::model::Habit;

#[async_trait]
pub trait HabitRepository: Send + Sync {
    async fn create(&self, name: &str) -> Result<Habit, Error>;
    async fn delete(&self, id: i32) -> Result<bool, Error>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, Error>;
    async fn get_all(&self) -> Result<Vec<Habit>, Error>;
}

pub struct PostgresHabitRepository {
    pool: PgPool,
}

impl PostgresHabitRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresHabitRepository {
            pool,
        }
    }
}

#[async_trait]
impl HabitRepository for PostgresHabitRepository {
    async fn create(&self, name: &str) -> Result<Habit, Error> {
        let habit: Habit = query_as("INSERT INTO habits (name) VALUES ($1) RETURNING id, name")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok(habit)
    }

    async fn delete(&self, id: i32) -> Result<bool, Error> {
        let result = query("DELETE FROM habits WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() == 1)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Habit>, Error> {
        let habit: Option<Habit> = query_as("SELECT id, name FROM habits WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(habit)
    }

    async fn get_all(&self) -> Result<Vec<Habit>, Error> {
        let habits: Vec<Habit> = query_as("SELECT id, name FROM habits ORDER BY id")
            .fetch_all(&self.pool)
            .await?;

        Ok(habits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    #[ignore = "integration: needs DB"]
    async fn test_create_habit(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let repo = PostgresHabitRepository::new(pool);

        let habit_name = "Walking 10 minutes";
        let habit = repo.create(habit_name).await?;

        assert!(habit.id() > 0);
        assert_eq!(habit.name(), "Walking 10 minutes");
        Ok(())
    }

    #[sqlx::test]
    #[ignore = "integration: needs DB"]
    async fn test_delete_habit(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let repo = PostgresHabitRepository::new(pool);
        let habit = repo.create("Meditate").await?;

        // Act
        let deleted = repo.delete(habit.id()).await?;

        // Assert
        assert!(deleted);
        let found = repo.get_by_id(habit.id()).await?;
        assert!(found.is_none());

        Ok(())
    }

    #[sqlx::test]
    #[ignore = "integration: needs DB"]
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
    #[ignore = "integration: needs DB"]
    async fn test_get_by_id(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let repo = PostgresHabitRepository::new(pool);
        let created = repo.create("Meditate").await?;

        // Act
        let fetched = repo.get_by_id(created.id()).await?;

        // Assert
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched, created);

        Ok(())
    }

    #[sqlx::test]
    #[ignore = "integration: needs DB"]
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
    #[ignore = "integration: needs DB"]
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

// TODO: Refactorer pour utiliser l'injection d'exécuteur.
// Actuellement, le repo possède le pool (couplage fort).
// Standard pro : Passer &PgPool ou &mut Transaction en argument des méthodes
// pour permettre l'atomicité des opérations (plusieurs repos dans une seule transaction).