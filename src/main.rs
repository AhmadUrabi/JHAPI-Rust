#[macro_use]
extern crate rocket;

mod controllers;
mod routes;
mod server;
mod utils;

use dotenv::dotenv;

use server::JHApiServer;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    let server = JHApiServer::init().await;
    let _ = server.launch().await;
}
