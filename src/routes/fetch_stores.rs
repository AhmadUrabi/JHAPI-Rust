use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{State, get};
use oracle::pool::Pool;

use crate::ApiKey;

use crate::fetch_stores::fetch_store_list;
use crate::signing::decode_token_data;

use crate::fetch_stores::structs::Store;

#[get("/StoreList")]
pub async fn get_store_list(pool: &State<Pool>, _key: ApiKey<'_>) -> Json<Vec<Store>> {

    info!("StoreList Request");
    match decode_token_data(_key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    Json(fetch_store_list(pool).await)
}