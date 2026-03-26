use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Check {
    pub id: i32,
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

#[derive(Debug, PartialEq)]
pub struct CreateCheck {
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

#[derive(Debug, PartialEq)]
pub struct UpdateCheck {
    pub id: i32,
    pub value: i32,
}
