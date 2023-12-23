#![allow(non_snake_case)]
use oracle::Error;
use oracle::pool::Pool;


use rocket::State;
use rocket::serde::json::Json;

pub mod structs;

use structs::LogData;


pub fn get_all_logs_fn(pool: &State<Pool>, limit: Option<i32>) -> Result<Json<Vec<LogData>>,Error>{
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap());
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
        return Err(stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();


    let rows = stmt.query(&[&logLimit]);
    if rows.is_err() {
        error!("Error executing statement");
        return Err(rows.err().unwrap());
    }
    let rows = rows.unwrap();

    for row_res in rows {
        let row = row_res;
        if row.is_err() {
            error!("Error getting row");
            return Err(row.err().unwrap());
        }
        let row = row.unwrap();

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
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap());
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
        return Err(stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();


    let rows = stmt.query(&[&username, &logLimit]);
    if rows.is_err() {
        error!("Error executing statement");
        return Err(rows.err().unwrap());
    }
    let rows = rows.unwrap();

    for row_res in rows {
        let row = row_res;
        if row.is_err() {
            error!("Error getting row");
            return Err(row.err().unwrap());
        }
        let row = row.unwrap();
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
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap());
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
        return Err(stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&route, &logLimit]);
    if rows.is_err() {
        error!("Error executing statement");
        return Err(rows.err().unwrap());
    }
    let rows = rows.unwrap();

    for row_res in rows {
        let row = row_res;
        if row.is_err() {
            error!("Error getting row");
            return Err(row.err().unwrap());
        }
        let row = row.unwrap();
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
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap());
    }
    let conn = conn.unwrap();
    let stmt;
    match limit {
        Some(lim)=> {
            stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE ROWID IN (SELECT ROWID FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username ORDER BY TIMESTAMP DESC FETCH FIRST :limit ROWS ONLY)").build();
            if stmt.is_err() {
                error!("Error building statement");
                return Err(stmt.err().unwrap());
            }
            let mut stmt = stmt.unwrap();
            match stmt.execute(&[&username, &lim]) {
                Ok(_) => (),
                Err(err) => {
                    error!("Error executing statement");
                    return Err(err);
                }
            };
        },
        None => {
            stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username").build();
            if stmt.is_err() {
                error!("Error building statement");
                return Err(stmt.err().unwrap());
            }
            let mut stmt = stmt.unwrap();
            match stmt.execute(&[&username]) {
                Ok(_) => (),
                Err(err) => {
                    error!("Error: {}", err);
                    return Err(err);
                }
            };
            },
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
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap().to_string());
    }
    let conn = conn.unwrap();

    let stmt = conn.statement("DELETE FROM ODBC_JHC.API_LOGS WHERE LOG_ID = :log_id").build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap().to_string());
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[&log_id]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(err.to_string());
        }
    };
    
    
    match conn.commit() {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error: {}", err);
            Err("Error".to_string())
        }
    }

}
