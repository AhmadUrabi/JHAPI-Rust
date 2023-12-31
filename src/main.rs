#[macro_use]
extern crate rocket;

mod fetch_stores;
mod file_server;
mod logs;
mod permissions;
mod product_data;
mod routes;
mod signing;
mod user_control;
mod utils;
mod version_check;

use dotenv::dotenv;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::Response;

use oracle::pool::PoolBuilder;

use routes::fetch_stores::get_store_list;
use routes::fetch_stores::get_store_list_for_user;
use routes::fetch_stores::update_store_list;
use routes::file_server::get_image;
use routes::file_server::upload;
use routes::permissions::edit_permissions;
use routes::permissions::get_permissions;
use routes::product_data::get_products;
// use routes::product_data::get_products_pi;
use routes::logs::get_user_logs;
use routes::signing::sign;
use routes::user_control::create_user_route;
use routes::user_control::delete_user_route;
use routes::user_control::edit_user_route;
use routes::user_control::get_user_by_id;
use routes::user_control::get_user_list;
/*use routes::logs::get_route_logs;*/
use routes::logs::delete_log_logs;
use routes::logs::delete_user_logs;
use routes::logs::get_all_logs;
use routes::version_check::route_version_check;
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

pub struct IPCheck;

#[rocket::async_trait]
impl<'r> Fairing for IPCheck {
    fn info(&self) -> Info {
        Info {
            name: "IP Check",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, req: &mut Request<'_>, _data: &mut rocket::Data<'_>) {
        match req.remote() {
            Some(i) => println!("Client IP: {}", i.ip()),
            None => println!("Client IP: Unknown"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LogCheck(pub bool);

// Hack: To handle Options request on firefox
#[options("/<_path..>")]
fn cors_preflight_handler(_path: std::path::PathBuf) -> rocket::http::Status {
    rocket::http::Status::Ok
}

#[launch]
fn rocket() -> _ {
    // Load .env file
    dotenv().ok();

    // Logging Setup, Unwrapping is fine here, if it fails, the program should crash
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // Logging Setup End

    //let routes = routes![get_products, get_store_list, sign, files, get_permissions, edit_permissions, get_user_list];

    // Build Connection Pool, program should crash if it fails
    let username = std::env::var("LOGIN_USERNAME").expect("LOGIN_USERNAME must be set.");
    let password = std::env::var("LOGIN_PASSWORD").expect("LOGIN_PASSWORD must be set.");
    let database = std::env::var("DB_CONNECTION").expect("DB_CONNECTION must be set.");

    let pool = PoolBuilder::new(username, password, database)
        .min_connections(8) // Min == Max always
        .max_connections(8)
        .build();

    // If pool is an error, log and exit
    if pool.is_err() {
        error!("Failed to build connection pool");
        std::process::exit(1);
    }
    let pool = pool.unwrap();
    // Pool built

    rocket::build()
        .register(
            "/",
            catchers![
                unauthorized,
                not_found,
                internal_error,
                bad_request,
                unprocessable_entity,
                conflict
            ],
        )
        .manage(pool)
        .mount(
            "/api",
            routes![
                get_products,
                get_store_list,
                update_store_list,
                sign,
                get_permissions,
                edit_permissions,
                get_user_list,
                get_user_by_id,
                create_user_route,
                edit_user_route,
                delete_user_route,
                get_image,
                upload,
                cors_preflight_handler,
                get_store_list_for_user,
                get_user_logs,
                //get_route_logs,
                get_all_logs,
                //get_products_pi,
                delete_log_logs,
                delete_user_logs,
                route_version_check,
            ],
        )
        .attach(CORS)
        .attach(IPCheck)
}

// Start Request Guard Functions
#[derive(Debug, Clone)]
pub struct ApiKey<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid JWT Token.

        match req.headers().get_one("Authorization") {
            None => {
                error!("No Authentication header found");
                Outcome::Error((
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
                Outcome::Error((
                    Status::Unauthorized,
                    "Please include a valid Authentication header".to_string(),
                ))
            }
        }
    }
}

// Used to check for X-Log-Request header
#[rocket::async_trait]
impl<'r> FromRequest<'r> for LogCheck {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        match request.headers().get_one("X-Log-Request") {
            Some(key) => {
                if key == "false" {
                    Outcome::Success(LogCheck(false))
                } else {
                    Outcome::Success(LogCheck(true))
                }
            }
            _ => Outcome::Success(LogCheck(true)),
        }
    }
}
// End Request Guard Functions

// Route catchers

#[catch(400)]
fn bad_request() -> &'static str {
    "Bad Request, please make sure your request body is valid"
}

#[catch(401)]
fn unauthorized() -> &'static str {
    "Unauthorized, please include a valid Authentication header, or check your request body"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(409)]
fn conflict(_req: &Request) -> String {
    format!("Data Conflict, please make sure you are not trying to insert duplicate data")
}

#[catch(422)]
fn unprocessable_entity(_req: &Request) -> String {
    format!("The body data is invalid, please make sure you are following the correct structure")
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}
