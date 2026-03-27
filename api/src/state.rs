use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    db: PgPool,
    jwt_secret: String,
}

impl AppState {
    #[must_use]
    pub fn new(db: PgPool, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    #[must_use]
    pub fn db(&self) -> &PgPool {
        &self.db
    }

    #[must_use]
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    #[must_use]
    pub fn jwt_secret_bytes(&self) -> &[u8] {
        self.jwt_secret.as_bytes()
    }
}
