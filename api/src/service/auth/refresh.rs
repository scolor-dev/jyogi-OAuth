use hex::encode as hex_encode;
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    adapter::persistence::{session_repo, refresh_token_repo},
    errors::api::ApiError,
};

//
// ========================
// Token生成
// ========================
//

/// 32バイトのランダムトークン生成（クライアント用）
pub fn generate_refresh_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex_encode(bytes)
}

/// トークンをSHA-256でハッシュ化（DB保存用）
pub fn hash_refresh_token(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex_encode(digest)
}

//
// ========================
// DB操作
// ========================
//

/// セッション＆リフレッシュトークンを保存（login用）
pub async fn store_refresh_token(
    pool: &PgPool,
    user_uuid: Uuid,
    token_hash: &str,
) -> Result<(), ApiError> {
    // 1. セッション作成 → session_id取得
    let session_id = session_repo::create(pool, user_uuid).await?;
    // 2. リフレッシュトークン保存
    refresh_token_repo::create(pool, session_id, token_hash).await?;
    Ok(())
}

/// リフレッシュトークン検証（user_uuid取得）
pub async fn verify_refresh_token(
    pool: &PgPool,
    raw_token: &str,
) -> Result<Uuid, ApiError> {
    let token_hash = hash_refresh_token(raw_token);
    // refresh_tokens → session_id → sessions.user_uuid
    let user_uuid = refresh_token_repo::find_user_uuid_by_hash(pool, &token_hash)
        .await?
        .ok_or(ApiError::Unauthorized)?;
    Ok(user_uuid)
}

/// リフレッシュトークン無効化（logout用）
pub async fn revoke_refresh_token(
    pool: &PgPool,
    raw_token: &str,
) -> Result<(), ApiError> {
    let token_hash = hash_refresh_token(raw_token);
    refresh_token_repo::revoke_by_hash(pool, &token_hash).await?;
    Ok(())
}