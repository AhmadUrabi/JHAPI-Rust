use oracle::pool::PoolBuilder;

mod fairings;
pub mod request_guard;

use fairings::log::Logger;
use fairings::cors::CORS;

pub struct JHApiServer {
    pub server: rocket::Rocket<rocket::Build>,
}

impl JHApiServer {
    pub fn init(routes: Vec<rocket::Route>, catchers: Vec<rocket::Catcher>) -> JHApiServer {
        let pool = JHApiServer::build_pool();
        let rocket = rocket::build()
        .attach(CORS)
        .attach(Logger)
        .register(
            "/",
            catchers
        )
        .manage(pool)
        .mount(
            "/api",
            routes
        );
        JHApiServer {
            server: rocket
        }
    }

    pub fn build_pool() -> oracle::pool::Pool {
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
        pool
    }

    
}