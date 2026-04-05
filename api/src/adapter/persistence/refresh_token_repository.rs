use uuid::Uuid;
use sqlx::{PgConnection, PgPool};

use crate::{
    domain::models::refresh_token::RefreshToken,
    error::AppError,
};

pub async fn find_by_hash(pool: &PgPool, token_hash: &str) -> Result<Option<RefreshToken>, AppError> {
    sqlx::query_as!(
        RefreshToken,
        "SELECT id, token_hash, session_id, user_id, user_uuid, session_uuid,
                is_used, expires_at, created_at
         FROM refresh_tokens WHERE token_hash = $1",
        token_hash
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    session_id: i64,
    user_id: i64,
    user_uuid: Uuid,
    session_uuid: Uuid,
    token_hash: &str,
) -> Result<RefreshToken, AppError> {
    sqlx::query_as!(
        RefreshToken,
        "INSERT INTO refresh_tokens (token_hash, session_id, user_id, user_uuid, session_uuid)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, token_hash, session_id, user_id, user_uuid, session_uuid,
                   is_used, expires_at, created_at",
        token_hash,
        session_id,
        user_id,
        user_uuid,
        session_uuid,
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

/// ローテーション時に古いトークンを使用済みにする。
/// 必ず create とtx内でセットで呼ぶこと。
pub async fn mark_used(conn: &mut PgConnection, token_hash: &str) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE refresh_tokens SET is_used = true WHERE token_hash = $1",
        token_hash
    )
    .execute(conn)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}

/// ログアウト時にセッション単位で全トークンを無効化する。
pub async fn revoke_all_by_session(pool: &PgPool, session_id: i64) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE refresh_tokens SET is_used = true WHERE session_id = $1",
        session_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}