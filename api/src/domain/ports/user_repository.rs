use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::user::User;
use crate::error::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, AppError>;
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<User>, AppError>;
    async fn create(&self) -> Result<User, AppError>;
    async fn activate(&self, id: i64) -> Result<User, AppError>;
}