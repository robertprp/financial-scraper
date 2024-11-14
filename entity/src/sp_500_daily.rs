use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Decimal;
#[derive(Clone, Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct SP500DailyModel {
    pub date: DateTime<Utc>,
    pub value: Decimal
}