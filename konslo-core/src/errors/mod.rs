use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum AppError {
    Conflict(String),
    Database(SqlxError),
    Validation(String),
    NotFound(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Conflict(msg) => write!(f, "Duplicate error: {msg}"),
            AppError::Database(e) => write!(f, "Database error: {e}"),
            AppError::Validation(msg) => write!(f, "Validation error: {msg}"),
            AppError::NotFound(msg) => write!(f, "Not found error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
