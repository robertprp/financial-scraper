use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Decimal;
#[derive(Clone, Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct StockAssetAllocationModel {
    pub date: DateTime<Utc>,
    pub percentage: Decimal
}