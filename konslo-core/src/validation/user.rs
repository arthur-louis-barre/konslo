use crate::errors::AppError;

pub fn validate_username(username: &str) -> Result<(), AppError> {
    let username = username.trim();
    let length = username.len();

    if length < 4 {
        Err(AppError::Validation(
            "username must be at least 4 characters".into(),
        ))
    } else if length > 20 {
        Err(AppError::Validation(
            "username must be at most 20 characters".into(),
        ))
    } else if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        Err(AppError::Validation(
            "username may only contain letters, digits, underscores, and hyphens".into(),
        ))
    } else if username.eq_ignore_ascii_case("demo") {
        Err(AppError::Validation("username \'demo\' is reserved".into()))
    } else {
        Ok(())
    }
}

pub fn validate_password(password: &str) -> Result<(), AppError> {
    let length = password.len();

    if length < 8 {
        Err(AppError::Validation(
            "password must be at least 8 characters".into(),
        ))
    // Argon2 silently truncates passwords beyond 72 bytes
    } else if length > 72 {
        Err(AppError::Validation(
            "password must be at most 72 characters".into(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username_ok() {
        assert!(validate_username("Tocqueville").is_ok());
    }

    #[test]
    fn test_validate_username_3_chars_returns_validation_error() {
        assert!(matches!(
            validate_username("bob").unwrap_err(),
            AppError::Validation(_)
        ));
    }

    #[test]
    fn test_validate_username_4_chars_ok() {
        assert!(validate_username(&"x".repeat(4)).is_ok());
    }

    #[test]
    fn test_validate_username_20_chars_ok() {
        assert!(validate_username(&"x".repeat(20)).is_ok());
    }

    #[test]
    fn test_validate_username_21_chars_returns_validation_error() {
        assert!(matches!(
            validate_username(&"x".repeat(21)).unwrap_err(),
            AppError::Validation(_)
        ));
    }

    #[test]
    fn test_validate_username_invalid_chars_returns_validation_error() {
        assert!(matches!(
            validate_username("Alexis!de!Tocqueville").unwrap_err(),
            AppError::Validation(_)
        ));
    }

    #[test]
    fn test_validate_username_demo_returns_validation_error() {
        assert!(matches!(
            validate_username("demo").unwrap_err(),
            AppError::Validation(_)
        ));
    }

    #[test]
    fn test_validate_password_ok() {
        assert!(validate_password("ArT--ZeY--24_48").is_ok());
    }

    #[test]
    fn test_validate_password_7_chars_returns_validation_error() {
        assert!(matches!(
          validate_password(&"x".repeat(7)).unwrap_err(),
          AppError::Validation(_)
      ));
    }

    #[test]
    fn test_validate_password_8_chars_ok() {
        assert!(validate_password(&"x".repeat(8)).is_ok());
    }

    #[test]
    fn test_validate_password_72_chars_ok() {
        assert!(validate_password(&"x".repeat(72)).is_ok());
    }

    #[test]
    fn test_validate_password_73_chars_returns_validation_error() {
        assert!(matches!(
          validate_password(&"x".repeat(73)).unwrap_err(),
          AppError::Validation(_)
      ));
    }
}
