use crate::error::AppError;
use async_trait::async_trait;
use sqlx::{PgPool, query_file};
use time::OffsetDateTime;
use uuid::Uuid;

#[async_trait]
pub trait DemoUserRepository: Send + Sync {
    async fn create(&self, user_id: Uuid) -> Result<(), AppError>;
    async fn delete_all_older_than(&self, timestamp: OffsetDateTime) -> Result<(), AppError>;
}

pub struct PostgresDemoUserRepository {
    pool: PgPool,
}

impl PostgresDemoUserRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresDemoUserRepository { pool }
    }
}

#[async_trait]
impl DemoUserRepository for PostgresDemoUserRepository {
    async fn create(&self, user_id: Uuid) -> Result<(), AppError> {
        query_file!("queries/insert_demo_user.sql", user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_all_older_than(&self, timestamp: OffsetDateTime) -> Result<(), AppError> {
        query_file!("queries/delete_demo_users_older_than.sql", timestamp)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
