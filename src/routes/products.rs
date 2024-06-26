#![allow(non_snake_case)]
use crate::server::JHApiServerState;

use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::functions::products::get_product;
use crate::server::request_guard::api_key::ApiKey;

use crate::functions::products::structs::FetchParams;
use crate::functions::products::structs::Product;

#[post("/products", data = "<params>")]
pub async fn get_products(
    params: Json<FetchParams>,
    state: &State<JHApiServerState>,
    key: ApiKey<'_>,
) -> Json<Vec<Product>> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    info!("GetProductData Request: {:?}", params);
    match get_product(params, &pool, &sql_manager, &key).await {
        Ok(products) => {
            Json(products)
        }
        Err(_err) => {
            error!("Error");
            Json(vec![])
        }
    }
}

/*
#[post("/GetProductDataPI", data = "<params>")]
pub async fn get_products_pi(
    params: Json<FetchParams>,
    state: &State<JHApiServerState>,
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
