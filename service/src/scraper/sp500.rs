use chrono::{DateTime, Datelike, Utc};
use crate::scraper::Scraper;
use csv::ReaderBuilder;
use entity::sp_500_daily::SP500DailyModel;

pub struct SP500Scraper;

const XLS_URL: &str = "http://www.econ.yale.edu/~shiller/data/ie_data.xls";
const DAILY_CSV_URL: &str = format!("https://query1.finance.yahoo.com/v7/finance/download/%5ESP500TR?period1=${lastYear}&period2=2500000000&interval=1d&events=history&crumb=",
    lastYear = (Utc::now() - chrono::Duration::days(365)).timestamp()
).as_str();

impl Scraper for SP500Scraper {
    async fn run() {
        let xls = reqwest::get(XLS_URL).await.unwrap().bytes().await.unwrap();
        let daily_csv = reqwest::get(DAILY_CSV_URL).await.unwrap().text().await.unwrap();
        // Parse xls and daily_csv
    }
}

impl SP500Scraper {
    pub async fn fetch_daily() {
        let daily_csv = reqwest::get(DAILY_CSV_URL).await.unwrap().text().await.unwrap();
        // Parse daily_csv
        let mut rdr = ReaderBuilder::new().from_reader(daily_csv.as_bytes());

        // preprocess data
        let mut records: Vec<SP500DailyModel> = Vec::new();
        for result in rdr.records().rev() {
            let record = result.unwrap();
            let date = record[0].to_owned();
            let close = record[4].to_owned();

            let date = DateTime::parsefroms
            records.push(SP500DailyModel { date: , close });

            // println!("Name: {}, Age: {}", record[0], record[1].parse::<u8>().unwrap());
        }
    }
}