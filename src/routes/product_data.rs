#![allow(non_snake_case)]
use oracle::pool::Pool;

use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{post, State};

use std::net::IpAddr;

use crate::product_data::get_product;
// use crate::product_data::get_product_pi;
use crate::signing::decode_token_data;
use crate::ApiKey;
use crate::LogCheck;

use crate::product_data::structs::FetchParams;
use crate::product_data::structs::Product;
use crate::utils::logging::get_timestamp;
use crate::utils::logging::log_data;
use crate::utils::permissions::is_admin_perm;

#[post("/products", data = "<params>")]
pub async fn get_products(
    params: Json<FetchParams>,
    pool: &State<Pool>,
    key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Json<Vec<Product>> {
    info!("GetProductData Request: {:?}", params);

    #[allow(unused_assignments)]
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

    // Convert json to String

    let params_clone = params.clone();

    match get_product(params, pool, &key).await {
        Ok(products) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&key, pool)) {
                log_data(
                    pool,
                    username,
                    client_ip.unwrap().to_string(),
                    "/GetProductData".to_string(),
                    Some(serde_json::to_string(&params_clone.0).unwrap()),
                    get_timestamp(),
                    key.clone().0.to_string(),
                    "Success".to_string(),
                    "GET".to_string(),
                );
            }
            Json(products)
        }
        Err(_err) => {
            error!("Error");
            if log_check.0 || (!log_check.0 && !is_admin_perm(&key, pool)) {
                log_data(
                    pool,
                    username,
                    client_ip.unwrap().to_string(),
                    "/GetProductData".to_string(),
                    Some(serde_json::to_string(&params_clone.0).unwrap()),
                    get_timestamp(),
                    key.clone().0.to_string(),
                    "Error Fetching".to_string(),
                    "GET".to_string(),
                );
            }
            Json(vec![])
        }
    }
}

/*
#[post("/GetProductDataPI", data = "<params>")]
pub async fn get_products_pi(
    params: Json<FetchParams>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Json<Vec<Product>> {
    info!("GetProductData Request: {:?}", params);
    info!("Client IP: {:?}", client_ip);
    #[allow(unused_assignments)]
    let mut username: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            username = data.USER_ID.unwrap();
        }
        None => {
            info!("Token Data: None");
            username = "None".to_string();
        }
    }

    let tokenUsed = _key.0.to_string();

    // Convert json to String

    let params_clone = params.clone();

    match get_product_pi(params, pool, &_key).await {
        Ok(products) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/GetProductData".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                get_timestamp(),
                tokenUsed,
                "Success".to_string(),
                "GET".to_string()
            );
        }
            Json(products)
        }
        Err(err) => {
            error!("Error: {}", err);
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/GetProductData".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                get_timestamp(),
                tokenUsed,
                "Error Fetching".to_string(),
                "GET".to_string()
            );
        }
            Json(vec![])
        }
    }
}
*/
