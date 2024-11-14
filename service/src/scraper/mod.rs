pub mod sp500;
pub mod recession;
pub mod stock_asset_allocation;

use lib;
pub trait Scraper {
    async fn run() -> Result<(), lib::error::Error>;
}