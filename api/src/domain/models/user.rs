use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuthCredential {
    pub user_id: i64,
    pub user_uuid: Uuid,
    pub secret_hash: String,
    pub cred_type: String, // password, discord, googleなど
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub uuid: Uuid,
    pub display_name: String,
    pub identifier: String,
}