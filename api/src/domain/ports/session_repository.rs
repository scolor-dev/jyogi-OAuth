use async_trait::async_trait;
use std::net::IpAddr;
use uuid::Uuid;

use crate::domain::models::session::Session;
use crate::error::AppError;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<Session>, AppError>;
    async fn find_by_uuid(&self, session_uuid: Uuid) -> Result<Option<Session>, AppError>;

    async fn create(
        &self,
        user_id: i64,
        user_uuid: Uuid,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> Result<Session, AppError>;

    async fn touch(&self, id: i64) -> Result<(), AppError>;
    async fn revoke(&self, id: i64) -> Result<(), AppError>;
    async fn revoke_all_by_user(&self, user_id: i64) -> Result<(), AppError>;
}