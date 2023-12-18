pub mod structs;

use crate::version_check::structs::Version;
use oracle::{pool::Pool, Error};
use rocket::{State, serde::json::Json};

pub fn get_latest_version(platform: &str, pool: &State<Pool>) -> Result<Json<Version>, Error> {
    let conn = pool.get().unwrap();
    
    let mut stmt = conn
        .statement("SELECT * FROM ODBC_JHC.VERSION_CHECK WHERE PLATFORM = :1 ORDER BY RELEASE_DATE DESC FETCH NEXT 1 ROWS ONLY").build()?;
    let rows = stmt.query(&[&platform]);
    match rows {
        Ok(mut rows) => {
            
            let row = rows.next().unwrap()?;
            Ok(Json(Version {
                version: row.get("VERSION").unwrap(),
                platform: row.get("PLATFORM").unwrap(),
                url: row.get("URL").unwrap(),
                release_date: row.get("RELEASE_DATE").unwrap(),
            }))
        }
        Err(err) => {
            error!("Error: {}", err);
            Err(err)
        }
    }
}