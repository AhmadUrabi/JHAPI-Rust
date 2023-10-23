#![allow(non_snake_case)]
use oracle::pool::Pool;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{post, State};

use std::net::IpAddr;

use crate::product_data::get_product;
use crate::signing::decode_token_data;
use crate::ApiKey;

use crate::product_data::structs::FetchParams;
use crate::product_data::structs::Product;

#[post("/GetProductData", data = "<params>")]
pub async fn get_products(
    params: Json<FetchParams>,
    pool: &State<Pool>,
    key: ApiKey<'_>,
    client_ip: Option<IpAddr>
) -> Json<Vec<Product>> {
    info!("GetProductData Request: {:?}", params);
    info!("Client IP: {:?}", client_ip);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    match get_product(params, pool, key).await {
        Ok(products) => Json(products),
        Err(err) => {
            error!("Error: {}", err);
            Json(vec![])
        }
    }
}
