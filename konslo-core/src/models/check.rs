use time::OffsetDateTime;
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct Check {
    pub id: i32,
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

impl Check {
    pub fn new(id: i32, habit_id: i32, value: i32, checked_at: OffsetDateTime) -> Self {
        Self {
            id,
            habit_id,
            value,
            checked_at
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AddCheck {
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

impl AddCheck {
    pub fn new(habit_id: i32, value: i32, checked_at: OffsetDateTime) -> Self {
        Self {
            habit_id,
            value,
            checked_at,
        }
    }
}
