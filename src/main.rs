#[macro_use] extern crate rocket;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use oracle::pool::Pool;

use rocket::{State, post};


use oracle::pool::PoolBuilder;
use rocket::serde::json::Json;


mod getproductdata;
mod apistructs;


use crate::getproductdata::get_product_data;
use crate::apistructs::FetchParams;
use crate::apistructs::Product;

#[launch]
fn rocket() -> _ {
    let username = "odbc_jhc";
    let password = "odbc_jhc";
    let database = "//10.0.0.21:1521/a12";

    let pool = PoolBuilder::new(username, password, database)
    .min_connections(10) // Had to specify, would otherwise cause error: Invalid number of sessions
    .max_connections(10) // min and max must be the same for it to work on linux?
    .build();

    let pool = match pool {
        Ok(pool) => pool,
        Err(err) => panic!("Error Creating Pool: {}", err.to_string()),
    };
    rocket::build().manage(pool).mount("/", routes![get_products])
}

#[post("/GetProductData", data = "<params>")]
async fn get_products(params: Json<FetchParams>, pool: &State<Pool>) -> Option<Json<Vec<Product>>> {
    println!("Time on request: {:?}", SystemTime::now().duration_since(UNIX_EPOCH));
    return get_product_data(params, pool).await;
}