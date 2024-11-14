use std::io::{ Cursor};
use std::str::Bytes;
use calamine::{DataType, Reader};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use entity::stock_asset_allocation::StockAssetAllocationModel;
use lib::error::Error;
use crate::database::Database;

pub struct StockAssetAllocation;

const XLSX_URL: &str = "https://fred.stlouisfed.org/graph/fredgraph.xls?chart_type=line&recession_bars=on&log_scales=&bgcolor=%23b3cde7&graph_bgcolor=%23ffffff&fo=Open+Sans&ts=8&tts=8&txtcolor=%23000000&show_legend=yes&show_axis_titles=yes&drp=0&cosd=1945-10-01&coed=2099-01-01&height=378&stacking=&range=&mode=fred&id=NCBEILQ027S_BCNSDODNS_CMDEBT_FGSDODNS_SLGSDODNS_FBCELLQ027S_DODFFSWCMI&transformation=lin_lin_lin_lin_lin_lin_lin&nd=______&ost=-99999_-99999_-99999_-99999_-99999_-99999_-99999&oet=99999_99999_99999_99999_99999_99999_99999&lsv=&lev=&scale=left&line_color=%230000ff&line_style=solid&lw=3&mark_type=none&mw=4&mma=0&fml=((a%2Bf)%2F1000)%2F(((a%2Bf)%2F1000)%2Bb%2Bc%2Bd%2Be%2Bg)&fgst=lin&fgsnd=2007-12-01&fq=Quarterly&fam=avg&vintage_date=&revision_date=&width=630";
impl StockAssetAllocation {
    pub async fn fetch_data() {
        Self::truncate_allocation_data().await;
        let response = reqwest::get(XLSX_URL).await.unwrap();

        let bytes = response.bytes().await.unwrap();

        let cursor = Cursor::new(bytes);

        let mut xls = calamine::open_workbook_auto_from_rs(cursor).unwrap();

        let sheet_name = "FRED Graph";

        let range = xls.worksheet_range(sheet_name).unwrap();
        let range = range.range((11, 0), (range.height() as u32, range.width() as u32));

        let rows = range.rows();
        for row in rows {
            let first_cell = row.get(0);
            let second_cell = row.get(1);

            if first_cell.is_none() || second_cell.is_none() || first_cell.unwrap().is_empty() || second_cell.unwrap().is_empty() {
                continue;
            }
            
            let date = row.get(0).unwrap().get_datetime().unwrap().as_datetime().unwrap().and_utc();
            let stock_asset_allocation = row.get(1).unwrap().get_float().unwrap();

            let alloc = StockAssetAllocationModel {
                date,
                percentage: Decimal::from_f64(stock_asset_allocation).unwrap_or(Decimal::ZERO).round_dp(6)
            };

            Self::insert_allocation(alloc).await;
        }
        
        println!("Stock asset allocation data has been fetched and saved to database");
    }
    
    pub async fn truncate_allocation_data() {
        // Truncate stock asset allocation data
        let db = Database::new().await;
        let pool = db.get_pool().await;
        let query = r#"
            TRUNCATE TABLE stock_asset_allocation
        "#;
        
        sqlx::query(query)
            .execute(pool)
            .await
            .unwrap();
    }
    pub async fn insert_allocation(allocation: StockAssetAllocationModel) {
        // Save stock asset allocation to database
        let db = Database::new().await;
        let pool = db.get_pool().await;
        let query = r#"
            INSERT INTO stock_asset_allocation (date, percentage)
            VALUES ($1, $2)
        "#;
        
        sqlx::query(query)
            .bind(allocation.date)
            .bind(allocation.percentage)
            .execute(pool)
            .await
            .unwrap();
    }
}