use chrono::{DateTime, Utc};

/// `roles` テーブルに対応するドメインモデル。
#[derive(Debug, Clone)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// `permissions` テーブルに対応するドメインモデル。
#[derive(Debug, Clone)]
pub struct Permission {
    pub id: i64,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}