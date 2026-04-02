use async_trait::async_trait;

use crate::domain::models::user_identity::{IdentityType, UserIdentity};
use crate::error::AppError;

#[async_trait]
pub trait UserIdentityRepository: Send + Sync {
    async fn find_by_identifier(
        &self,
        identity_type: &IdentityType,
        identifier: &str,
    ) -> Result<Option<UserIdentity>, AppError>;

    async fn create(
        &self,
        user_id: i64,
        identity_type: IdentityType,
        identifier: &str,
        is_primary: bool,
    ) -> Result<UserIdentity, AppError>;
}