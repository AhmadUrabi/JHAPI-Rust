#![allow(non_snake_case)]
use oracle::Error;
use oracle::pool::Pool;


use rocket::State;
use rocket::serde::json::Json;

pub mod structs;

use structs::LogData;


pub fn get_user_logs_fn(username: String, pool: &State<Pool>) -> Result<Json<Vec<LogData>>,Error>{
    let conn;
    match pool.get() {
        Ok(connection) => conn = connection,
        Err(err) => {
            error!("Error: {}", err);
            return Err(err);
        }
    }
    let mut logs: Vec<LogData> = Vec::new();
    let mut stmt = conn.statement("SELECT * FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username ORDER BY TIMESTAMP DESC FETCH NEXT 100").build()?;
    let rows = stmt.query(&[&username])?;

    for row_res in rows {
        let row = row_res?;
        logs.push(LogData {
            id: row.get("LOG_ID")?,
            username: row.get("USERNAME")?,
            route: row.get("ROUTE")?,
            parameters: row.get("PARAMETERS")?,
            timestamp: row.get("TIMESTAMP")?,
            result: row.get("RESULT")?,
            token_used: row.get("TOKEN_USED")?,
            ip_address: row.get("IP_ADDRESS")?,
        });
    }
    Ok(Json(logs))
}