#![allow(non_snake_case)]
#[macro_use]
extern crate rocket;

mod fetch_stores;
mod permissions;
mod product_data;
mod routes;
mod signing;
mod user_control;
mod utils;

use dotenv::dotenv;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::Response;

use oracle::pool::PoolBuilder;

use routes::fetch_stores::get_store_list;
use routes::file_server::files;
use routes::permissions::edit_permissions;
use routes::permissions::get_permissions;
use routes::product_data::get_products;
use routes::signing::sign;
use routes::user_control::create_user_route;
use routes::user_control::delete_user_route;
use routes::user_control::edit_user_route;
use routes::user_control::get_user_by_id;
use routes::user_control::get_user_list;
// use crate::routes::user_control::edit_user;

use signing::validate_token;

// CORS Setup
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, PUT, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    // Load .env file
    dotenv().ok();

    // Logging Setup
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // Logging Setup End

    //let routes = routes![get_products, get_store_list, sign, files, get_permissions, edit_permissions, get_user_list];

    // Build Connection Pool
    let username = std::env::var("LOGIN_USERNAME").expect("LOGIN_USERNAME must be set.");
    let password = std::env::var("LOGIN_PASSWORD").expect("LOGIN_PASSWORD must be set.");
    let database = std::env::var("DB_CONNECTION").expect("DB_CONNECTION must be set.");

    let pool = PoolBuilder::new(username, password, database)
        .min_connections(10) // Had to specify, would otherwise cause error: Invalid number of sessions
        .max_connections(10) // min and max must be the same for it to work on linux? TODO: Test with new values
        .build();

    let pool = match pool {
        Ok(pool) => pool,
        Err(err) => panic!("Error Creating Pool: {}", err.to_string()),
    };
    // Pool built

    rocket::build()
        .register("/", catchers![Unauthorized, not_found])
        .manage(pool)
        .mount(
            "/",
            routes![
                get_products,
                get_store_list,
                sign,
                files,
                get_permissions,
                edit_permissions,
                get_user_list,
                get_user_by_id,
                create_user_route,
                edit_user_route,
                delete_user_route
            ],
        )
        .attach(CORS)
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
                Outcome::Failure((
                    Status::Unauthorized,
                    "Please include an Authentication header".to_string(),
                ))
            }
            Some(key) if validate_token(key) => {
                info!("Valid Token Found");
                Outcome::Success(ApiKey(key))
            }
            Some(_) => {
                error!("Invalid Token Found");
                Outcome::Failure((
                    Status::Unauthorized,
                    "Please include a valid Authentication header".to_string(),
                ))
            }
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
