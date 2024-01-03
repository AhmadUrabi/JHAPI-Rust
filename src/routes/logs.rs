#![allow(non_snake_case)]
use std::net::IpAddr;

use oracle::pool::Pool;

use rocket::http::Status;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::State;

use crate::signing::decode_token_data;
use crate::ApiKey;

use crate::utils::permissions::is_admin_perm;

use crate::utils::logging::get_timestamp;
use crate::utils::logging::log_data;

use crate::logs::structs::LogData;

#[get("/logs?<limit>")]
pub async fn get_all_logs(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    limit: Option<i32>,
    client_ip: Option<IpAddr>,
) -> Result<Json<Vec<LogData>>, Status> {
    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !is_admin_perm(&_key, pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/logs").to_string(),
            None,
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "GET".to_string(),
        );
        return Err(Status::Unauthorized);
    }

    match crate::logs::get_all_logs_fn(&pool, limit) {
        Ok(logs) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs").to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "Success".to_string(),
                "GET".to_string(),
            );
            Ok(logs)
        }
        Err(_err) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs").to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "DB Error".to_string(),
                "GET".to_string(),
            );
            return Err(Status::InternalServerError);
        }
    }
}

#[get("/logs/user/<username>?<limit>")]
pub async fn get_user_logs(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    username: String,
    limit: Option<i32>,
    client_ip: Option<IpAddr>,
) -> Result<Json<Vec<LogData>>, Status> {
    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    let username_clone = username.clone();

    if !is_admin_perm(&_key, pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/logs/user/".to_owned() + &username_clone).to_string(),
            None,
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "GET".to_string(),
        );
        return Err(Status::Unauthorized);
    }

    match crate::logs::get_user_logs_fn(username, pool, limit) {
        Ok(logs) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs/user/".to_owned() + &username_clone).to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "Success".to_string(),
                "GET".to_string(),
            );
            Ok(logs)
        }
        Err(_err) => Err(Status::InternalServerError),
    }
}

// TODO: Fix this route
// Unused, should handle nested routes
/*
#[get("/logs/route/<route>?<limit>")]
pub async fn get_route_logs(pool: &State<Pool>, _key: ApiKey<'_> , route: String,limit: Option<i32>, client_ip: Option<IpAddr>) -> Result<Json<Vec<LogData>>, Status> {
    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    let route_clone = route.clone();

    if !is_admin_perm(&_key, pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/logs/route/".to_owned()+&route_clone).to_string(),
            None,
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "GET".to_string()
        );
        return Err(Status::Unauthorized);
    }

    let r = "/".to_owned()+&route_clone;

    match crate::logs::get_route_logs_fn(r, pool, limit) {
        Ok(logs) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs/route/".to_owned()+&route_clone).to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "Success".to_string(),
                "GET".to_string()
            );
            Ok(logs)
        }
        Err(_err) => {
            Err(Status::InternalServerError)
        }
    }


}
*/

#[delete("/logs/user/<username>?<limit>")]
pub async fn delete_user_logs(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    username: String,
    limit: Option<i32>,
    client_ip: Option<IpAddr>,
) -> Result<String, Status> {
    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    let username_clone = username.clone();

    if !is_admin_perm(&_key, pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/logs/user/".to_owned() + &username_clone).to_string(),
            None,
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "DELETE".to_string(),
        );
        return Err(Status::Unauthorized);
    }

    match crate::logs::delete_user_logs_fn(username, pool, limit) {
        Ok(_logs) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs/user/".to_owned() + &username_clone).to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "Success".to_string(),
                "DELETE".to_string(),
            );
            Ok("Logs Deleted".to_string())
        }
        Err(_err) => Err(Status::InternalServerError),
    }
}

// TODO: warn on missing log
#[delete("/logs/<log_id>")]
pub async fn delete_log_logs(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    log_id: i32,
    client_ip: Option<IpAddr>,
) -> Result<String, Status> {
    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !is_admin_perm(&_key, pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/logs/".to_owned() + &log_id.to_string()).to_string(),
            None,
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "DELETE".to_string(),
        );
        return Err(Status::Unauthorized);
    }

    match crate::logs::delete_log_logs_fn(log_id, pool) {
        Ok(_logs) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs/".to_owned() + &log_id.to_string()).to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "Success".to_string(),
                "DELETE".to_string(),
            );
            Ok("Logs Deleted".to_string())
        }
        Err(_err) => Err(Status::InternalServerError),
    }
}
