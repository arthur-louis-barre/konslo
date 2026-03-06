use crate::errors::AppError;

pub fn validate_habit_name(name: &str) -> Result<(), AppError> {
    let name = name.trim();
    if name.is_empty() { Err(AppError::Validation("habit name is empty".into())) }
    else if name.chars().count() > 255 { Err(AppError::Validation("habit name is too long".into())) }
    else { Ok(())}
}