use std::ops::{Div, Sub};
use std::ptr::addr_eq;
use async_graphql::{Context, MergedObject, MergedSubscription, Object, SimpleObject, Subscription};
use sqlx::types::chrono::{DateTime, Utc};
use rust_decimal::{Decimal, MathematicalOps};
use linreg::{linear_regression, linear_regression_of};
use rust_decimal::prelude::FromPrimitive;
use service::database::Database;
use service::database::queries::Queries;
use crate::format::convert_decimal_to_f32;
use async_graphql::{OutputType};
#[derive(Default)]
pub struct StockToAssetAllocationQuery;


#[derive(SimpleObject)]
pub struct AllocationData {
    date: DateTime<Utc>,
    percentage: Option<Decimal>,
    return_10:  Option<Decimal>,
}

#[derive(SimpleObject)]
pub struct StockToAssetAllocation {
    correlation_squared: Option<Decimal>,
    expected_returns: Option<Decimal>,
    extrapolated_returns: Option<Decimal>,
    last_updated_date: DateTime<Utc>,
    last_extrapolated_date: DateTime<Utc>,
    data: Vec<AllocationData>,
}

#[Object]
impl StockToAssetAllocationQuery {
    async fn query_allocation_and_spx_returns(&self, ctx: &Context<'_>,) ->  async_graphql::Result<StockToAssetAllocation> {
        let db = Database::new().await;
        let pool = db.get_pool().await;
        let annualized_and_asset_allocation_returns = Queries::fetch_annualized_and_asset_allocation_returns(&pool).await.unwrap();
        let data: Vec<AllocationData> = annualized_and_asset_allocation_returns.into_iter().map(|(date, percentage, return_10)| {
            AllocationData {
                // Should we add 3 months?
                date: date,
                percentage,
                return_10,
            }
        }).collect();

        let regression_data: Vec<(f32, f32)> = data
            .iter()
            .filter_map(|d| {
                d.return_10.map(|return_10| (d.percentage.unwrap(), return_10))
            })
            .map(|(percentage, return_10)| {
                (convert_decimal_to_f32(percentage), convert_decimal_to_f32(return_10))
            })
            .collect();


        let results: (f32, f32) = linear_regression_of(&regression_data).unwrap();

        let (slope, intercept) = results;
        let slope = Decimal::from_f32(slope).unwrap();
        let intercept = Decimal::from_f32(intercept).unwrap();
        let latest_allocation = data.last().unwrap().percentage.unwrap();
        let expected_returns = slope.checked_mul(latest_allocation).unwrap().checked_add(intercept).unwrap();
        let last_updated_date = data.last().unwrap().date;

        let corr = Queries::fetch_stock_allocation_vs_return_correlation(&pool).await.unwrap();

        let last_sp500_daily = Queries::get_sp500_daily_last_date(&pool).await.unwrap();

        let latest_allocation = Queries::get_latest_allocation_with_raw_date(&pool, last_sp500_daily.date).await.unwrap();

        let latest_sp500_value = last_sp500_daily.value;
        let latest_allocation_500_value = latest_allocation.value;

        let additional_returns =  (latest_allocation_500_value.div(latest_sp500_value).powf(0.1f64)).sub(Decimal::ONE);
        let extrapolated_returns = expected_returns.checked_add(additional_returns).unwrap();
        let last_extrapolated_date = last_sp500_daily.date;
        let correlation_squared = corr.checked_mul(corr);

        let allocation = StockToAssetAllocation {
            correlation_squared,
            expected_returns: Some(expected_returns),
            extrapolated_returns: Some(extrapolated_returns),
            last_updated_date,
            last_extrapolated_date,
            data,
        };

        Ok(allocation)
    }
}

#[derive(MergedObject, Default)]
pub struct Query(
    StockToAssetAllocationQuery
);
