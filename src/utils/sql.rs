use std::path::Path;

use tokio::fs;

use super::structs::APIError;

use std::collections::HashMap;

#[allow(dead_code)]
pub async fn read_sql(name: &str) -> Result<String, APIError> {
    let start = std::time::Instant::now();
    let path = Path::new("src/sql").join(name.to_owned() + ".sql");
    let sql = fs::read_to_string(path).await;
    if sql.is_err() {
        return Err(APIError::IOError);
    }
    let sql = sql.unwrap();
    info!("Read SQL file in {} ms", start.elapsed().as_millis());
    Ok(sql)
}

pub async fn load_to_hashmap() -> Result<HashMap<String, String>, APIError> {
    println!("Loading SQL Files");
    let mut table: HashMap<String, String> = HashMap::new();
    let sql_path = Path::new("src/sql");
    let files = fs::read_dir(sql_path).await;
    match files {
        Ok(mut filelist) => {
            while let Ok(entry) = filelist.next_entry().await {
                if entry.is_none() {
                    break;
                }
                let entry = entry.unwrap();
                let data = std::fs::read_to_string(entry.path());
                table
                    .entry(
                        entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .to_string()
                            .replace(".sql", ""),
                    )
                    .or_insert(data.unwrap());
            }
        }
        Err(_err) => {
            return Err(APIError::IOError);
        }
    }
    println!("Loaded SQL Files");
    Ok(table)
}

pub struct SQLManager {
    pub map: HashMap<String, String>,
}

impl SQLManager {
    pub async fn init() -> Self {
        SQLManager {
            map: load_to_hashmap().await.unwrap(),
        }
    }
    pub fn get_sql(&self, function: &str) -> Result<String, APIError> {
        println!("Function: {}", function);
        if let Some(res) = self.map.get(function) {
            return Ok(res.to_owned());
        } else {
            return Err(APIError::IOError);
        }
    }
}
