use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct CreateCheckRequest {
    pub(crate) value: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) checked_at: OffsetDateTime,
}