use crate::error::AppError;
use crate::models::user::{NewUser, User};
use crate::repositories::user::UserRepository;
use crate::validations::user::{validate_password, validate_username};
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
        validate_username(username)?;
        validate_password(password)?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let new_user = NewUser {
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
            .ok_or(AppError::Unauthorized)?;

        let password = password.as_bytes();
        let stored_password_hash =
            PasswordHash::new(&user.password_hash).map_err(|e| AppError::Internal(e.to_string()))?;

        Argon2::default()
            .verify_password(password, &stored_password_hash)
            .map(|_| user)
            .map_err(|_| AppError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user::MockUserRepository;
    use time::macros::datetime;
    use uuid::Uuid;

    fn make_service(user_repo: MockUserRepository) -> DefaultUserService {
        DefaultUserService::new(Arc::new(user_repo))
    }

    fn make_user(password: &str) -> User {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        User {
            id: Uuid::new_v4(),
            username: "alice".to_string(),
            password_hash: hash,
            created_at: datetime!(2026-02-10 00:00 UTC),
        }
    }

    mod register {
        use super::*;

        #[tokio::test]
        async fn test_register_ok_returns_user() {
            let mut user_repo = MockUserRepository::new();
            user_repo.expect_create().return_once(|new| {
                let username = new.username.clone();
                let password_hash = new.password_hash.clone();
                Box::pin(async move {
                    Ok(User {
                        id: Uuid::new_v4(),
                        username,
                        password_hash,
                        created_at: datetime!(2026-02-10 00:00 UTC),
                    })
                })
            });

            let service = make_service(user_repo);
            let result = service.register("alice", "password123").await;

            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_register_duplicate_username_returns_conflict_error() {
            let mut user_repo = MockUserRepository::new();
            user_repo.expect_create().return_once(|_| {
                Box::pin(async move { Err(AppError::Conflict("username already taken".to_string())) })
            });

            let service = make_service(user_repo);
            let result = service.register("alice", "password123").await;

            assert!(matches!(result, Err(AppError::Conflict(_))));
        }

        #[tokio::test]
        async fn test_register_invalid_username_returns_validation_error() {
            let service = make_service(MockUserRepository::new());
            let result = service.register("ab", "password123").await;

            assert!(matches!(result, Err(AppError::Validation(_))));
        }

        #[tokio::test]
        async fn test_register_invalid_password_returns_validation_error() {
            let service = make_service(MockUserRepository::new());
            let result = service.register("alice", "short").await;

            assert!(matches!(result, Err(AppError::Validation(_))));
        }
    }

    mod login {
        use super::*;

        #[tokio::test]
        async fn test_login_ok_returns_user() {
            let s_user = make_user("password123");
            let username = s_user.username.clone();

            let mut user_repo = MockUserRepository::new();
            user_repo
                .expect_get_by_username()
                .return_once(move |_| Box::pin(async move { Ok(Some(s_user)) }));

            let service = make_service(user_repo);
            let result = service.login(&username, "password123").await;

            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_login_unknown_username_returns_unauthorized_error() {
            let mut user_repo = MockUserRepository::new();
            user_repo
                .expect_get_by_username()
                .return_once(|_| Box::pin(async move { Ok(None) }));

            let service = make_service(user_repo);
            let result = service.login("unknown", "password123").await;

            assert!(matches!(result, Err(AppError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_login_wrong_password_returns_unauthorized_error() {
            let s_user = make_user("password123");
            let username = s_user.username.clone();

            let mut user_repo = MockUserRepository::new();
            user_repo
                .expect_get_by_username()
                .return_once(move |_| Box::pin(async move { Ok(Some(s_user)) }));

            let service = make_service(user_repo);
            let result = service.login(&username, "wrongpassword").await;

            assert!(matches!(result, Err(AppError::Unauthorized)));
        }
    }
}
