#![allow(non_snake_case)]
use oracle::pool::Pool;

use rocket::State;
use rocket::serde::json::Json;

pub mod structs;

use structs::LogData;

use crate::utils::structs::APIErrors;


pub fn get_all_logs_fn(pool: &State<Pool>, limit: Option<i32>) -> Result<Json<Vec<LogData>>,APIErrors>{
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let logLimit: i32;
    match limit {
        Some(limit) => logLimit = limit,
        None => logLimit = 100,
    }

    let mut logs: Vec<LogData> = Vec::new();
    let stmt = conn.statement("SELECT * FROM ODBC_JHC.API_LOGS ORDER BY TIMESTAMP DESC FETCH NEXT :limit ROWS ONLY").build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();


    let rows = stmt.query(&[&logLimit]);
    if rows.is_err() {
        error!("Error executing statement");
        return Err(APIErrors::DBError);
    }
    let rows = rows.unwrap();

    for row_res in rows {
        let row = row_res;
        if row.is_err() {
            error!("Error getting row");
            return Err(APIErrors::DBError);
        }
        let row = row.unwrap();
        
        // Added Default Values to prevent panics, Frontend should handle this
        logs.push(LogData {
            LOG_ID: row.get("LOG_ID").unwrap_or(-1),
            USERNAME: row.get("USERNAME").unwrap_or(None),
            ROUTE: row.get("ROUTE").unwrap_or("NULL".to_string()),
            PARAMETERS: row.get("PARAMETERS").unwrap_or(None),
            RESULT: row.get("RESULT").unwrap_or("NULL".to_string()),
            TIMESTAMP: row.get("TIMESTAMP").unwrap_or("NULL".to_string()),
            TOKEN_USED: row.get("TOKEN_USED").unwrap_or("NULL".to_string()),
            IP_ADDRESS: row.get("IP_ADDRESS").unwrap_or(None),
            METHOD: row.get("METHOD").unwrap_or(None),
        });
    }
    Ok(Json(logs))
}

pub fn get_user_logs_fn(username: String, pool: &State<Pool>, limit: Option<i32>) -> Result<Json<Vec<LogData>>,APIErrors>{
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let logLimit: i32;
    match limit {
        Some(limit) => logLimit = limit,
        None => logLimit = 100,
    }

    let mut logs: Vec<LogData> = Vec::new();
    let stmt = conn.statement("SELECT * FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username ORDER BY TIMESTAMP DESC FETCH NEXT :limit ROWS ONLY").build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();


    let rows = stmt.query(&[&username, &logLimit]);
    if rows.is_err() {
        error!("Error executing statement");
        return Err(APIErrors::DBError);
    }
    let rows = rows.unwrap();

    for row_res in rows {
        let row = row_res;
        if row.is_err() {
            error!("Error getting row");
            return Err(APIErrors::DBError);
        }
        let row = row.unwrap();
        logs.push(LogData {
            LOG_ID: row.get("LOG_ID").unwrap_or(-1),
            USERNAME: row.get("USERNAME").unwrap_or(None),
            ROUTE: row.get("ROUTE").unwrap_or("NULL".to_string()),
            PARAMETERS: row.get("PARAMETERS").unwrap_or(None),
            RESULT: row.get("RESULT").unwrap_or("NULL".to_string()),
            TIMESTAMP: row.get("TIMESTAMP").unwrap_or("NULL".to_string()),
            TOKEN_USED: row.get("TOKEN_USED").unwrap_or("NULL".to_string()),
            IP_ADDRESS: row.get("IP_ADDRESS").unwrap_or(None),
            METHOD: row.get("METHOD").unwrap_or(None),
        });
    }
    Ok(Json(logs))
}

/*
pub fn get_route_logs_fn(route: String, pool: &State<Pool>, limit: Option<i32>) -> Result<Json<Vec<LogData>>,APIErrors>{
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let logLimit: i32;
    match limit {
        Some(limit) => logLimit = limit,
        None => logLimit = 100,
    }

    let mut logs: Vec<LogData> = Vec::new();
    let stmt = conn.statement("SELECT * FROM ODBC_JHC.API_LOGS WHERE ROUTE = :route ORDER BY TIMESTAMP DESC FETCH NEXT :limit ROWS ONLY").build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&route, &logLimit]);
    if rows.is_err() {
        error!("Error executing statement");
        return Err(APIErrors::DBError);
    }
    let rows = rows.unwrap();

    for row_res in rows {
        let row = row_res;
        if row.is_err() {
            error!("Error getting row");
            return Err(APIErrors::DBError);
        }
        let row = row.unwrap();
        logs.push(LogData {
            LOG_ID: row.get("LOG_ID").unwrap(),
            USERNAME: row.get("USERNAME").unwrap(),
            ROUTE: row.get("ROUTE").unwrap(),
            PARAMETERS: row.get("PARAMETERS").unwrap(),
            RESULT: row.get("RESULT").unwrap(),
            TIMESTAMP: row.get("TIMESTAMP").unwrap(),
            TOKEN_USED: row.get("TOKEN_USED").unwrap(),
            IP_ADDRESS: row.get("IP_ADDRESS").unwrap(),
            METHOD: row.get("METHOD").unwrap(),
        });
    }
    Ok(Json(logs))
}
*/

pub fn delete_user_logs_fn(username: String, pool: &State<Pool>, limit: Option<i32>) -> Result<(),APIErrors>{
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();
    let stmt;
    match limit {
        Some(lim)=> {
            stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE ROWID IN (SELECT ROWID FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username ORDER BY LOG_ID DESC FETCH FIRST :limit ROWS ONLY)").build();
            if stmt.is_err() {
                error!("Error building statement");
                return Err(APIErrors::DBError);
            }
            let mut stmt = stmt.unwrap();
            match stmt.execute(&[&username, &lim]) {
                Ok(_) => (),
                Err(_err) => {
                    error!("Error executing statement");
                    return Err(APIErrors::DBError);
                }
            };
        },
        None => {
            stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username").build();
            if stmt.is_err() {
                error!("Error building statement");
                return Err(APIErrors::DBError);
            }
            let mut stmt = stmt.unwrap();
            match stmt.execute(&[&username]) {
                Ok(_) => (),
                Err(err) => {
                    error!("Error: {}", err);
                    return Err(APIErrors::DBError);
                }
            };
            },
    }
    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error: {}", err);
            return Err(APIErrors::DBError);
        }
    }
    
    Ok(())
}

pub fn delete_log_logs_fn(log_id: i32, pool: &State<Pool>) -> Result<(),APIErrors>{
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE LOG_ID = :log_id").build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[&log_id]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(APIErrors::DBError);
        }
    };
    
    
    match conn.commit() {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error: {}", err);
            return Err(APIErrors::DBError);
        }
    }

}
