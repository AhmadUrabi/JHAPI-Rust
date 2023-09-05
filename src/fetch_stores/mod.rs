use oracle::Result;
use oracle::pool::Pool;

use crate::fetch_stores::structs::Store;

pub mod structs;

pub async fn fetch_store_list(pool: &Pool) -> Vec<Store> {
    match get_stores(pool) {
        Ok(stores) => stores,
        Err(err) => {
            println!("Error: {}", err.to_string());
            Vec::new()
        }
    }  
}

fn get_stores(pool: &Pool) -> Result<Vec<Store>> {
    let conn = pool.get()?;
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
    info!("Stores Count: {}", stores.len());
    Ok(stores)
}