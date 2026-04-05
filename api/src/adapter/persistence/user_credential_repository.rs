use sqlx::{PgConnection, PgPool};

use crate::{
    domain::models::user_credential::UserCredential,
    error::AppError,
};

pub async fn find_by_user_and_type(
    pool: &PgPool,
    user_id: i64,
    credential_type: &str,
) -> Result<Option<UserCredential>, AppError> {
    sqlx::query_as!(
        UserCredential,
        "SELECT id, user_id, credential_type, secret, created_at, updated_at
         FROM user_credentials
         WHERE user_id = $1 AND credential_type = $2",
        user_id,
        credential_type,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    user_id: i64,
    credential_type: &str,
    secret: &str,
) -> Result<UserCredential, AppError> {
    sqlx::query_as!(
        UserCredential,
        "INSERT INTO user_credentials (user_id, credential_type, secret)
         VALUES ($1, $2, $3)
         RETURNING id, user_id, credential_type, secret, created_at, updated_at",
        user_id,
        credential_type,
        secret,
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn update_secret(
    pool: &PgPool,
    user_id: i64,
    credential_type: &str,
    new_secret: &str,
) -> Result<UserCredential, AppError> {
    sqlx::query_as!(
        UserCredential,
        "UPDATE user_credentials SET secret = $1, updated_at = now()
         WHERE user_id = $2 AND credential_type = $3
         RETURNING id, user_id, credential_type, secret, created_at, updated_at",
        new_secret,
        user_id,
        credential_type,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}