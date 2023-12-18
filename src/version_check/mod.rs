pub mod structs;

use crate::version_check::structs::Version;
use oracle::{pool::Pool, Error};
use rocket::{serde::json::Json, State};

pub fn get_latest_version(platform: &str, pool: &State<Pool>) -> Result<Json<Version>, Error> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error: {}", conn.err().unwrap());
        return Err(Error::InternalError("Error Getting Connection".to_string()));
    }
    let conn = conn.unwrap();
    let mut stmt = conn
        .statement("SELECT * FROM ODBC_JHC.VERSION_CHECK WHERE PLATFORM = :1 ORDER BY RELEASE_DATE DESC FETCH NEXT 1 ROWS ONLY").build()?;
    let rows_query = stmt.query(&[&platform]);
    match rows_query {
        Ok(rows) => {
            let rows: Vec<_> = rows.collect();
            if rows.is_empty() {
                return Err(Error::NullValue);
            }
            let row = &rows[0];
            match row {
                Ok(row) => {
                    info!("Version Found");
                    Ok(Json(Version {
                        version: row.get("VERSION")?,
                        platform: row.get("PLATFORM")?,
                        url: row.get("URL")?,
                        release_date: row.get("RELEASE_DATE")?,
                    }))
                }
                Err(err) => {
                    error!("Error: {}", err);
                    Err(Error::NullValue)
                }
            }
           
        }
        Err(err) => {
            error!("Error: {}", err);
            Err(err)
        }
    }
}
