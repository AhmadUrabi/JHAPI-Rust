use oracle::pool::Pool;

use crate::fetch_stores::structs::Store;

pub mod structs;

pub fn get_stores(pool: &Pool, user_id: String) -> Result<Vec<Store>, String> {
    let conn = pool.get();
    if conn.is_err() {
        return Err(format!("Error getting connection: {}", conn.err().unwrap()));
    }
    let conn = conn.unwrap();
    let stmt = conn
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
                        (u.username = :user_id AND usa.all_stores_access = 1)
                        OR (u.username = :user_id AND usa.store_id = s.store_id)
                )")
        .build();

    if stmt.is_err() {
        return Err(format!("Error preparing statement: {}", stmt.err().unwrap()));
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&user_id]);

    if rows.is_err() {
        return Err(format!("Error executing query: {}", rows.err().unwrap()));
    }
    let rows = rows.unwrap();

    let mut stores: Vec<Store> = Vec::new();
    for row_result in rows {
        let row = row_result;
        if row.is_err() {
            return Err(format!("Error getting row: {}", row.err().unwrap()));
        }
        let row = row.unwrap();

        // Multiple use of unwraps here, but we know the types are correct
        // Might have to change the handling of this later
        // TODO: Review error handling on value unwraps
        let store = Store {
            STORE_ID: row.get(0).unwrap(),
            STORE_DESC: row.get(1).unwrap(),
            STORE_DESC_S: row.get(2).unwrap()
        };

        stores.push(store);
    }
    info!("Stores Count: {}", stores.len());
    Ok(stores)
}
