use crate::errors::AppError;
use crate::models::user::{CreateUser, User};
use crate::repositories::UserRepository;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
#[cfg_attr(any(test, feature = "mockable"), mockall::automock)]
pub trait UserService: Send + Sync {
    async fn register(&self, username: &str, password: &str) -> Result<User, AppError>;
    async fn login(&self, username: &str, password: &str) -> Result<User, AppError>;
}

#[derive(Clone)]
pub struct DefaultUserService {
    user_repo: Arc<dyn UserRepository>,
}

impl DefaultUserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl UserService for DefaultUserService {
    async fn register(&self, username: &str, password: &str) -> Result<User, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let new_user = CreateUser {
            username: username.to_string(),
            password_hash: password_hash.to_string(),
        };

        let user = self.user_repo.create(&new_user).await?;

        Ok(user)
    }

    async fn login(&self, username: &str, password: &str) -> Result<User, AppError> {
        let user = self
            .user_repo
            .get_by_username(username)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("user with username {} not found", username)))?;

        let password = password.as_bytes();
        let stored_password_hash =
            PasswordHash::new(&user.password_hash).map_err(|e| AppError::Internal(e.to_string()))?;

        Argon2::default()
            .verify_password(password, &stored_password_hash)
            .map(|_| user)
            .map_err(|_| AppError::Validation("invalid password".to_string()))
    }
}
