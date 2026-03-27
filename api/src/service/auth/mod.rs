pub mod jwt;
pub mod refresh;

use sqlx::PgPool;
use crate::{
    domain::models::auth::{SignupRequest, SignupResponse, LoginRequest, MeResponse},
    errors::api::ApiError,
    adapter::persistence::user_repo,
    state::AppState,
};

/// -------------------------
/// Signup
/// --------------------------
pub async fn signup(pool: &PgPool, req: &SignupRequest) -> Result<SignupResponse, ApiError> {
    if req.identifier.is_empty() || req.password.is_empty() || req.display_name.is_empty() {
        return Err(ApiError::BadRequest(
            "identifier, password, display_name は必須です".to_string(),
        ));
    }
    let secret_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)?;
    let user_uuid = user_repo::create_user(pool, &req.identifier, &secret_hash, &req.display_name).await?;
    Ok(SignupResponse { user_uuid })  // user_id → user_uuid
}

/// -------------------------
/// Login
/// --------------------------
pub const DUMMY_HASH: &str = "$2b$12$dummy.hash.for.timing.attack.prevention.only";

pub async fn login(
    pool: &PgPool,
    state: &AppState,
    req: &LoginRequest,
) -> Result<(String, String), ApiError> {
    let credential = user_repo::find_credential_by_identifier(pool, &req.identifier).await?;
    let (secret_hash, user_found) = credential.as_ref().map_or(
        (DUMMY_HASH, false),
        |c| (c.secret_hash.as_str(), true),
    );
    let valid = bcrypt::verify(&req.password, secret_hash).unwrap_or(false);
    if !user_found || !valid {
        return Err(ApiError::Unauthorized);
    }
    let credential = credential.unwrap();
    let user_uuid = credential.user_uuid;

    // アクセストークン生成
    let access_token = jwt::generate_access_token(state, user_uuid)?;

    // リフレッシュトークン生成・保存
    let refresh_token = refresh::generate_refresh_token();
    let token_hash = refresh::hash_refresh_token(&refresh_token);
    refresh::store_refresh_token(pool, user_uuid, &token_hash).await?;  // user_id → user_uuid
    Ok((access_token, refresh_token))
}

/// -------------------------
/// Refresh
/// --------------------------
pub async fn refresh(
    pool: &PgPool,
    state: &AppState,
    raw_token: &str,
) -> Result<(String, String), ApiError> {
    // 1. 検証 → user_uuid直接取得（find_by_id不要）
    let user_uuid = refresh::verify_refresh_token(pool, raw_token).await?;

    // 2. 古いトークン無効化
    refresh::revoke_refresh_token(pool, raw_token).await?;

    // 3. 新しいアクセストークン生成
    let access_token = jwt::generate_access_token(state, user_uuid)?;

    // 4. 新しいリフレッシュトークン生成・保存
    let new_refresh_token = refresh::generate_refresh_token();
    let token_hash = refresh::hash_refresh_token(&new_refresh_token);
    refresh::store_refresh_token(pool, user_uuid, &token_hash).await?;

    Ok((access_token, new_refresh_token))
}

/// -------------------------
/// Logout
/// --------------------------
pub async fn logout(
    pool: &PgPool,
    raw_token: &str,
) -> Result<(), ApiError> {
    refresh::revoke_refresh_token(pool, raw_token).await?;
    Ok(())
}

/// -------------------------
/// Me
/// --------------------------
pub async fn me(
    pool: &PgPool,
    state: &AppState,
    raw_token: &str,
) -> Result<MeResponse, ApiError> {
    let claims = jwt::verify_access_token(state, raw_token)?;
    let user_uuid = jwt::user_id_from_claims(&claims)?;
    let user = user_repo::find_by_uuid(pool, user_uuid)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(MeResponse {
        user_uuid,
        display_name: user.display_name,
        identifier: user.identifier,
    })
}