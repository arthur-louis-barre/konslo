use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct CreateCheckRequest {
    pub value: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub checked_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct UpdateCheckRequest {
    pub value: i32,
}