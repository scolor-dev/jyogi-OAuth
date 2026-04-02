use chrono::{DateTime, Utc};

/// `user_credentials.credential_type` に対応する enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialType {
    Password,
    // 将来: Totp, RecoveryCode, Passkey, ...
}

/// `user_credentials` テーブルに対応するドメインモデル。
/// secret にはパスワードハッシュ等を格納する。
#[derive(Debug, Clone)]
pub struct UserCredential {
    pub id: i64,
    pub user_id: i64,
    pub credential_type: CredentialType,
    /// パスワードハッシュ / TOTPシークレット / passkey公開鍵 など
    pub secret: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}