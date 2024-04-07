pub mod structs;

use crate::{utils::structs::APIErrors, functions::versions::structs::Version};
use oracle::pool::Pool;
use rocket::{serde::json::Json, State};

pub fn get_latest_version(platform: &str, pool: &State<Pool>) -> Result<Json<Version>, APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error: {}", conn.err().unwrap());
        return Err(APIErrors::InternalServerError);
    }
    let conn = conn.unwrap();

    let stmt = conn.statement("SELECT * FROM ODBC_JHC.VERSION_CHECK WHERE PLATFORM = :1 ORDER BY RELEASE_DATE DESC FETCH NEXT 1 ROWS ONLY").build();
    if stmt.is_err() {
        error!("Error: {}", stmt.err().unwrap());
        return Err(APIErrors::InternalServerError);
    }
    let mut stmt = stmt.unwrap();

    let rows_query = stmt.query(&[&platform]);
    match rows_query {
        Ok(rows) => {
            let rows: Vec<_> = rows.collect();
            if rows.is_empty() {
                return Err(APIErrors::NoData);
            }
            let row = &rows[0];
            match row {
                Ok(row) => {
                    info!("Version Found");
                    Ok(Json(Version {
                        // TODO: Fix this unwrap
                        version: row.get("VERSION").unwrap(),
                        platform: row.get("PLATFORM").unwrap(),
                        url: row.get("URL").unwrap(),
                        release_date: row.get("RELEASE_DATE").unwrap(),
                    }))
                }
                Err(err) => {
                    error!("Error: {}", err);
                    Err(APIErrors::NoData)
                }
            }
        }
        Err(err) => {
            error!("Error: {}", err);
            Err(APIErrors::DBError)
        }
    }
}
