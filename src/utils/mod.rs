use oracle::pool::Pool;

use self::structs::APIErrors;

pub mod logging;
pub mod permissions;
pub mod structs;


pub fn check_user_exists(username: String, pool: &Pool) -> Result<bool, APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error Connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();
    println!("Username Check: {}", username);
    let stmt = conn
        .statement("SELECT USERNAME FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&(username).to_lowercase()]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(APIErrors::DBError);
    }
    let rows = rows.unwrap();
    if rows.count() > 0 {
        return Ok(true);
    }
    Ok(false)
}