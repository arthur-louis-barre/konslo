use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Serialize)]
pub struct Check {
    pub id: i32,
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheck {
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}