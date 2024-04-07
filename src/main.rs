#[macro_use]
extern crate rocket;

mod functions;
mod routes;
mod utils;
mod server;

use dotenv::dotenv;

use routes::fetch_stores::*;
use routes::file_server::*;
use routes::logs::*;
use routes::permissions::*;
use routes::product_data::*;
use routes::signing::*;
use routes::user_control::*;
use routes::version_check::*;

use server::JHApiServer;

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

    let routes = routes![
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
        get_all_logs,
        delete_log_logs,
        delete_user_logs,
        route_version_check,
    ];

    let server = JHApiServer::init(routes);
    server.server
}