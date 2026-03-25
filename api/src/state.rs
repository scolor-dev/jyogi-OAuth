use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    #[must_use]
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    #[must_use]
    pub fn db(&self) -> &PgPool {
        &self.db
    }
}
