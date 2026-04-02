use async_trait::async_trait;

use crate::domain::models::user_credential::{CredentialType, UserCredential};
use crate::error::AppError;

#[async_trait]
pub trait UserCredentialRepository: Send + Sync {
    async fn find_by_user_and_type(
        &self,
        user_id: i64,
        credential_type: &CredentialType,
    ) -> Result<Option<UserCredential>, AppError>;

    async fn create(
        &self,
        user_id: i64,
        credential_type: CredentialType,
        secret: &str,
    ) -> Result<UserCredential, AppError>;

    async fn update_secret(
        &self,
        user_id: i64,
        credential_type: &CredentialType,
        new_secret: &str,
    ) -> Result<UserCredential, AppError>;
}