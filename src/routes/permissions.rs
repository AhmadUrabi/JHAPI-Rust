use std::net::IpAddr;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use oracle::pool::Pool;

use crate::permissions::structs::{PermissionEditParams, Permissions};

use crate::signing::decode_token_data;

use crate::{ApiKey, LogCheck};

use crate::utils::permissions::{is_admin_perm, is_perm_perm};

use crate::utils::logging::{getTimestamp, log_data};

#[get("/permissions/<username>")]
pub async fn get_permissions(
    username: String,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<Json<Permissions>, Status> {
    let tokenUsed = _key.0.to_string();

    let mut userId: String = "".to_string();
    info!("/permissions/<username> Get Request: {:?}", username);
    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
            info!("Token User Id: {:?}", userId);
        }
        None => info!("Token Data: None"),
    }

    if !is_perm_perm(&_key, pool)
        && !is_admin_perm(&_key, pool)
        && username.to_lowercase() != userId.to_lowercase()
    {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/permissions/".to_owned() + &username).to_string(),
            None,
            getTimestamp(),
            tokenUsed,
            "Not authorized".to_string(),
            "GET".to_string()
        );
    }
        return Err(Status::Unauthorized);
    }

    match crate::permissions::get_user_permissions(&username.to_lowercase(), pool) {
        Ok(permissions) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/permissions/".to_owned() + &username).to_string(),
                None,
                getTimestamp(),
                tokenUsed,
                "Success".to_string(),
                "GET".to_string()
            );
        }
            Ok(Json(permissions))
        }
        Err(err) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/permissions/".to_owned() + &username).to_string(),
                None,
                getTimestamp(),
                tokenUsed,
                "Error fetching Permissions".to_string(),
                "GET".to_string()
            );
        }
            error!("Error: {}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/permissions/<username>", data = "<params>")]
pub async fn edit_permissions(
    username: String,
    params: Json<PermissionEditParams>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<String, Status> {
    let tokenUsed = _key.0.to_string();

    let params_clone = params.clone();
    let mut userId: String = "".to_string();
    info!("/permissions/{:?} Request: {:?}", username.clone() ,params);
    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
            info!("Token User Id: {:?}", &userId)
        }
        None => info!("Token Data: None"),
    }
    if !is_perm_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/permissions/".to_owned() + &username).to_string(),
            Some(serde_json::to_string(&params_clone.0).unwrap()),
            getTimestamp(),
            tokenUsed,
            "Not authorized".to_string(),
            "POST".to_string()
        );
    }
        return Err(Status::Unauthorized);
    }
    match crate::permissions::edit_user_permissions(
        (username.clone()).to_lowercase(),
        pool,
        params.p_permissions.clone(),
    ) {
        Ok(permissions) => {
            info!("Permissions Edited");
            info!("New Permissions: {:?}", permissions);
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/permissions/".to_owned() + &username).to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                getTimestamp(),
                tokenUsed,
                "Success".to_string(),
                "POST".to_string()
            );
        }
            Ok("Permissions Edited".to_string())
        }
        Err(err) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/permissions/".to_owned() + &username).to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                getTimestamp(),
                tokenUsed,
                "Error editing permissions".to_string(),
                "POST".to_string()
            );
        }
            error!("Error: {}", err);
            Err(Status::InternalServerError)
        }
    }
}
