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

mod tests {
    use super::*;

    #[test]
    fn test_validate_habit_name_empty_returns_validation_error() {
        let result = validate_habit_name("");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_validate_habit_name_too_long_returns_validation_error() {
        let ok_long_name = "X".repeat(255);
        let too_long_name = "x".repeat(256);

        let result_2= validate_habit_name(ok_long_name.as_ref());
        let result_3 = validate_habit_name(too_long_name.as_ref());

        assert!(result_2.is_ok());
        assert!(result_3.is_err());
        assert!(matches!(result_3.unwrap_err(), AppError::Validation(_)));
    }
}