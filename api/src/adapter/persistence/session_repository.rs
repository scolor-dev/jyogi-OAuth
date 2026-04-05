use std::net::IpAddr;
use uuid::Uuid;
use sqlx::{PgConnection, PgPool};

use crate::{
    domain::models::session::Session,
    error::AppError,
};

pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Session>, AppError> {
    sqlx::query_as!(
        Session,
        "SELECT id, session_uuid, user_id, user_uuid, ip_address, user_agent,
                last_active_at, expires_at, revoked_at, created_at, updated_at
         FROM sessions WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn find_by_uuid(pool: &PgPool, session_uuid: Uuid) -> Result<Option<Session>, AppError> {
    sqlx::query_as!(
        Session,
        "SELECT id, session_uuid, user_id, user_uuid, ip_address, user_agent,
                last_active_at, expires_at, revoked_at, created_at, updated_at
         FROM sessions WHERE session_uuid = $1",
        session_uuid
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    user_id: i64,
    user_uuid: Uuid,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> Result<Session, AppError> {
    sqlx::query_as!(
        Session,
        "INSERT INTO sessions (session_uuid, user_id, user_uuid, ip_address, user_agent)
         VALUES (gen_random_uuid(), $1, $2, $3, $4)
         RETURNING id, session_uuid, user_id, user_uuid, ip_address, user_agent,
                   last_active_at, expires_at, revoked_at, created_at, updated_at",
        user_id,
        user_uuid,
        ip_address as Option<IpAddr>,
        user_agent,
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn touch(pool: &PgPool, id: i64) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE sessions SET last_active_at = now(), updated_at = now() WHERE id = $1",
        id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}

pub async fn revoke(pool: &PgPool, id: i64) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE sessions SET revoked_at = now(), updated_at = now() WHERE id = $1",
        id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}

pub async fn revoke_all_by_user(pool: &PgPool, user_id: i64) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE sessions SET revoked_at = now(), updated_at = now()
         WHERE user_id = $1 AND revoked_at IS NULL",
        user_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}