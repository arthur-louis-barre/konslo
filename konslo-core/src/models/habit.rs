use crate::models::check::Check;
use time::OffsetDateTime;

#[derive(Clone, Copy, Debug, PartialEq, sqlx::Type)]
#[sqlx(type_name = "goal_period_enum", rename_all = "lowercase")]
pub enum GoalPeriod {
    Day,
    Week,
    Month,
}

#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, PartialEq)]
pub struct HabitWithCheck {
    pub id: i32,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
    pub created_at: OffsetDateTime,
    pub checks: Vec<Check>,
}

#[derive(Debug)]
pub struct CreateHabit {
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
}
