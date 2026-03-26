use serde::Deserialize;
use time::OffsetDateTime;
use crate::responses::GoalPeriodDto;

#[derive(Deserialize)]
pub struct CreateHabitRequest {
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriodDto,
}

#[derive(Deserialize)]
pub struct CreateCheckRequest {
    pub value: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub checked_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct UpdateCheckRequest {
    pub value: i32,
}

#[derive(Deserialize)]
pub struct HabitsQuery {
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub date: Option<OffsetDateTime>,
}