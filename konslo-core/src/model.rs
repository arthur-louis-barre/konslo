use sqlx::FromRow;
use serde::Serialize;

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
}