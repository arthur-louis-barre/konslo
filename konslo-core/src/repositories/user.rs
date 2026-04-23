use crate::error::AppError;
use crate::models::user::{NewUser, User};
use async_trait::async_trait;
use sqlx::{PgPool, query_file_as};

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), mockall::automock)]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: &NewUser) -> Result<User, AppError>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, new_user: &NewUser) -> Result<User, AppError> {
        let user = query_file_as!(
            User,
            "queries/insert_user.sql",
            new_user.username,
            new_user.password_hash,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = query_file_as!(User, "queries/select_user_by_username.sql", username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
