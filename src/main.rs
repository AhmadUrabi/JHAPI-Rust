#![allow(non_snake_case)]

#[macro_use] extern crate rocket;

use rocket::log::private::info;
use rocket::{State, post};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::fs::{relative};

use oracle::pool::Pool;
use oracle::pool::PoolBuilder;

mod getproductdata;
mod apistructs;
mod fetchstores;
mod signing;

use crate::getproductdata::get_product;
use crate::fetchstores::fetch_store_list;
use crate::signing::validate_token;
use crate::signing::decode_token_data;

use crate::apistructs::FetchParams;
use crate::apistructs::LoginParams;
use crate::apistructs::Product;
use crate::apistructs::Store;




#[launch]
fn rocket() -> _ {
    // Logging Setup
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // Logging Setup End

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

    rocket::build().register("/", catchers![Unauthorized, not_found]).manage(pool).mount("/", routes![get_products, get_store_list, sign]).mount("/images", rocket::fs::FileServer::from(relative!("static")))
}

// Start Request Guard Functions
#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid JWT Token.

        match req.headers().get_one("Authentication") {
            None => {
                error!("No Authentication header found");
                Outcome::Failure((Status::Unauthorized, "Please include an Authentication header".to_string()))
            },
            Some(key) if validate_token(key) => {
                info!("Valid Token Found");
                Outcome::Success(ApiKey(key))
            },
            Some(_) => {
                error!("Invalid Token Found");
                Outcome::Failure((Status::Unauthorized, "Please include a valid Authentication header".to_string()))},
        }
    }
}
// End Request Guard Functions

#[post("/GetProductData", data = "<params>")]
async fn get_products(params: Json<FetchParams>, pool: &State<Pool>, key: ApiKey<'_>) -> Json<Vec<Product>> {
    
    info!("GetProductData Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    // No error handling as the function will always return a result
    Json(get_product(params, pool, key).unwrap())
}

#[get("/StoreList")]
async fn get_store_list(pool: &State<Pool>, _key: ApiKey<'_>) -> Json<Vec<Store>> {

    info!("StoreList Request");
    match decode_token_data(_key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    Json(fetch_store_list(pool).await)
}

#[post("/Sign", data = "<params>")]
async fn sign(params: Json<LoginParams>, pool: &State<Pool>) -> Result<Json<String>, Status> {
    info!("Sign Request: {:?}", params.0.pUserName);
    match signing::signin(params, pool).await {
        Some(token) => {
            info!("Valid User Data, Token Sent");
            Ok(Json(token.to_string()))
        },
        None => {
            error!("Invalid User Data, Token Not Sent");
            Err(Status::Unauthorized)
        },
    }
}

// Route catchers
#[catch(401)]
fn Unauthorized() -> &'static str {
    "Unauthorized, please include a valid Authentication header, or check your request body"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
