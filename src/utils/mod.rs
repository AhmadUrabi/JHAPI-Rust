use oracle::pool::Pool;

use self::structs::APIErrors;

use dotenv::dotenv;

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

pub fn get_valid_user_cred() -> (String, String) {
    dotenv().ok();
    let username = std::env::var("VALID_USER_TEST").expect("VALID_USER_TEST must be set.");
    let password = std::env::var("VALID_PASS_TEST").expect("VALID_PASS_TEST must be set.");
    (username, password)
}
pub fn get_invalid_user_cred() -> (String, String) {
    dotenv().ok();
    let username = std::env::var("INVALID_USER_TEST").expect("INVALID_USER_TEST must be set.");
    let password = std::env::var("INVALID_PASS_TEST").expect("INVALID_PASS_TEST must be set.");
    (username, password)
}