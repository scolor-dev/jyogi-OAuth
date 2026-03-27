use jsonwebtoken::{
    decode, encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    errors::api::ApiError,
    domain::models::token::Claims,
    state::AppState
};

/// アクセストークンの有効期限（分）
const ACCESS_TOKEN_EXPIRE_MINUTES: i64 = 15;

//
// ========================
// Token生成
// ========================
//

/// アクセストークン生成
pub fn generate_access_token(
    state: &AppState,
    user_uuid: Uuid,
) -> Result<String, ApiError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let exp = now + ACCESS_TOKEN_EXPIRE_MINUTES * 60;

    let claims = Claims {
        sub: user_uuid.to_string(),
        iat: now as usize,
        exp: exp as usize,
    };

    encode(
        &Header::default(), // HS256
        &claims,
        &EncodingKey::from_secret(state.jwt_secret().as_bytes()),
    )
    .map_err(ApiError::from)
}

//
// ========================
// Token検証
// ========================
//

/// アクセストークン検証
pub fn verify_access_token(
    state: &AppState,
    token: &str,
) -> Result<Claims, ApiError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret().as_bytes()),
        &validation,
    )
    .map_err(|_| ApiError::Unauthorized)?;

    Ok(token_data.claims)
}

//
// ========================
// Utility
// ========================
//

/// Claimsからuser_id取得
pub fn user_id_from_claims(claims: &Claims) -> Result<Uuid, ApiError> {
    Uuid::parse_str(&claims.sub).map_err(|_| ApiError::Unauthorized)
}