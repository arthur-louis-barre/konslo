use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct Check {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

#[derive(Debug, PartialEq)]
pub struct NewCheck {
    pub habit_id: Uuid,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}
