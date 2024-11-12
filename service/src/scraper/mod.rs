mod sp500;
use lib;
pub trait Scraper {
    async fn run() -> Result<(), lib::error::Error>;
}