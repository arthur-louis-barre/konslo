use time::OffsetDateTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, sqlx::Type)]
#[sqlx(type_name = "goal_period_enum", rename_all = "lowercase")]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum GoalPeriod {
    Day,
    Week,
    Month,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct CreateHabit {
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
}

#[derive(Debug, PartialEq, sqlx::FromRow)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
    pub created_at: OffsetDateTime,
}