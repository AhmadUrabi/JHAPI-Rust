use std::path::Path;

use tokio::fs;

use super::structs::APIErrors;

use std::collections::HashMap;

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

pub fn load_to_hashmap() -> Result<HashMap<String,String>,APIErrors> {
    let mut table: HashMap<String,String> = HashMap::new();
    let sql_path = Path::new("src/sql");
    let files = std::fs::read_dir(sql_path);
    match files {
        Ok(mut filelist)=>{
            while let Some(entry) = filelist.next(){
                let entry = entry.unwrap();
                let data = std::fs::read_to_string(entry.path());
                table.entry(entry.file_name().to_str().unwrap().to_string()).or_insert(data.unwrap());
            }
        },
        Err(_err) => {
            return Err(APIErrors::IOError);
        }
    }

    Ok(table)
}

pub struct SQLManager{
    map: HashMap<String,String>
}

impl SQLManager {
    pub fn init() -> Self {
        SQLManager {
            map: load_to_hashmap().unwrap()
        }
    }
    pub fn read_sql(self, function: &str) -> Result<String,APIErrors> {
        if let Some(res) = self.map.get(function) {
            return Ok(res.to_owned());
        } else {
            return Err(APIErrors::IOError);
        }
    }
}