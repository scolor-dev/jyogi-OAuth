use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::refresh_token::RefreshToken;
use crate::error::AppError;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<RefreshToken>, AppError>;

    async fn create(
        &self,
        session_id: i64,
        user_id: i64,
        user_uuid: Uuid,
        session_uuid: Uuid,
        token_hash: &str,
    ) -> Result<RefreshToken, AppError>;

    /// トークンを使用済みにマークする（Rotation時）
    async fn mark_used(&self, token_hash: &str) -> Result<(), AppError>;

    /// セッション単位で全トークンを無効化（ログアウト時）
    async fn revoke_all_by_session(&self, session_id: i64) -> Result<(), AppError>;
}