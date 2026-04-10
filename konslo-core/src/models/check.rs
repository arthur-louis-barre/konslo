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

impl Check {
    pub fn new(id: Uuid, habit_id: Uuid, value: i32, checked_at: OffsetDateTime) -> Self {
        Self {
            id,
            habit_id,
            value,
            checked_at,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AddCheck {
    pub habit_id: Uuid,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

impl AddCheck {
    pub fn new(habit_id: Uuid, value: i32, checked_at: OffsetDateTime) -> Self {
        Self {
            habit_id,
            value,
            checked_at,
        }
    }
}
