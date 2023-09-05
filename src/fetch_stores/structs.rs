use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub STORE_ID : Option<String>,
    pub STORE_DESC : Option<String>,
    pub STORE_DESC_S : Option<String>,
}