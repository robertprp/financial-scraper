use std::path::Path;
use tokio;
use service::client::TestYahooFinanceApi;
use sqlx::migrate::Migrator;
use log;
use service::scraper::recession::RecessionScraper;
use service::scraper::sp500::SP500Scraper;
use service::scraper::stock_asset_allocation::StockAssetAllocation;
use env_logger;
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    // init log
    dotenv().ok();
    env_logger::init();
    // Run migrations
    let m = Migrator::new(Path::new("migrations")).await.unwrap();
    let db = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/financial-scraper").await.unwrap();
    m.run(&db).await.unwrap();

    // SP500Scraper::fetch_monthly().await;
    // SP500Scraper::fetch_daily().await;
    // RecessionScraper::fetch().await;
    // StockAssetAllocation::fetch_data().await;
    run_server().await;
}

async fn run_server() {
    let server = graphql::server::Server::new();
    server.start().await;
}