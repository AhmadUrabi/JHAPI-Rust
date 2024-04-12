use oracle::pool::PoolBuilder;

mod fairings;
mod catchers;
pub mod request_guard;

use fairings::log::Logger;
use fairings::cors::CORS;

use crate::utils::sql::SQLManager;

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

    fn build_pool() -> oracle::pool::Pool {
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
            let err = pool.unwrap_err();
            println!("Error: {:?}", err);
            std::process::exit(1);
        }
        let pool = pool.unwrap();
        pool
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
        let pool = JHApiServer::build_pool();
        let sql_manager = JHApiServer::get_sql_manager().await;
        JHApiServerState {
            pool,
            sql_manager,
        }
    }

    pub async fn launch(self) {
        self.server.launch().await;
    }
    
}