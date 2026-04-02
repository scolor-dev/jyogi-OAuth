use chrono::{DateTime, Utc};
use uuid::Uuid;

/// `refresh_tokens` テーブルに対応するドメインモデル。
/// session_id に紐づき、is_used で使い捨て（Rotation）を管理する。
#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: i64,
    pub token_hash: String,
    pub session_id: i64,
    pub user_id: i64,

    /// JOINレス用の非正規化フィールド
    pub user_uuid: Uuid,
    pub session_uuid: Uuid,

    pub is_used: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn is_valid(&self) -> bool {
        !self.is_used && self.expires_at > chrono::Utc::now()
    }
}