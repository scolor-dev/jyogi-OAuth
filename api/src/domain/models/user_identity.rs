use chrono::{DateTime, Utc};

/// `user_identities.type` に対応する enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityType {
    Username,
    // 将来: Email, OAuthGoogle, OAuthDiscord, ...
}

/// `user_identities` テーブルに対応するドメインモデル。
/// ログイン識別子（username等）を users と分離して保持する。
#[derive(Debug, Clone)]
pub struct UserIdentity {
    pub id: i64,
    pub user_id: i64,
    pub identity_type: IdentityType,
    /// アプリ側で正規化済みの識別子
    pub identifier: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}