use rust_decimal::prelude::ToPrimitive;
use sqlx::types::Decimal;

pub fn convert_decimal_to_f32(d: Decimal) -> f32 {
    d.to_f32().unwrap()
}