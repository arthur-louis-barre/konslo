use crate::error::AppError;
use crate::models::check::NewCheck;
use crate::models::habit::{GoalPeriod, NewHabit};
use crate::models::user::{NewUser, User};
use crate::repositories::check::CheckRepository;
use crate::repositories::demo_users::DemoUserRepository;
use crate::repositories::habit::HabitRepository;
use crate::repositories::user::UserRepository;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use async_trait::async_trait;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[async_trait]
pub trait DemoUserService: Send + Sync {
    async fn create(&self) -> Result<User, AppError>;
    async fn delete_all_older_than(&self, timestamp: OffsetDateTime) -> Result<(), AppError>;
}

pub struct DefaultDemoUserService {
    demo_user_repo: Arc<dyn DemoUserRepository>,
    user_repo: Arc<dyn UserRepository>,
    habit_repo: Arc<dyn HabitRepository>,
    check_repo: Arc<dyn CheckRepository>,
}

impl DefaultDemoUserService {
    pub fn new(
        demo_user_repo: Arc<dyn DemoUserRepository>,
        user_repo: Arc<dyn UserRepository>,
        habit_repo: Arc<dyn HabitRepository>,
        check_repo: Arc<dyn CheckRepository>,
    ) -> Self {
        Self {
            demo_user_repo,
            user_repo,
            habit_repo,
            check_repo,
        }
    }

    async fn seed(&self, user_id: &Uuid) -> Result<(), AppError> {
        let now = OffsetDateTime::now_utc();

        let meditate = self
            .habit_repo
            .create(&NewHabit {
                user_id: *user_id,
                name: "Meditate".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            })
            .await?;

        let sunlight_walk = self
            .habit_repo
            .create(&NewHabit {
                user_id: *user_id,
                name: "Sunlight walk".to_string(),
                goal_value: 10,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            })
            .await?;

        let deep_work = self
            .habit_repo
            .create(&NewHabit {
                user_id: *user_id,
                name: "Deep work".to_string(),
                goal_value: 45,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Day,
            })
            .await?;

        let cardio = self
            .habit_repo
            .create(&NewHabit {
                user_id: *user_id,
                name: "Cardio zone 2".to_string(),
                goal_value: 180,
                goal_unit: "min".to_string(),
                goal_period: GoalPeriod::Week,
            })
            .await?;

        let strength = self
            .habit_repo
            .create(&NewHabit {
                user_id: *user_id,
                name: "Strength training".to_string(),
                goal_value: 2,
                goal_unit: "sessions".to_string(),
                goal_period: GoalPeriod::Week,
            })
            .await?;

        let _read = self
            .habit_repo
            .create(&NewHabit {
                user_id: *user_id,
                name: "Read".to_string(),
                goal_value: 1,
                goal_unit: "book".to_string(),
                goal_period: GoalPeriod::Month,
            })
            .await?;

        // history: days 1–6
        for days_ago in 1..=6i64 {
            let date = now - Duration::days(days_ago);

            // meditate: every day
            self.check_repo
                .upsert(&NewCheck {
                    habit_id: meditate.id,
                    value: 10,
                    checked_at: date,
                })
                .await?;

            // sunlight walk: every day
            self.check_repo
                .upsert(&NewCheck {
                    habit_id: sunlight_walk.id,
                    value: 10,
                    checked_at: date,
                })
                .await?;

            // deep work: 4 out of 6 days (skip days 3 and 5)
            if days_ago != 3 && days_ago != 5 {
                self.check_repo
                    .upsert(&NewCheck {
                        habit_id: deep_work.id,
                        value: 45,
                        checked_at: date,
                    })
                    .await?;
            }

            // Cardio zone 2: one session 4 days ago
            if days_ago == 4 {
                self.check_repo
                    .upsert(&NewCheck {
                        habit_id: cardio.id,
                        value: 90,
                        checked_at: date,
                    })
                    .await?;
            }

            // Strength training: one session 5 days ago
            if days_ago == 5 {
                self.check_repo
                    .upsert(&NewCheck {
                        habit_id: strength.id,
                        value: 1,
                        checked_at: date,
                    })
                    .await?;
            }
        }

        // today
        self.check_repo
            .upsert(&NewCheck {
                habit_id: meditate.id,
                value: 5,
                checked_at: now,
            })
            .await?;

        self.check_repo
            .upsert(&NewCheck {
                habit_id: sunlight_walk.id,
                value: 10,
                checked_at: now,
            })
            .await?;

        self.check_repo
            .upsert(&NewCheck {
                habit_id: deep_work.id,
                value: 25,
                checked_at: now,
            })
            .await?;

        self.check_repo
            .upsert(&NewCheck {
                habit_id: cardio.id,
                value: 60,
                checked_at: now,
            })
            .await?;

        Ok(())
    }
}

#[async_trait]
impl DemoUserService for DefaultDemoUserService {
    async fn create(&self) -> Result<User, AppError> {
        let username = format!("demo_{}", Uuid::new_v4());
        let password = format!("{}", Uuid::new_v4());

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .to_string();

        let user = self
            .user_repo
            .create(&NewUser {
                username,
                password_hash,
            })
            .await?;
        self.demo_user_repo.create(user.id).await?;
        self.seed(&user.id).await?;

        Ok(user)
    }

    async fn delete_all_older_than(&self, timestamp: OffsetDateTime) -> Result<(), AppError> {
        self.demo_user_repo.delete_all_older_than(timestamp).await
    }
}