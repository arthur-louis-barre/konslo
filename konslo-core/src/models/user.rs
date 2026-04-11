use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}
