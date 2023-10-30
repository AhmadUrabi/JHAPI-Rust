#![allow(non_snake_case)]
use oracle::Error;
use oracle::pool::Pool;


use rocket::State;
use rocket::serde::json::Json;

pub mod structs;

use structs::LogData;


pub fn get_all_logs_fn(pool: &State<Pool>, limit: Option<i32>) -> Result<Json<Vec<LogData>>,Error>{
    let conn;
    match pool.get() {
        Ok(connection) => conn = connection,
        Err(err) => {
            error!("Error: {}", err);
            return Err(err);
        }
    }

    let logLimit: i32;
    match limit {
        Some(limit) => logLimit = limit,
        None => logLimit = 100,
    }

    let mut logs: Vec<LogData> = Vec::new();
    let mut stmt = conn.statement("SELECT * FROM ODBC_JHC.API_LOGS ORDER BY TIMESTAMP DESC FETCH NEXT :limit ROWS ONLY").build()?;
    let rows = stmt.query(&[&logLimit])?;

    for row_res in rows {
        let row = row_res?;
        logs.push(LogData {
            LOG_ID: row.get("LOG_ID")?,
            USERNAME: row.get("USERNAME")?,
            ROUTE: row.get("ROUTE")?,
            PARAMETERS: row.get("PARAMETERS")?,
            RESULT: row.get("RESULT")?,
            TIMESTAMP: row.get("TIMESTAMP")?,
            TOKEN_USED: row.get("TOKEN_USED")?,
            IP_ADDRESS: row.get("IP_ADDRESS")?,
            METHOD: row.get("METHOD")?,
        });
    }
    Ok(Json(logs))
}

pub fn get_user_logs_fn(username: String, pool: &State<Pool>, limit: Option<i32>) -> Result<Json<Vec<LogData>>,Error>{
    let conn;
    match pool.get() {
        Ok(connection) => conn = connection,
        Err(err) => {
            error!("Error: {}", err);
            return Err(err);
        }
    }

    let logLimit: i32;
    match limit {
        Some(limit) => logLimit = limit,
        None => logLimit = 100,
    }

    let mut logs: Vec<LogData> = Vec::new();
    let mut stmt = conn.statement("SELECT * FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username ORDER BY TIMESTAMP DESC FETCH NEXT :limit ROWS ONLY").build()?;
    let rows = stmt.query(&[&username, &logLimit])?;

    for row_res in rows {
        let row = row_res?;
        logs.push(LogData {
            LOG_ID: row.get("LOG_ID")?,
            USERNAME: row.get("USERNAME")?,
            ROUTE: row.get("ROUTE")?,
            PARAMETERS: row.get("PARAMETERS")?,
            RESULT: row.get("RESULT")?,
            TIMESTAMP: row.get("TIMESTAMP")?,
            TOKEN_USED: row.get("TOKEN_USED")?,
            IP_ADDRESS: row.get("IP_ADDRESS")?,
            METHOD: row.get("METHOD")?,
        });
    }
    Ok(Json(logs))
}

pub fn get_route_logs_fn(route: String, pool: &State<Pool>, limit: Option<i32>) -> Result<Json<Vec<LogData>>,Error>{
    let conn;
    match pool.get() {
        Ok(connection) => conn = connection,
        Err(err) => {
            error!("Error: {}", err);
            return Err(err);
        }
    }

    let logLimit: i32;
    match limit {
        Some(limit) => logLimit = limit,
        None => logLimit = 100,
    }

    let mut logs: Vec<LogData> = Vec::new();
    let mut stmt = conn.statement("SELECT * FROM ODBC_JHC.API_LOGS WHERE ROUTE = :route ORDER BY TIMESTAMP DESC FETCH NEXT :limit ROWS ONLY").build()?;
    let rows = stmt.query(&[&route, &logLimit])?;

    for row_res in rows {
        let row = row_res?;
        logs.push(LogData {
            LOG_ID: row.get("LOG_ID")?,
            USERNAME: row.get("USERNAME")?,
            ROUTE: row.get("ROUTE")?,
            PARAMETERS: row.get("PARAMETERS")?,
            RESULT: row.get("RESULT")?,
            TIMESTAMP: row.get("TIMESTAMP")?,
            TOKEN_USED: row.get("TOKEN_USED")?,
            IP_ADDRESS: row.get("IP_ADDRESS")?,
            METHOD: row.get("METHOD")?,
        });
    }
    Ok(Json(logs))
}

pub fn delete_user_logs_fn(username: String, pool: &State<Pool>, limit: Option<i32>) -> Result<(),Error>{
    let conn;
    match pool.get() {
        Ok(connection) => conn = connection,
        Err(err) => {
            error!("Error: {}", err);
            return Err(err);
        }
    }
    let mut stmt;
    match limit {
        Some(lim)=> {
            stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE ROWID IN (SELECT ROWID FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username ORDER BY TIMESTAMP DESC FETCH FIRST :limit ROWS ONLY)").build()?;
            stmt.execute(&[&username, &lim])?;},
        None => {stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username").build()?;
            stmt.execute(&[&username])?;},
    }
    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error: {}", err);
            return Err(err);
        }
    }
    
    Ok(())
}

pub fn delete_log_logs_fn(log_id: i32, pool: &State<Pool>) -> Result<(),String>{
    let conn;
    match pool.get() {
        Ok(connection) => conn = connection,
        Err(err) => {
            error!("Error: {}", err);
            return Err("Error".to_string())
        }
    }

    let mut stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE LOG_ID = :log_id").build().unwrap();
    stmt.execute(&[&log_id]).unwrap();
    
    match conn.commit() {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error: {}", err);
            Err("Error".to_string())
        }
    }

}
