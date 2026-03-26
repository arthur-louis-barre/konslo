use crate::errors::AppError;
use time::{Duration, OffsetDateTime};

pub fn validate_check_value(value: i32, habit_goal_value: i32) -> Result<(), AppError> {
    if value < 0 {
        return Err(AppError::Validation("value must be greater than 0".into()));
    } else if value > habit_goal_value {
        return Err(AppError::Validation(
            "value can't be greater than habit's goal_value".into(),
        ));
    }

    Ok(())
}

pub fn validate_checked_at(checked_at: &OffsetDateTime) -> Result<(), AppError> {
    if checked_at > &(OffsetDateTime::now_utc() + Duration::seconds(30)) {
        return Err(AppError::Validation("checked_at cannot be in the future".into()));
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_check_value_ok() {
        assert!(validate_check_value(5, 10).is_ok());
    }

    #[test]
    fn test_validate_check_value_negative_returns_validation_error() {
        let result = validate_check_value(-1, 10);
        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[test]
    fn test_validate_check_value_exceeds_goal_returns_validation_error() {
        // arrange
        let value = 15;
        let habit_goal_value = 10;

        // act
        let result = validate_check_value(value, habit_goal_value);

        // assert
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_validate_checked_at_ok() {
        // arrange
        let checked_at = OffsetDateTime::now_utc();

        // act
        let result = validate_checked_at(&checked_at);

        // assert
        assert!(result.is_ok())
    }

    #[test]
    fn test_validate_checked_at_future_returns_validation_error() {
        // arrange
        let checked_at = OffsetDateTime::now_utc() + Duration::seconds(60);

        // act
        let result = validate_checked_at(&checked_at);

        // assert
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)))
    }
}
