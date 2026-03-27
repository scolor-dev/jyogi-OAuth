use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::models::user::{AuthCredential, UserRow};

/// signup 時のユーザー作成（4テーブルへのトランザクション挿入）
pub async fn create_user(
    pool: &PgPool,
    identifier: &str,
    password_hash: &str,
    display_name: &str,
) -> Result<Uuid, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let user_uuid = Uuid::new_v4();
    let normalized = identifier.to_lowercase();

    // users
    let user_id: i64 = sqlx::query_scalar(
        "INSERT INTO users (uuid, status) VALUES ($1, 'active') RETURNING id",
    )
    .bind(user_uuid)
    .fetch_one(&mut *tx)
    .await?;

    // user_profile
    sqlx::query(
        "INSERT INTO user_profile (user_id, display_name) VALUES ($1, $2)",
    )
    .bind(user_id)
    .bind(display_name)
    .execute(&mut *tx)
    .await?;

    // user_identities
    sqlx::query(
        "INSERT INTO user_identities (user_id, type, identifier, normalized_identifier, is_primary) \
         VALUES ($1, 'username', $2, $3, true)",
    )
    .bind(user_id)
    .bind(identifier)
    .bind(&normalized)
    .execute(&mut *tx)
    .await?;

    // user_credentials
    sqlx::query(
        "INSERT INTO user_credentials (user_id, type, secret_hash, is_primary) \
         VALUES ($1, 'password', $2, true)",
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(user_uuid)
}

/// identifier からユーザー認証情報を取得
pub async fn find_credential_by_identifier(
    pool: &PgPool,
    identifier: &str,
) -> Result<Option<AuthCredential>, sqlx::Error> {
    let normalized = identifier.to_lowercase();
    sqlx::query_as::<_, AuthCredential>(
        "SELECT u.id as user_id, u.uuid as user_uuid, uc.secret_hash, uc.type as cred_type \
         FROM user_credentials uc \
         JOIN user_identities ui ON ui.user_id = uc.user_id \
         JOIN users u ON u.id = uc.user_id \
         WHERE ui.normalized_identifier = $1 \
           AND ui.revoked_at IS NULL \
           AND uc.revoked_at IS NULL",
    )
    .bind(&normalized)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_uuid(
    pool: &PgPool,
    user_uuid: Uuid,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT u.uuid, up.display_name, ui.identifier \
         FROM users u \
         JOIN user_profile up ON up.user_id = u.id \
         JOIN user_identities ui ON ui.user_id = u.id \
         WHERE u.uuid = $1 \
           AND ui.is_primary = true \
           AND ui.revoked_at IS NULL",
    )
    .bind(user_uuid)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id(
    pool: &PgPool,
    user_id: i64,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT u.uuid, up.display_name, ui.identifier \
         FROM users u \
         JOIN user_profile up ON up.user_id = u.id \
         JOIN user_identities ui ON ui.user_id = u.id \
         WHERE u.id = $1 \
           AND ui.is_primary = true \
           AND ui.revoked_at IS NULL",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}