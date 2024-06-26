use crate::config::ENV;
use crate::dto::auth::internal::AccessToken;
use crate::enums::errors::internal::{to_internal, InternalError};
use crate::logger::{enums::Services::TokenUtils, ResultExt};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

pub async fn generate_access_token(
    user_id: &str,
    device_id: &str,
) -> Result<String, InternalError> {
    let now = Local::now();
    let access_token_values = AccessToken {
        exp: (now + Duration::hours(1)).timestamp(),
        iat: now.timestamp(),
        device_id: Uuid::parse_str(&device_id)
            .map_err(to_internal)
            .log_error(TokenUtils)
            .await?,
        user_id: Uuid::parse_str(&user_id)
            .map_err(to_internal)
            .log_error(TokenUtils)
            .await?,
    };

    let access_token = encode(
        &Header::default(),
        &access_token_values,
        &EncodingKey::from_secret(ENV.jwt_access_secret.as_ref()),
    )
    .map_err(to_internal)
    .log_error(TokenUtils)
    .await?;

    Ok(access_token)
}
