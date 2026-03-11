use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Serialize)]
pub struct Check {
    pub id: i32,
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

impl Check {
    pub fn new(id: i32, habit_id: i32, value: i32) -> Self {
        Self {
            id,
            habit_id,
            value,
            checked_at: OffsetDateTime::now_utc(),
        }
    }
}
