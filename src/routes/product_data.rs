#![allow(non_snake_case)]
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{State, post};
use oracle::pool::Pool;

use crate::ApiKey;
use crate::product_data::get_product;
use crate::signing::decode_token_data;

use crate::product_data::structs::Product;
use crate::product_data::structs::FetchParams;

#[post("/GetProductData", data = "<params>")]
pub async fn get_products(params: Json<FetchParams>, pool: &State<Pool>, key: ApiKey<'_>) -> Json<Vec<Product>> {
    
    info!("GetProductData Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    // No error handling as the function will always return a result
    //Json(get_product(params, pool, key).unwrap())
    match get_product(params, pool, key) {
        Ok(products) => Json(products),
        Err(err) => {
            error!("Error: {}", err);
            Json(vec![])
        },
    }

}