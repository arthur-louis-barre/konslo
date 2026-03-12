use crate::errors::AppError;
use crate::models::check::CreateCheck;
use std::ops::Add;
use time::{Duration, OffsetDateTime};

pub fn validate_check(new_check: &CreateCheck) -> Result<(), AppError> {
    if new_check.checked_at > OffsetDateTime::now_utc().add(Duration::seconds(30)) {
        Err(AppError::Validation("checked_at cannot be in the future".into()))
    } else if new_check.value <= 0 {
        Err(AppError::Validation("value must be greater than 0".into()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;
    use time::{Duration, OffsetDateTime};

    #[test]
    fn test_validate_check_future_timestamp_returns_error() {
        // arrange
        let new_check = CreateCheck {
            habit_id: 1,
            value: 1,
            checked_at: OffsetDateTime::now_utc().add(Duration::seconds(60)),
        };

        // act
        let result = validate_check(&new_check);

        // assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_validate_check_value_not_positive_returns_error() {
        // arrange
        let new_check_zero = CreateCheck {
            habit_id: 1,
            value: 0,
            checked_at: OffsetDateTime::now_utc(),
        };
        let new_check_neg = CreateCheck {
            value: -1,
            ..new_check_zero
        };

        // act
        let result_zero = validate_check(&new_check_zero);
        let result_neg = validate_check(&new_check_neg);

        // assert
        assert!(result_zero.is_err());
        assert!(matches!(result_zero.unwrap_err(), AppError::Validation(_)));
        assert!(result_neg.is_err());
        assert!(matches!(result_neg.unwrap_err(), AppError::Validation(_)));
    }
}