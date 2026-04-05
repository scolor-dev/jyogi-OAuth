use chrono::{DateTime, Utc};
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub id: i64,
    pub session_uuid: Uuid,
    pub user_id: i64,
    pub user_uuid: Uuid,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub last_active_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn is_valid(&self) -> bool {
        self.revoked_at.is_none() && self.expires_at > chrono::Utc::now()
    }
}