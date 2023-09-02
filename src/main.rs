#![allow(non_snake_case)]

#[macro_use] extern crate rocket;

use rocket::log::private::info;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

use oracle::pool::PoolBuilder;

mod product_data;
mod fetch_stores;
mod signing;
mod routes;

use crate::routes::product_data::get_products;
use crate::routes::fetch_stores::get_store_list;
use crate::routes::signing::sign;
use crate::routes::file_server::files;

use crate::signing::validate_token;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::Response;
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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

    rocket::build().register("/", catchers![Unauthorized, not_found]).manage(pool).mount("/", routes![get_products, get_store_list, sign, files]).attach(CORS)
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







// Route catchers
#[catch(401)]
fn Unauthorized() -> &'static str {
    "Unauthorized, please include a valid Authentication header, or check your request body"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
