use oracle::pool::{Pool, PoolBuilder};

mod fairings;
mod catchers;
pub mod request_guard;

use fairings::log::Logger;
use fairings::cors::CORS;
use rocket::{Ignite, Rocket};

use crate::utils::sql::SQLManager;
use crate::utils::structs::APIErrors;

pub struct JHApiServer {
    pub server: rocket::Rocket<rocket::Build>,
}

pub struct JHApiServerState {
    pub pool: oracle::pool::Pool,
    pub sql_manager: SQLManager,
}

// TODO: Switch to a single shared object

impl JHApiServer {
    pub async fn init(routes: Vec<rocket::Route>) -> JHApiServer {
        let state = JHApiServer::get_state().await;
        let rocket = rocket::build()
        .attach(CORS)
        .attach(Logger)
        .register(
            "/",
            Self::get_catchers()
        )
        .manage(state)
        .mount(
            "/api",
            routes
        );
        JHApiServer {
            server: rocket
        }
    }

    fn build_pool() -> Result<Pool, APIErrors> {
        let username = std::env::var("LOGIN_USERNAME").expect("LOGIN_USERNAME must be set.");
        let password = std::env::var("LOGIN_PASSWORD").expect("LOGIN_PASSWORD must be set.");
        let database = std::env::var("DB_CONNECTION").expect("DB_CONNECTION must be set.");
    
        let pool = PoolBuilder::new(username, password, database)
            .min_connections(8) // Min == Max always
            .max_connections(8)
            .build();
    
        // If pool is an error, log and exit
        pool.map_err(|e| {
            error!("Error building pool: {:?}", e);
            APIErrors::DBError
        })
    }

    fn get_catchers() -> Vec<rocket::Catcher> {
        let catchers = catchers![
            catchers::bad_request,
            catchers::unauthorized,
            catchers::not_found,
            catchers::conflict,
            catchers::unprocessable_entity,
            catchers::internal_error,
        ];
        catchers
    }

    async fn get_sql_manager() -> SQLManager {
        let sql_manager = SQLManager::init().await;
        sql_manager
    }

    async fn get_state() -> JHApiServerState {
        let pool = JHApiServer::build_pool().expect("Failed to build db pool");
        let sql_manager = JHApiServer::get_sql_manager().await;
        JHApiServerState {
            pool,
            sql_manager,
        }
    }

    pub async fn launch(self) -> Result<Rocket<Ignite>, rocket::Error>{
        self.server.launch().await
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;
    use crate::routes::health_check;

    // Ensure that the pool is being built correctly
    #[tokio::test]
    async fn test_build_pool() {
        dotenv().ok();
        let pool = JHApiServer::build_pool();
        assert!(pool.is_ok());
    }

    // Ensure that sql files are being loaded into the HashMap
    #[tokio::test]
    async fn test_get_sql_manager() {
        dotenv().ok();
        let sql_manager = JHApiServer::get_sql_manager().await;
        assert!(sql_manager.map.len() > 0);
    }

    // Ensure that the server is being launched correctly and sends a request to the health_check route
    #[tokio::test]
    async fn test_launch() {
        dotenv().ok();
        let routes = routes![health_check];
        let server_wrapper = JHApiServer::init(routes).await;
        let server = server_wrapper.server;
        let client = rocket::local::asynchronous::Client::tracked(server).await.expect("Failed to launch server");
        let req = client.get("/api/health_check").dispatch().await;
        assert_eq!(req.status(), rocket::http::Status::Ok);
    }

}