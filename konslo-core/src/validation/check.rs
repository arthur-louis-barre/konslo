use crate::errors::AppError;
use time::{Date, Duration, OffsetDateTime};

pub fn validate_check_value(value: i32) -> Result<(), AppError> {
    if value > 0 {
        Ok(())
    } else {
        Err(AppError::Validation("value must be strictly positive".into()))
    }
}

pub fn validate_check_checked_at(checked_at: &OffsetDateTime) -> Result<(), AppError> {
    if checked_at <= &(OffsetDateTime::now_utc() + Duration::seconds(30)) {
        Ok(())
    } else {
        Err(AppError::Validation("checked_at must not be in the future".into()))
    }
}

pub fn validate_date_range(from: Date, to: Date) -> Result<(), AppError> {
    if from <= to {
        Ok(())
    } else {
        Err(AppError::Validation("'from' must not be after 'to'".into()))
    }
}

pub fn validate_period_cap(check_val: i32, period_total: i32, goal_value: i32) -> Result<(), AppError> {
    if check_val + period_total <= goal_value {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "period total must not exceed goal ({} / {})",
            period_total + check_val,
            goal_value
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_check_value_positive_returns_ok() {
        assert!(validate_check_value(5).is_ok());
    }

    #[test]
    fn test_validate_check_value_zero_returns_validation_error() {
        assert!(matches!(validate_check_value(0), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_validate_check_value_negative_returns_validation_error() {
        assert!(matches!(validate_check_value(-1), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_validate_checked_at_past_returns_ok() {
        assert!(validate_check_checked_at(&(OffsetDateTime::now_utc() - Duration::hours(1))).is_ok());
    }

    #[test]
    fn test_validate_check_checked_at_now_returns_ok() {
        assert!(validate_check_checked_at(&OffsetDateTime::now_utc()).is_ok());
    }

    #[test]
    fn test_validate_check_checked_at_within_grace_window_returns_ok() {
        assert!(validate_check_checked_at(&(OffsetDateTime::now_utc() + Duration::seconds(15))).is_ok());
    }

    #[test]
    fn test_validate_check_checked_at_past_grace_window_returns_validation_error() {
        let result = validate_check_checked_at(&(OffsetDateTime::now_utc() + Duration::seconds(31)));
        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[test]
    fn test_validate_date_range_from_before_to_returns_ok() {
        let from = Date::from_calendar_date(2026, time::Month::January, 1).unwrap();
        let to = Date::from_calendar_date(2026, time::Month::January, 31).unwrap();
        assert!(validate_date_range(from, to).is_ok());
    }

    #[test]
    fn test_validate_date_range_from_equals_to_returns_ok() {
        let date = Date::from_calendar_date(2026, time::Month::January, 1).unwrap();
        assert!(validate_date_range(date, date).is_ok());
    }

    #[test]
    fn test_validate_date_range_from_after_to_returns_validation_error() {
        let from = Date::from_calendar_date(2026, time::Month::January, 31).unwrap();
        let to = Date::from_calendar_date(2026, time::Month::January, 1).unwrap();
        assert!(matches!(validate_date_range(from, to), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_validate_period_cap_under_cap_returns_ok() {
        assert!(validate_period_cap(5, 3, 10).is_ok());
    }

    #[test]
    fn test_validate_period_cap_exactly_at_cap_returns_ok() {
        assert!(validate_period_cap(5, 5, 10).is_ok());
    }

    #[test]
    fn test_validate_period_cap_over_cap_returns_validation_error() {
        assert!(matches!(validate_period_cap(6, 5, 10), Err(AppError::Validation(_))));
    }
}
