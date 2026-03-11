use crate::errors::AppError;

pub fn validate_habit_name(name: &str) -> Result<(), AppError> {
    let name = name.trim();
    if name.is_empty() {
        Err(AppError::Validation("habit name is empty".into()))
    } else if name.chars().count() > 255 {
        Err(AppError::Validation("habit name is too long".into()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_habit_name_empty_returns_validation_error() {
        let result = validate_habit_name("");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_validate_habit_name_whitespace_only_returns_validation_error() {
        let result = validate_habit_name("      ");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_validate_habit_name_255_chars_returns_ok() {
        let name = "X".repeat(255);

        let result = validate_habit_name(name.as_ref());

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_habit_name_256_chars_returns_validation_error() {
        let name = "X".repeat(256);

        let result = validate_habit_name(name.as_ref());

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }
}
