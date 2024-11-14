pub mod queries;

pub struct Database {
    db: sqlx::PgPool
}

impl Database {
    pub async fn new() -> Self {
        let db = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/financial-scraper")
            .await
            .expect("Failed to connect to database");

        Database { db }
    }

    pub async fn get_pool(&self) -> &sqlx::PgPool {
        &self.db
    }
}