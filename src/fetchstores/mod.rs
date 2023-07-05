use oracle::{Result};
use oracle::pool::Pool;
use rocket::serde::json::Json;

use crate::apistructs::Store;

pub async fn fetch_store_list(pool: &Pool) -> Option<Json<Vec<Store>>> {
    println!("Fetching Store List");
    let stores = get_stores(pool).unwrap();
    if stores.is_empty() {
        return None;
    } else {
        Some(Json(stores.into_iter().map(|store| Store {
            STORE_ID: store.STORE_ID,
            STORE_DESC: store.STORE_DESC,
            STORE_DESC_S: store.STORE_DESC_S,
        }).collect()
        ))
    }
}

fn get_stores(pool: &Pool) -> Result<Vec<Store>> {
    let mut conn = pool.get()?;
    let mut stmt = conn.statement("SELECT STORE_ID, STORE_DESC, STORE_DESC_S FROM ODBC_JHC.JHC_STORES").build()?;
    let rows = stmt.query(&[])?;
    let mut stores: Vec<Store> = Vec::new();
    for row_result in rows {
        let row = row_result?;

        let store = Store {
            STORE_ID: row.get(0)?,
            STORE_DESC: row.get(1)?,
            STORE_DESC_S: row.get(2)?,
        };

        stores.push(store);
    }
    Ok(stores)
}