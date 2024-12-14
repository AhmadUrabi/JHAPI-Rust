use oracle::pool::Pool;

use crate::utils::check_user_exists;

use crate::utils::sql::SQLManager;
use crate::{controllers::stores::structs::Store, utils::structs::APIError};

pub mod structs;

pub async fn get_stores(
    pool: &Pool,
    sql_manager: &SQLManager,
    user_id: String,
) -> Result<Vec<Store>, APIError> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIError::DBError);
    }
    let conn = conn.unwrap();

    match check_user_exists(user_id.clone(), pool, &sql_manager).await {
        Ok(b) => {
            if !b {
                error!("User does not exist");
                return Err(APIError::DataNotFound);
            }
        }
        Err(_err) => {
            error!("Error checking for duplicate user");
            return Err(APIError::DBError);
        }
    }

    let stmt = conn
        .statement(sql_manager.get_sql("get_user_stores")?.as_str())
        .build();

    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIError::DBError);
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&user_id]);

    if rows.is_err() {
        error!("Error executing query");
        return Err(APIError::DBError);
    }
    let rows = rows.unwrap();

    let mut stores: Vec<Store> = Vec::new();
    for row_result in rows {
        let row = row_result;
        if row.is_err() {
            error!("Error fetching row");
            return Err(APIError::DBError);
        }
        let row = row.unwrap();

        // Multiple use of unwraps here, but we know the types are correct

        let store = Store {
            STORE_ID: row.get(0).unwrap(),
            STORE_DESC: row.get(1).unwrap(),
            STORE_DESC_S: row.get(2).unwrap(),
        };

        stores.push(store);
    }
    // info!("Stores Count: {}", stores.len());
    Ok(stores)
}
