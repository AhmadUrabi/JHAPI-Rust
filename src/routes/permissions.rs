use std::net::IpAddr;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use oracle::pool::Pool;

use crate::permissions::structs::{PermissionEditParams, Permissions};

use crate::signing::decode_token_data;

use crate::ApiKey;

use crate::utils::permissions::{is_admin_perm, is_perm_perm};

use crate::utils::logging::{getTimestamp, log_data};

#[get("/GetUserPermissions/<username>")]
pub async fn get_permissions(
    username: String,
    pool: &State<Pool>,
    key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
) -> Result<Json<Permissions>, Status> {
    let tokenUsed = key.0.to_string();

    let mut userId: String = "".to_string();
    info!("GetUserPermissions Request: {:?}", username);
    match decode_token_data(key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
            info!("Token User Id: {:?}", userId);
        }
        None => info!("Token Data: None"),
    }

    if !is_perm_perm(&key, pool)
        && !is_admin_perm(&key, pool)
        && username.to_lowercase() != userId.to_lowercase()
    {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/GetUserPermissions/".to_owned() + &username).to_string(),
            None,
            getTimestamp(),
            tokenUsed,
            "Not authorized".to_string(),
        );
        return Err(Status::Unauthorized);
    }

    match crate::permissions::get_user_permissions(&username.to_lowercase(), pool) {
        Ok(permissions) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/GetUserPermissions/".to_owned() + &username).to_string(),
                None,
                getTimestamp(),
                tokenUsed,
                "Success".to_string(),
            );
            Ok(Json(permissions))
        }
        Err(err) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/GetUserPermissions/".to_owned() + &username).to_string(),
                None,
                getTimestamp(),
                tokenUsed,
                "Error fetching Permissions".to_string(),
            );
            error!("Error: {}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/EditUserPermissions", data = "<params>")]
pub async fn edit_permissions(
    params: Json<PermissionEditParams>,
    pool: &State<Pool>,
    key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
) -> Result<String, Status> {
    let tokenUsed = key.0.to_string();

    let params_clone = params.clone();
    let mut userId: String = "".to_string();
    info!("GetUserPermissions Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
            info!("Token User Id: {:?}", &userId)
        }
        None => info!("Token Data: None"),
    }
    if !is_perm_perm(&key, pool) && !is_admin_perm(&key, pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/EditUserPermissions/".to_string(),
            Some(serde_json::to_string(&params_clone.0).unwrap()),
            getTimestamp(),
            tokenUsed,
            "Not authorized".to_string(),
        );
        return Err(Status::Unauthorized);
    }
    match crate::permissions::edit_user_permissions(
        key,
        (params.pUserName.clone()).to_lowercase(),
        pool,
        params.pPermissions.clone(),
    ) {
        Ok(permissions) => {
            info!("Permissions Edited");
            info!("New Permissions: {:?}", permissions);
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                "/EditUserPermissions/".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                getTimestamp(),
                tokenUsed,
                "Success".to_string(),
            );
            Ok("Permissions Edited".to_string())
        }
        Err(err) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                "/EditUserPermissions/".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                getTimestamp(),
                tokenUsed,
                "Error editing permissions".to_string(),
            );
            error!("Error: {}", err);
            Err(Status::InternalServerError)
        }
    }
}
