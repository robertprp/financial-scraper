use chrono::{DateTime, Duration, Utc};
use reqwest::header::HeaderMap;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::time::OffsetDateTime;
use yahoo_finance_api::{YResponse, YSearchResult, YSearchResultOpt, YahooError};
use yahoo_finance_api::time::format_description::modifier::UnixTimestamp;
use crate::scraper::sp500::SP500Scraper;

pub struct YahooFinanceClient(pub yahoo::YahooConnector);

impl YahooFinanceClient {
    pub fn new() -> Self {
        let provider = yahoo::YahooConnector::new().unwrap();

        Self(provider)
    }

    pub async fn search_ticker_opt(&self, ticker: &str) -> Result<YSearchResult, YahooError> {
        self.0.search_ticker(ticker).await
    }

    pub async fn get_quote_history_1d(
        &self,
        ticker: &str,
        start_date: OffsetDateTime,
        end_date: OffsetDateTime,
    ) -> Result<YResponse, YahooError> {
        self.0.get_quote_history_interval(ticker, start_date, end_date, "1d").await
    }

    pub fn convert_to_offset_date_time(&self, date: DateTime<Utc>) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(date.timestamp()).unwrap()
    }
}

pub struct TestYahooFinanceApi;

impl TestYahooFinanceApi {
    pub async fn test() {
        //SP500Scraper::fetch_daily().await;
        
    }
}