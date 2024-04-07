use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Store {
    pub STORE_ID: Option<String>,
    pub STORE_DESC: Option<String>,
    pub STORE_DESC_S: Option<String>,
}

#[derive(serde::Deserialize, Debug, Serialize, Clone)]
pub struct StoreListUpdateParams {
    pub p_username: String,
    pub p_stores: Option<Vec<i8>>,
    pub p_allstoresaccess: i8,
}
