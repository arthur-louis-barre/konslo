use crate::errors::habit::HabitError;

pub fn validate_habit_name(name: &str) -> Result<(), HabitError> {
    let name = name.trim();
    if name.is_empty() { Err(HabitError::InvalidName("habit name is empty".into())) }
    else if name.chars().count() > 255 { Err(HabitError::InvalidName("habit name is too long".into())) }
    else { Ok(())}
}