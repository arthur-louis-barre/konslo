use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, PartialEq, Serialize)]
pub struct Habit {
    pub id: i32,
    pub name: String,
}

impl Habit {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.into()
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}





#[derive(Debug)]
pub struct Check {
    pub id: i64,
    pub habit_id: i64,
    pub checked_date: chrono::NaiveDate,
}