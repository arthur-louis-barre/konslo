use sqlx::Error as SqlxError;
use sqlx::error::DatabaseError;
use crate::errors::AppError;

pub mod habits;
pub const PG_UNIQUE_VIOLATION: &str = "23505";

pub fn to_app_error(e: SqlxError) -> AppError {
    match e {
        SqlxError::Database(db_err) if code_eq(db_err.as_ref(), PG_UNIQUE_VIOLATION) => AppError::Conflict("already exists".to_string()),
        other => AppError::Database(other),
    }
}

fn code_eq(db_err: &dyn DatabaseError, code: &str) -> bool {
    db_err.code().as_deref() == Some(code)
}
