use std::{sync::Arc, vec};

use ldap3::{drive, Ldap, LdapConnAsync, LdapConnSettings};
use oracle::pool::{Pool, PoolBuilder};

mod catchers;
mod fairings;
pub mod request_guard;
pub mod response;

use fairings::cors::CORS;
use fairings::log::Logger;
use rocket::{tokio::sync::Mutex, Ignite, Rocket};

use crate::routes::get_all_routes;
use crate::utils::sql::SQLManager;
use crate::utils::structs::APIError;

pub struct JHApiServer {
    pub server: rocket::Rocket<rocket::Build>,
}

pub struct JHApiServerState {
    pub pool: oracle::pool::Pool,
    pub sql_manager: SQLManager,
    pub ldap: Arc<Mutex<Ldap>>,
}

impl JHApiServer {
    pub async fn init() -> JHApiServer {
        let routes = get_all_routes();
        let state = JHApiServer::get_state().await;
        let rocket = rocket::build()
            .attach(CORS)
            .attach(Logger)
            .register("/", Self::get_catchers())
            .manage(state)
            .mount("/api", routes);
        JHApiServer { server: rocket }
    }

    fn build_pool() -> Result<Pool, APIError> {
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
            APIError::DBError
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
        let ldap = Arc::new(Mutex::new(JHApiServer::create_ldap_connection().await));
        JHApiServerState {
            pool,
            sql_manager,
            ldap,
        }
    }

    pub async fn create_ldap_connection() -> Ldap {
        // Define the LDAP server address
        let ldap_server = std::env::var("LDAP_SERVER").unwrap();

        // Define the domain, username, and password
        let username = std::env::var("LDAP_USERNAME").unwrap();
        let password = std::env::var("LDAP_PASSWORD").unwrap();

        // Establish a connection with the LDAP server
        let ldap_conn_settings = LdapConnSettings::new().set_starttls(true);
        let (conn, mut ldap) =
            LdapConnAsync::with_settings(ldap_conn_settings, ldap_server.as_str())
                .await
                .unwrap();

        drive!(conn);

        ldap.simple_bind(username.as_str(), password.as_str())
            .await
            .unwrap()
            .success()
            .unwrap();

        ldap
    }

    pub async fn launch(self) -> Result<Rocket<Ignite>, rocket::Error> {
        self.server.launch().await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;

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
        let server_wrapper = JHApiServer::init().await;
        let server = server_wrapper.server;
        let client = rocket::local::asynchronous::Client::tracked(server)
            .await
            .expect("Failed to launch server");
        let req = client.get("/api/health_check").dispatch().await;
        assert_eq!(req.status(), rocket::http::Status::Ok);
    }
}
