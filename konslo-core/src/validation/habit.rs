use crate::errors::AppError;

pub fn validate_habit_name(name: &str) -> Result<(), AppError> {
    let name = name.trim();

    match name.len() {
        0 => Err(AppError::Validation("habit name must not be empty".into())),
        101.. => Err(AppError::Validation("habit name must be at most 100 characters".into())),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_habit_name_ok() {
        assert!(validate_habit_name("Meditate").is_ok());
    }

    #[test]
    fn test_validate_habit_name_empty_returns_validation_error() {
        assert!(matches!(validate_habit_name("").unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_validate_habit_name_whitespace_only_returns_validation_error() {
        assert!(matches!(validate_habit_name("      ").unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_validate_habit_name_101_chars_returns_validation_error() {
        assert!(matches!(validate_habit_name(&"x".repeat(101)).unwrap_err(), AppError::Validation(_)));
    }
}
