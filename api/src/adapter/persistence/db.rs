use sqlx::PgPool;

use crate::error::AppError;

/// 環境変数 `DATABASE_URL` からPgPoolを生成する。
pub async fn connect() -> Result<PgPool, AppError> {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPool::connect(&url)
        .await
        .map_err(AppError::Database)
}