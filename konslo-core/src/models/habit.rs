use serde::{Deserialize, Serialize};
use sqlx;
use time::OffsetDateTime;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "goal_period_enum", rename_all = "lowercase")]
pub enum GoalPeriod {
    Day,
    Week,
    Month,
}

#[derive(Debug, Deserialize)]
pub struct CreateHabit {
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
}

#[derive(Debug, PartialEq, Serialize, sqlx::FromRow, )]
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
    pub created_at: OffsetDateTime,
}