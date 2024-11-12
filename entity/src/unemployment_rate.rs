use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Decimal;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UnemploymentRateModel {
    pub date: DateTime<Utc>,
    pub percentage: Decimal,
}