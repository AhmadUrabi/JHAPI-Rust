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
use crate::utils::logging::getTimestamp;
use crate::utils::logging::log_data;

#[post("/GetProductData", data = "<params>")]
pub async fn get_products(
    params: Json<FetchParams>,
    pool: &State<Pool>,
    key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
) -> Json<Vec<Product>> {
    info!("GetProductData Request: {:?}", params);
    info!("Client IP: {:?}", client_ip);
    let mut username: String = "".to_string();
    match decode_token_data(key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            username = data.USER_ID.unwrap();
        }
        None => {
            info!("Token Data: None");
            username = "None".to_string();
        }
    }

    let tokenUsed = key.0.to_string();

    // Convert json to String

    let params_clone = params.clone();

    match get_product(params, pool, key).await {
        Ok(products) => {
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/GetProductData".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                getTimestamp(),
                tokenUsed,
                "Success".to_string(),
            );
            Json(products)
        }
        Err(err) => {
            error!("Error: {}", err);
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/GetProductData".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                getTimestamp(),
                tokenUsed,
                "Error Fetching".to_string(),
            );
            Json(vec![])
        }
    }
}
