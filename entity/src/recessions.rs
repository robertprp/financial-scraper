use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct RecessionModel {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>
}