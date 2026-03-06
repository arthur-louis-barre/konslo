use sqlx::Error as SqlxError;
use std::fmt;
use crate::errors::PG_UNIQUE_VIOLATION;

#[derive(Debug)]
pub enum HabitError {
    AlreadyExists,
    Database(SqlxError),
    InvalidName(String),
    NotFound,
}

impl fmt::Display for HabitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HabitError::AlreadyExists => write!(f, "Habit already exists"),
            HabitError::Database(e) => write!(f, "Database error: {}", e),
            HabitError::InvalidName(msg) => write!(f, "Habit name is invalid: {}", msg),
            HabitError::NotFound => write!(f, "Habit not found"),
        }
    }
}

impl From<SqlxError> for HabitError {
    fn from(e: SqlxError) -> Self {
        match e {
            SqlxError::Database(db_err)
                if db_err.code().as_deref() == Some(PG_UNIQUE_VIOLATION) =>
            {
                HabitError::AlreadyExists
            }
            other => HabitError::Database(other),
        }
    }
}
