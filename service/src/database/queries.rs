use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use entity::sp_500_daily::SP500DailyModel;

pub struct Queries;

impl Queries {
    pub async fn fetch_annualized_and_asset_allocation_returns(
        pool: &PgPool,
    ) -> Result<Vec<(DateTime<Utc>, Option<Decimal>, Option<Decimal>)>, sqlx::Error> {
        let query = r#"
        SELECT b.date, b.percentage::DECIMAL, a.return_10::DECIMAL
        FROM analysis.sp_500_annualized_return a, stock_asset_allocation b
        WHERE (date_trunc('month', a.date) = date_trunc('month', b.date + INTERVAL '2 month'))
        AND b.percentage IS NOT NULL
    "#;

        let results = sqlx::query_as::<_, (DateTime<Utc>, Option<Decimal>, Option<Decimal>)>(query)
            .fetch_all(pool)
            .await;

        results
    }

    pub async fn fetch_stock_allocation_vs_return_correlation(
        pool: &PgPool,
    ) -> Result<Decimal, sqlx::Error> {
        let query = r#"
        SELECT return_10::DECIMAL
        FROM analysis.usa_stock_allocation_vs_return_corr
    "#;

        let correlation = sqlx::query_scalar::<_, Decimal>(query)
            .fetch_one(pool)
            .await;

        correlation
    }

    pub async fn get_sp500_daily_last_date(
        pool: &PgPool,
    ) -> Result<SP500DailyModel, sqlx::Error> {
        let query = r#"
        SELECT *
        FROM sp_500_daily
        ORDER BY date DESC
        LIMIT 1
    "#;

        let res = sqlx::query_as(query)
            .fetch_one(pool)
            .await;

        res
        }

        pub async fn get_latest_allocation_with_raw_date(
            pool: &PgPool,
            raw_date: DateTime<Utc>,
        ) -> Result<SP500DailyModel, sqlx::Error> {
            let query = r#"
            SELECT *
            FROM sp_500_daily
            WHERE date = $1
            ORDER BY date DESC
            LIMIT 1
        "#;

                let res = sqlx::query_as(query)
                    .bind(raw_date)
                    .fetch_one(pool)
                    .await;

            res
            }
}