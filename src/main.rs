use std::path::Path;
use tokio;
use sqlx::migrate::Migrator;
use env_logger;
use dotenv::dotenv;
use service::scraper::recession::RecessionScraper;
use service::scraper::sp500::SP500Scraper;
use service::scraper::stock_asset_allocation::StockAssetAllocation;

#[tokio::main]
async fn main() {
    // init log
    dotenv().ok();
    env_logger::init();
    // Run migrations
    let migrator = Migrator::new(Path::new("migrations")).await.unwrap();
    let db = service::database::Database::new().await;
    let pool = db.get_pool().await;
    migrator.run(pool).await.unwrap();

    SP500Scraper::fetch_monthly().await;
    SP500Scraper::fetch_daily().await;
    RecessionScraper::fetch().await;
    StockAssetAllocation::fetch_data().await;
    run_server().await;
}

async fn run_server() {
    let server = graphql::server::Server::new();
    server.start().await;
}