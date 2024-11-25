#[macro_use]
extern crate rocket;

mod functions;
mod routes;
mod server;
mod utils;

use dotenv::dotenv;

use routes::get_all_routes;
use server::JHApiServer;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Logging Setup, Unwrapping is fine here, if it fails, the program should crash
    // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // Logging Setup End

    let routes = get_all_routes();

    let server = JHApiServer::init(routes).await;
    let _ = server.launch().await;
}
