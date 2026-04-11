use crate::models::check::Check;
use sqlx::{FromRow, Type};
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, PartialEq, FromRow)]
pub struct Habit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, PartialEq)]
pub struct HabitWithCheck {
    pub id: Uuid,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
    pub created_at: OffsetDateTime,
    pub checks: Vec<Check>,
}

#[derive(Debug)]
pub struct NewHabit {
    pub user_id: Uuid,
    pub name: String,
    pub goal_value: i32,
    pub goal_unit: String,
    pub goal_period: GoalPeriod,
}

#[derive(Clone, Copy, Debug, PartialEq, Type)]
#[sqlx(type_name = "goal_period_enum", rename_all = "lowercase")]
pub enum GoalPeriod {
    Day,
    Week,
    Month,
}

impl GoalPeriod {
    pub fn get_period_start(&self, timestamp: OffsetDateTime) -> Date {
        match self {
            GoalPeriod::Day => timestamp.date(),
            GoalPeriod::Week => {
                let num_days = timestamp.weekday().number_days_from_monday() as i64;
                let first_week_day = timestamp - Duration::days(num_days);
                first_week_day.date()
            }
            GoalPeriod::Month => {
                let first_month_day = timestamp.replace_day(1).unwrap();
                first_month_day.date()
            }
        }
    }

    pub fn get_period_end(&self, timestamp: OffsetDateTime) -> Date {
        match self {
            GoalPeriod::Day => timestamp.date(),
            GoalPeriod::Week => {
                let num_days = timestamp.weekday().number_days_from_monday() as i64;
                let last_week_day = timestamp + Duration::days(6 - num_days);
                last_week_day.date()
            }
            GoalPeriod::Month => {
                let month_length = timestamp.month().length(timestamp.year());
                let last_month_day = timestamp.replace_day(month_length).unwrap();
                last_month_day.date()
            }
        }
    }
}
