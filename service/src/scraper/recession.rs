use std::io::Cursor;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use entity::recessions::RecessionModel;
use crate::database::Database;

const CSV_URL: &str = "https://fred.stlouisfed.org/graph/fredgraph.csv?chart_type=line&recession_bars=off&log_scales=&bgcolor=%23e1e9f0&graph_bgcolor=%23ffffff&fo=Open+Sans&ts=12&tts=12&txtcolor=%23444444&show_legend=yes&show_axis_titles=yes&drp=0&cosd=1854-12-01&coed=2014-08-01&height=450&stacking=&range=&mode=fred&id=USREC&transformation=lin&nd=&ost=-99999&oet=99999&lsv=&lev=&mma=0&fml=a&fgst=lin&fgsnd=2009-06-01&fq=Monthly&fam=avg&vintage_date=&revision_date=&line_color=%234572a7&line_style=solid&lw=2&scale=left&mark_type=none&mw=2&width=1168";

pub struct RecessionScraper;

impl RecessionScraper {
    pub async fn fetch() {
        let bytes = reqwest::get(CSV_URL)
            .await
            .unwrap().bytes().await.unwrap();

        let cursor = Cursor::new(bytes);
        let csv = csv::Reader::from_reader(cursor);

        let mut is_recession = false;
        let mut start_date: Option<DateTime<Utc>> = None;
        let mut end_date: Option<DateTime<Utc>> = None;

        for result in csv.into_records() {
            let record = result.unwrap();
            let date = &record[0];
            let value = &record[1];
            println!("Date: {}, Value: {}", date, value);

            let is_event_recession = value == "1";

            if is_event_recession && !is_recession {
                let start = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
                is_recession = true;
                start_date = Some(Utc.from_utc_datetime(&start.and_hms_opt(0, 0, 0).unwrap()));
            } else if !is_event_recession && is_recession {
                let end = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
                end_date = Some(Utc.from_utc_datetime(&end.and_hms_opt(0, 0, 0).unwrap()));
                Self::save_recession(RecessionModel {
                    start_date: start_date.unwrap(),
                    end_date: end_date.unwrap(),
                }).await;

                is_recession = false;
                start_date = None;
                end_date = None;
            } else {
                continue;
            }
        }
    }

    pub async fn save_recession(recession: RecessionModel) {
        // Save recession to database
        let db = Database::new().await;
        let pool = db.get_pool().await;
        let query = r#"
            INSERT INTO recessions (start_date, end_date)
            VALUES ($1, $2)
        "#;

        sqlx::query(query)
            .bind(recession.start_date)
            .bind(recession.end_date)
            .execute(pool)
            .await
            .unwrap();
    }
}