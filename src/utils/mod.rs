use oracle::pool::Pool;

use self::{sql::SQLManager, structs::APIError};

pub mod logging;
pub mod permissions;
pub mod sql;
pub mod structs;
pub mod testing;

pub async fn check_user_exists(
    username: String,
    pool: &Pool,
    sql_manager: &SQLManager,
) -> Result<bool, APIError> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error Connecting to DB");
        return Err(APIError::DBError);
    }
    let conn = conn.unwrap();
    println!("Username Check: {}", username);
    let stmt = conn
        .statement(sql_manager.get_sql("check_user")?.as_str())
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIError::DBError);
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&(username).to_lowercase()]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(APIError::DBError);
    }
    let rows = rows.unwrap();
    if rows.count() > 0 {
        return Ok(true);
    }
    Ok(false)
}
