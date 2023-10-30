#![allow(non_snake_case)]
use std::net::IpAddr;

use oracle::pool::Pool;

use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;


use crate::signing::decode_token_data;
use crate::ApiKey;

use crate::utils::permissions::is_admin_perm;

use crate::utils::logging::getTimestamp;
use crate::utils::logging::log_data;

use crate::logs::structs::LogData;

#[get("/logs/<username>")]
pub async fn get_user_logs(pool: &State<Pool>, _key: ApiKey<'_> , username: String, client_ip: Option<IpAddr>) -> Result<Json<Vec<LogData>>, Status> {
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
            ("/logs/".to_owned()+&username_clone).to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
        );
        return Err(Status::Unauthorized);
    }

    match crate::logs::get_user_logs_fn(username, pool) {
        Ok(logs) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs/".to_owned()+&username_clone).to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Success".to_string(),
            );
            Ok(logs)
        }
        Err(err) => {
            error!("Error: {}", err);
            Err(Status::InternalServerError)
        }
    }
    
    
}
