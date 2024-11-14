use std::io::Cursor;
use calamine::{open_workbook_auto_from_rs, DataType, Reader};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use crate::scraper::Scraper;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use entity::sp_500_daily::SP500DailyModel;
use crate::client::YahooFinanceClient;

pub struct SP500Scraper;

const XLS_URL: &str = "https://img1.wsimg.com/blobby/go/e5e77e0b-59d1-44d9-ab25-4763ac982e53/downloads/dd6fb698-96e0-4cd2-ad91-61447f7087e2/ie_data.xls?ver=1731356767050";

impl Scraper for SP500Scraper {
    async fn run() -> Result<(), lib::error::Error> {
        SP500Scraper::fetch_daily().await;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MonthlySp500Data {
    date: DateTime<Utc>,
    close: Option<Decimal>,
    dividend: Option<Decimal>,
    earnings: Option<Decimal>,
    cpi: Option<Decimal>,
    gs10: Option<Decimal>,
    adjusted_close: Option<Decimal>,
    pe10: Option<Decimal>,
}
impl SP500Scraper {
    pub async fn fetch_monthly() {
        let response = reqwest::get(XLS_URL).await.unwrap();

        let bytes = response.bytes().await.unwrap();
        let data = Cursor::new(bytes);
        let mut xlsx = open_workbook_auto_from_rs(data).unwrap();
        let sheet_name = "Data";

        if let Ok(range) = xlsx.worksheet_range(sheet_name) {
            let range = range.range((8,0), (range.height() as u32, range.width() as u32));

            let rows = range.rows();
            let mut data = Vec::new();
            for row in rows {
                if row.get(0).unwrap().is_empty() {
                    continue
                }

                let year = row.get(0).unwrap().get_float().unwrap() as f32;
                let year_string = format!("{:.3}", year);
                let year = year_string.split_once(".").unwrap().0;
                let month = row.get(0).unwrap().get_float().unwrap();
                let month_string = format!("{:.2}", month);
                let month = month_string.split_once(".").unwrap().1;

                let date = NaiveDate::from_ymd_opt(year.parse::<i32>().unwrap(), month.parse::<u32>().unwrap(), 1).unwrap();
                
                let dividend = if let Some(dividend) = row.get(2).unwrap().get_float() {
                    let dividend = Decimal::from_f64(dividend).unwrap();
                    let dividend = dividend.checked_div(Decimal::TEN + Decimal::TWO).unwrap();
                    Some(dividend)
                } else { None };

                let gs10 = row.get(6).unwrap().get_float();
                let gs10 = if let Some(gs10) = gs10 {
                    if gs10 > 100f64 {
                        None
                    } else {
                        Some(Decimal::from_f64(gs10).unwrap())
                    }
                } else {
                    None
                };

                data.push(MonthlySp500Data {
                    date: Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()),
                    close: Decimal::from_f64(row.get(1).unwrap().get_float().unwrap_or(0f64)).map(|i| i.round_dp(2)),
                    dividend: dividend.map(|i| i.round_dp(2)),
                    earnings: row.get(3).unwrap().get_float().map(|fl| Decimal::from_f64(fl).unwrap().round_dp(2)),
                    cpi: Decimal::from_f64(row.get(4).unwrap().get_float().unwrap_or(0f64)).map(|i| i.round_dp(2)),
                    gs10: gs10.map(|i| i.round_dp(2)),
                    pe10: Decimal::from_f64(row.get(12).unwrap().get_float().unwrap_or(0f64)).map(|i| i.round_dp(2)),
                    adjusted_close: Decimal::from_f64(row.get(7).unwrap().get_float().unwrap_or(0f64)).map(|i| i.round_dp(2)),
                })
            }

            let mut reversed_data = data.into_iter().rev().collect::<Vec<_>>();
            let cloned_reversed_data = reversed_data.clone();

            // Iterate with index and data
            for (i, record) in reversed_data.iter_mut().enumerate() {
                if (i == 0) || cloned_reversed_data[i - 1].dividend.is_none() {
                    println!("Skipping record with index {} and dividend", i);
                    continue;
                }

                let prev = &cloned_reversed_data[i - 1];
                let new_adjusted_close = if let Some(adj_close) = prev.adjusted_close {
                    let current_close = record.close.unwrap();
                    let prev_close = prev.close.unwrap();
                    let prev_dividend = prev.dividend.unwrap();

                    let inner = current_close.checked_sub(prev_close).unwrap().checked_sub(prev_dividend).unwrap();
                    let inner= adj_close.checked_mul(inner).unwrap();
                    let inner = inner.checked_div(prev_close).unwrap();

                    Some(adj_close.checked_add(inner).unwrap().round_dp(2))
                } else {
                    println!("Skipping record with index {} and adjusted close {:?}", i, prev.adjusted_close);
                    None
                };

                record.adjusted_close = new_adjusted_close;
            }

            let truncate = r#"
                TRUNCATE TABLE sp_500_monthly;
            "#;

            let database = crate::database::Database::new().await;
            let pool = database.get_pool().await;
            sqlx::query(truncate).execute(pool).await.unwrap();

            let query = r#"
                INSERT INTO sp_500_monthly (date, close, dividend, earnings, cpi, gs10, adjusted_close, pe10)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#;

            for record in reversed_data {
                sqlx::query(query)
                    .bind(record.date)
                    .bind(record.close)
                    .bind(record.dividend)
                    .bind(record.earnings)
                    .bind(record.cpi)
                    .bind(record.gs10)
                    .bind(record.adjusted_close)
                    .bind(record.pe10)
                    .execute(pool)
                    .await
                    .unwrap();
            }

            println!("Finished inserting data");
        }
    }

    pub async fn fetch_daily() {
        let client = YahooFinanceClient::new();
        let ticker = "^SP500TR";

        let start = Utc::now() - chrono::Duration::days(365);
        let end = Utc::now();

        let start_date = client.convert_to_offset_date_time(start);
        let end_date = client.convert_to_offset_date_time(end);
        let history = client.get_quote_history_1d(ticker, start_date, end_date).await.unwrap();

        let mut daily = Vec::new();
        let quotes = history.quotes().unwrap();
        // Remove duplicates
        let mut seen = std::collections::HashSet::new();

        for quote in quotes {
            if seen.contains(&quote.timestamp) {
                continue;
            }
            seen.insert(quote.timestamp);

            daily.push(SP500DailyModel {
                date: DateTime::from_timestamp(quote.timestamp as i64, 0).unwrap(),
                value: quote.close
            });
        }

        let truncate = r#"
            TRUNCATE TABLE sp_500_daily;
        "#;

        let database = crate::database::Database::new().await;
        let pool = database.get_pool().await;
        sqlx::query(truncate).execute(pool).await.unwrap();

        let query = r#"
            INSERT INTO sp_500_daily (date, value)
            VALUES ($1, $2)
        "#;

        for record in daily {
            sqlx::query(query)
                .bind(record.date)
                .bind(record.value)
                .execute(pool)
                .await
                .unwrap();
        }
    }
}