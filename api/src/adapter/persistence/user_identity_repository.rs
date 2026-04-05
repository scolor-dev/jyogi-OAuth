use sqlx::{PgConnection, PgPool};

use crate::{
    domain::models::user_identity::UserIdentity,
    error::AppError,
};

pub async fn find_by_identifier(
    pool: &PgPool,
    identity_type: &str,
    identifier: &str,
) -> Result<Option<UserIdentity>, AppError> {
    sqlx::query_as!(
        UserIdentity,
        "SELECT id, user_id, identity_type, identifier, is_primary, created_at, updated_at
         FROM user_identities
         WHERE identity_type = $1 AND identifier = $2",
        identity_type,
        identifier,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    user_id: i64,
    identity_type: &str,
    identifier: &str,
    is_primary: bool,
) -> Result<UserIdentity, AppError> {
    sqlx::query_as!(
        UserIdentity,
        "INSERT INTO user_identities (user_id, identity_type, identifier, is_primary)
         VALUES ($1, $2, $3, $4)
         RETURNING id, user_id, identity_type, identifier, is_primary, created_at, updated_at",
        user_id,
        identity_type,
        identifier,
        is_primary,
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}