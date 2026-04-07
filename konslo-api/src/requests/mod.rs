use crate::responses::GoalPeriodDto;
use serde::Deserialize;
use time::{Date, OffsetDateTime};

#[derive(Deserialize)]
pub struct CreateHabitRequest {
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriodDto,
}

#[derive(Deserialize)]
pub struct AddCheckRequest {
    pub value: i32,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub checked_at: Option<OffsetDateTime>,
}

#[derive(Deserialize)]
pub struct HabitsQuery {
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub date: Option<OffsetDateTime>,
}

#[derive(Deserialize)]
pub struct ResetChecksQuery {
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub date: Option<OffsetDateTime>,
}

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Deserialize)]
pub struct ActivityQuery {
    #[serde(with = "date_format")]
    pub from: Date,
    #[serde(with = "date_format")]
    pub to: Date,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}


