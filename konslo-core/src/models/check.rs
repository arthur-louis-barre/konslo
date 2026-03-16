use time::OffsetDateTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Check {
    pub id: i32,
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct CreateCheck {
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct UpdateCheck {
    pub id: i32,
    pub value: i32,
}