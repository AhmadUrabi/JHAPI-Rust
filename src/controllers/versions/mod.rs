pub mod structs;

use crate::{
    controllers::versions::structs::Version,
    utils::{sql::SQLManager, structs::APIError},
};
use oracle::pool::Pool;
use rocket::serde::json::Json;

pub async fn get_latest_version(
    platform: &str,
    sql_manager: &SQLManager,
    pool: &Pool,
) -> Result<Json<Version>, APIError> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error: {}", conn.err().unwrap());
        return Err(APIError::InternalServerError);
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement(sql_manager.get_sql("get_platform_version")?.as_str())
        .build();
    println!("{:?}", stmt);
    if stmt.is_err() {
        error!("Error: {}", stmt.err().unwrap());
        return Err(APIError::InternalServerError);
    }
    let mut stmt = stmt.unwrap();

    let rows_query = stmt.query(&[&platform]);
    match rows_query {
        Ok(rows) => {
            let rows: Vec<_> = rows.collect();
            if rows.is_empty() {
                return Err(APIError::NoData);
            }
            let row = &rows[0];
            match row {
                Ok(row) => {
                    info!("Version Found");
                    Ok(Json(Version {
                        version: row.get("VERSION").unwrap(),
                        platform: row.get("PLATFORM").unwrap(),
                        url: row.get("URL").unwrap(),
                        release_date: row.get("RELEASE_DATE").unwrap(),
                    }))
                }
                Err(err) => {
                    error!("Error: {}", err);
                    Err(APIError::NoData)
                }
            }
        }
        Err(err) => {
            error!("Error: {}", err);
            Err(APIError::DBError)
        }
    }
}
