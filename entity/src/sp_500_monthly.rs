use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Decimal;
#[derive(Clone, Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct SP500MonthlyModel {
    pub date: DateTime<Utc>,
    pub close: Option<Decimal>,
    pub dividend:  Option<Decimal>,
    pub earnings:  Option<Decimal>,
    pub cpi:  Option<Decimal>,
    pub gs10:  Option<Decimal>,
    pub pe10:  Option<Decimal>,
    pub adjusted_close:  Option<Decimal>,
}