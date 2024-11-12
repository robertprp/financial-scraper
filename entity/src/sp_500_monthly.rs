use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Decimal;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SP500MonthlyModel {
    pub date: DateTime<Utc>,
    pub close: Decimal,
    pub dividend: Decimal,
    pub earnings: Decimal,
    pub cpi: Decimal,
    pub gs10: Decimal,
    pub pe10: Decimal,
    pub adjusted_close: Decimal,
}