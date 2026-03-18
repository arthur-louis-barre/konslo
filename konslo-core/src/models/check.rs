use time::OffsetDateTime;

#[derive(sqlx::FromRow, Clone, Debug, PartialEq)]
pub struct Check {
    pub id: i32,
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

pub struct CreateCheck {
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

pub struct UpdateCheck {
    pub id: i32,
    pub value: i32,
}