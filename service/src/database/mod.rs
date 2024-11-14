pub mod queries;

pub struct Database {
    db: sqlx::PgPool
}

impl Database {
    pub async fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL");
        let db = sqlx::PgPool::connect(&db_url.unwrap())
            .await
            .expect("Failed to connect to database");

        Database { db }
    }

    pub async fn get_pool(&self) -> &sqlx::PgPool {
        &self.db
    }
}