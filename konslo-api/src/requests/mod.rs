use crate::responses::GoalPeriodDto;
use time::OffsetDateTime;

#[derive(serde::Deserialize)]
pub struct CreateHabitRequest {
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriodDto,
}

#[derive(serde::Deserialize)]
pub struct AddCheckRequest {
    pub value: i32,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub checked_at: Option<OffsetDateTime>,
}

#[derive(serde::Deserialize)]
pub struct HabitsQuery {
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub date: Option<OffsetDateTime>,
}

#[derive(serde::Deserialize)]
pub struct ResetChecksQuery {
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub date: Option<OffsetDateTime>,
}
