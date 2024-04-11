use std::path::Path;

use tokio::fs;

use super::structs::APIErrors;

pub async fn read_sql(name: &str) -> Result<String, APIErrors> {
    let start = std::time::Instant::now();
    let path = Path::new("src/sql").join(name.to_owned() + ".sql");
    let sql = fs::read_to_string(path).await;
    if sql.is_err() {
        return Err(APIErrors::IOError);
    }
    let sql = sql.unwrap();
    info!("Read SQL file in {} ms", start.elapsed().as_millis());
    Ok(sql)
}