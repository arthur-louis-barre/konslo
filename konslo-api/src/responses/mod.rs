use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use konslo_core::models::check::Check;

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct CheckResponse {
    pub id: i32,
    pub habit_id: i32,
    pub value: i32,
    pub checked_at: OffsetDateTime,
}

impl From<Check> for CheckResponse {
    fn from(check: Check) -> Self {
        CheckResponse {
            id: check.id,
            habit_id: check.habit_id,
            value: check.value,
            checked_at: check.checked_at,
        }
    }
}