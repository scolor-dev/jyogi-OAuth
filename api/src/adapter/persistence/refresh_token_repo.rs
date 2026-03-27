use sqlx::PgPool;
use uuid::Uuid;

/// リフレッシュトークン作成
pub async fn create(
    pool: &PgPool,
    session_id: i64,
    token_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO refresh_tokens (session_id, token_hash, expires_at) \
         VALUES ($1, $2, NOW() + INTERVAL '30 days')",
    )
    .bind(session_id)
    .bind(token_hash)
    .execute(pool)
    .await?;
    Ok(())
}

/// token_hashからuser_uuidを取得
pub async fn find_user_uuid_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT s.user_uuid \
         FROM refresh_tokens rt \
         JOIN sessions s ON s.id = rt.session_id \
         WHERE rt.token_hash = $1 \
           AND rt.revoked_at IS NULL \
           AND rt.expires_at > NOW() \
           AND s.revoked_at IS NULL \
           AND s.expires_at > NOW()",
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
}

/// token_hashでリフレッシュトークンを無効化
pub async fn revoke_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE refresh_tokens SET revoked_at = NOW() \
         WHERE token_hash = $1 AND revoked_at IS NULL",
    )
    .bind(token_hash)
    .execute(pool)
    .await?;
    Ok(())
}