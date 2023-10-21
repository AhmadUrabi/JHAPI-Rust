use oracle::pool::Pool;
use oracle::Result;

use crate::fetch_stores::structs::Store;

pub mod structs;

pub async fn fetch_store_list(pool: &Pool, user_id: String) -> Vec<Store> {
    match get_stores(pool, user_id) {
        Ok(stores) => stores,
        Err(err) => {
            println!("Error: {}", err.to_string());
            Vec::new()
        }
    }
}

fn get_stores(pool: &Pool, user_id: String) -> Result<Vec<Store>> {
    let conn = pool.get()?;
    let mut stmt = conn
        .statement("
        SELECT
        lpad(s.STORE_ID, 2, '0') STORE_ID, s.STORE_DESC, s.STORE_DESC_S
            FROM
                ODBC_JHC.JHC_STORES s
            WHERE
                EXISTS (
                    SELECT 1
                    FROM
                        ODBC_JHC.USER_STORES_JHC usa
                    JOIN
                        ODBC_JHC.AUTHENTICATION_JHC u
                    ON
                        usa.username = u.username
                    WHERE
                        (u.username = :user_id AND usa.all_stores_access = 1) -- Replace 'User2' with the desired username
                        OR (u.username = :user_id AND usa.store_id = s.store_id)
                )")
        .build()?;
    let rows = stmt.query(&[&user_id])?;
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
