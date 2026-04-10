use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;
use konslo_core::models::check::Check;
use konslo_core::models::habit::{GoalPeriod, Habit, HabitWithCheck};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CheckResponse {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

impl From<Check> for CheckResponse {
    fn from(check: Check) -> Self {
        CheckResponse {
            id: check.id,
            habit_id: check.habit_id,
            value: check.value,
            checked_at: check.checked_at,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HabitResponse {
    pub id: Uuid,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriodDto,
    pub created_at: OffsetDateTime,
}

impl From<Habit> for HabitResponse {
    fn from(habit: Habit) -> Self {
        HabitResponse {
            id: habit.id,
            name: habit.name,
            goal_value: habit.goal_value,
            goal_unit: habit.goal_unit,
            goal_period: habit.goal_period.into(),
            created_at: habit.created_at,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HabitWithCheckResponse {
    pub id: Uuid,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriodDto,
    pub created_at: OffsetDateTime,
    pub checks: Vec<CheckResponse>,
}

impl From<HabitWithCheck> for HabitWithCheckResponse {
    fn from(h: HabitWithCheck) -> Self {
        HabitWithCheckResponse {
            id: h.id,
            name: h.name,
            goal_value: h.goal_value,
            goal_unit: h.goal_unit,
            goal_period: h.goal_period.into(),
            created_at: h.created_at,
            checks: h.checks.into_iter().map(CheckResponse::from).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GoalPeriodDto {
    Day,
    Week,
    Month,
}

impl From<GoalPeriod> for GoalPeriodDto {
    fn from(gp: GoalPeriod) -> Self {
        match gp {
            GoalPeriod::Day => GoalPeriodDto::Day,
            GoalPeriod::Week => GoalPeriodDto::Week,
            GoalPeriod::Month => GoalPeriodDto::Month,
        }
    }
}

impl From<GoalPeriodDto> for GoalPeriod {
    fn from(gp: GoalPeriodDto) -> Self {
        match gp {
            GoalPeriodDto::Day => GoalPeriod::Day,
            GoalPeriodDto::Week => GoalPeriod::Week,
            GoalPeriodDto::Month => GoalPeriod::Month,
        }
    }
}

#[derive(Serialize)]
pub struct ActivityResponse {
    pub dates: Vec<String>,
}

impl From<Vec<Date>> for ActivityResponse {
    fn from(dates: Vec<Date>) -> Self {
        Self {
            dates: dates.into_iter().map(|d| d.to_string()).collect(),
        }
    }
}