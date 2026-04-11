use sqlx::Error as SqlxError;
use sqlx::error::DatabaseError;

pub const PG_UNIQUE_VIOLATION: &str = "23505";
pub const PG_FK_VIOLATION: &str = "23503";
pub const PG_CONSTRAINT_VIOLATION: &str = "23514";

#[derive(Debug)]
pub enum AppError {
    Conflict(String),
    Database(SqlxError),
    Internal(String),
    NotFound(String),
    Unauthorized,
    Validation(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Conflict(msg) => write!(f, "Conflict: {msg}"),
            AppError::Database(e) => write!(f, "Database: {e}"),
            AppError::Internal(msg) => write!(f, "Internal: {msg}"),
            AppError::NotFound(msg) => write!(f, "Not found: {msg}"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Validation(msg) => write!(f, "Validation: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        match e {
            SqlxError::Database(db_err) if code_eq(db_err.as_ref(), PG_UNIQUE_VIOLATION) => {
                AppError::Conflict("unique violation".to_string())
            }
            SqlxError::Database(db_err) if code_eq(db_err.as_ref(), PG_FK_VIOLATION) => {
                AppError::NotFound("fk violation".to_string())
            }
            SqlxError::Database(db_err) if code_eq(db_err.as_ref(), PG_CONSTRAINT_VIOLATION) => {
                AppError::Validation("constraint violation".to_string())
            }
            other => AppError::Database(other),
        }
    }
}

fn code_eq(db_err: &dyn DatabaseError, code: &str) -> bool {
    db_err.code().as_deref() == Some(code)
}
