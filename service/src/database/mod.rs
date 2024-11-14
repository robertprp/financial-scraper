pub mod queries;

pub struct Database {
    db: sqlx::PgPool
}

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/financial-scraper";

impl Database {
    pub async fn new() -> Self {
        let db = sqlx::PgPool::connect(DATABASE_URL)
            .await
            .expect("Failed to connect to database");

        Database { db }
    }

    pub async fn get_pool(&self) -> &sqlx::PgPool {
        &self.db
    }
}