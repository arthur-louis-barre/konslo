use crate::error::AppError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    exp: u64,
}

pub fn generate_token(user_id: Uuid, jwt_secret: &[u8]) -> Result<String, AppError> {
    let current = OffsetDateTime::now_utc().unix_timestamp() as u64;
    let exp = current + 60 * 60;

    let claims = Claims { user_id, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret),
    )
    .map_err(|err| {
        tracing::error!("JWT generation failed: {}", err);
        AppError::InternalServerError
    })
}

pub fn verify_token(token: &str, jwt_secret: &[u8]) -> Result<Claims, AppError> {
    decode(
        token,
        &DecodingKey::from_secret(jwt_secret),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|err| {
        tracing::warn!("JWT verification failed: {}", err);
        AppError::Unauthorized
    })
}
