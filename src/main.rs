#![allow(non_snake_case)]

#[macro_use] extern crate rocket;

use rocket::{State, post};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

use oracle::pool::Pool;
use oracle::pool::PoolBuilder;

mod getproductdata;
mod apistructs;
mod fetchstores;
mod signing;

use crate::getproductdata::get_product_data;
use crate::fetchstores::fetch_store_list;
use crate::signing::validate_token;

use crate::apistructs::FetchParams;
use crate::apistructs::LoginParams;
use crate::apistructs::Product;
use crate::apistructs::Store;

// Start Request Guard Structs
struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}
// End Request Guard Structs

#[launch]
fn rocket() -> _ {
    // Build Connection Pool
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
    // Pool built

    rocket::build().register("/", catchers![Unauthorized, not_found]).manage(pool).mount("/", routes![get_products, get_store_list, sign])
}

// Start Request Guard Functions
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid JWT Token.

        match req.headers().get_one("Authentication") {
            None => Outcome::Failure((Status::Unauthorized, "Please include an Authentication header".to_string())),
            Some(key) if validate_token(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Unauthorized, "Please include a valid Authentication header".to_string())),
        }
    }
}
// End Request Guard Functions

#[post("/GetProductData", data = "<params>")]
async fn get_products(params: Json<FetchParams>, pool: &State<Pool>, key: ApiKey<'_>) -> Option<Json<Vec<Product>>> {
    return get_product_data(params, pool).await;
}

#[get("/StoreList")]
async fn get_store_list(pool: &State<Pool>, key: ApiKey<'_>) -> Option<Json<Vec<Store>>> {
    return fetch_store_list(pool).await;
}

// Catcher Test

#[catch(401)]
fn Unauthorized() -> &'static str {
    "Unauthorized, please include a valid Authentication header, or check your request body"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

//


#[post("/Sign", data = "<params>")]
async fn sign(params: Json<LoginParams>, pool: &State<Pool>) -> Result<Json<String>, Status> {
    match signing::signin(params, pool).await {
        Some(token) => Ok(Json(token.to_string())),
        None => Err(Status::Unauthorized),
    }
}