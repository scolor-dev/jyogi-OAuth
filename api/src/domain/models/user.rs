use chrono::{DateTime, Utc};
use uuid::Uuid;

/// users.status に対応する enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Pending,
    Active,
    Inactive,
    Suspended,
}

/// `users` テーブルに対応するドメインモデル。
/// id(BIGSERIAL) と uuid を分離した設計に合わせる。
#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub uuid: Uuid,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active && self.deleted_at.is_none()
    }
}