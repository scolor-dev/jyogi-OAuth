use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::models::user::Session;

/// セッションを作成する（login 用）
pub async fn create(
    pool: &PgPool,
    user_uuid: Uuid,
) -> Result<i64, sqlx::Error> {
    let session_id = sqlx::query_scalar(
        "INSERT INTO sessions (user_uuid, expires_at) \
         VALUES ($1, NOW() + INTERVAL '30 days') RETURNING id",
    )
    .bind(user_uuid)
    .fetch_one(pool)
    .await?;
    Ok(session_id)
}

/// セッションをIDで取得
pub async fn find_by_id(
    pool: &PgPool,
    session_id: i64,
) -> Result<Option<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        "SELECT id, user_uuid \
         FROM sessions \
         WHERE id = $1 \
           AND revoked_at IS NULL \
           AND expires_at > NOW()",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
}

/// セッションを失効させる（logout 用）
pub async fn revoke_by_id(
    pool: &PgPool,
    session_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE sessions SET revoked_at = NOW() \
         WHERE id = $1 AND revoked_at IS NULL",
    )
    .bind(session_id)
    .execute(pool)
    .await?;
    Ok(())
}