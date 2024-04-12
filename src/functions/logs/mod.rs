#![allow(non_snake_case)]
use oracle::pool::{self, Pool};

use rocket::serde::json::Json;
use rocket::State;

pub mod structs;

use structs::LogData;


use crate::utils::sql::SQLManager;
use crate::utils::structs::APIErrors;

pub async fn get_all_logs_fn(
    pool: &Pool,
    sql_manager: &SQLManager,
    limit: Option<i32>,
) -> Result<Json<Vec<LogData>>, APIErrors> {
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
    let stmt = conn
        .statement(sql_manager.get_sql("get_all_logs")?.as_str())
        .build();
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

pub async fn get_user_logs_fn(
    username: String,
    pool: &Pool,
    sql_manager: &SQLManager,
    limit: Option<i32>,
) -> Result<Json<Vec<LogData>>, APIErrors> {
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
    let stmt = conn.statement(sql_manager.get_sql("get_user_logs")?.as_str()).build();
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

pub async fn delete_user_logs_fn(
    username: String,
    pool: &Pool,
    sql_manager: &SQLManager,
    limit: Option<i32>,
) -> Result<(), APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();
    let stmt;
    match limit {
        Some(lim) => {
            stmt = conn.statement(sql_manager.get_sql("delete_user_logs_limit")?.as_str()).build();
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
        }
        None => {
            stmt = conn
                .statement(sql_manager.get_sql("delete_user_logs")?.as_str())
                .build();
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
        }
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

pub async fn delete_log_logs_fn(log_id: i32, pool: &Pool,sql_manager: &SQLManager) -> Result<(), APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement(sql_manager.get_sql("delete_log_by_id")?.as_str())
        .build();
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
