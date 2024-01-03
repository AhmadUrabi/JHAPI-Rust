use oracle::pool::Pool;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{get, State};

use crate::fetch_stores::structs::*;

use crate::utils::structs::APIErrors;
use crate::{ApiKey, LogCheck};
use std::net::IpAddr;

use crate::fetch_stores::get_stores;

use crate::signing::decode_token_data;

use crate::utils::{check_user_exists, permissions::*};

use crate::fetch_stores::structs::Store;

use crate::utils::logging::{get_timestamp, log_data};

#[get("/stores")]
pub async fn get_store_list(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<Json<Vec<Store>>, Status> {
    info!("Stores Get Request");
    let mut user_id: String = "".to_string();
    let username_clone = user_id.clone();

    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if is_stores_perm(&_key, &pool) || is_admin_perm(&_key, &pool) {
        match get_stores(pool, "admin".to_string()) {
            Ok(stores) => {
                if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                    log_data(
                        pool,
                        user_id,
                        client_ip.unwrap().to_string(),
                        "/stores".to_string(),
                        None,
                        get_timestamp(),
                        _key.0.to_string(),
                        "Success as Admin".to_string(),
                        "GET".to_string(),
                    );
                }
                return Ok(Json(stores));
            }
            Err(err) => {
                if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                    log_data(
                        pool,
                        user_id,
                        client_ip.unwrap().to_string(),
                        "/stores".to_string(),
                        None,
                        get_timestamp(),
                        _key.0.to_string(),
                        match err {
                            APIErrors::DBError => "DB Error".to_string(),
                            APIErrors::UserNotFound => "User Not Found".to_string(),
                            _ => "Error Fetching".to_string(),
                        },
                        "GET".to_string(),
                    );
                }
                match err {
                    APIErrors::DBError => return Err(Status::InternalServerError),
                    APIErrors::UserNotFound => return Err(Status::NotFound),
                    _ => return Err(Status::InternalServerError),
                }
            }
        }
    }
    match get_stores(pool, user_id) {
        Ok(stores) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                log_data(
                    pool,
                    username_clone,
                    client_ip.unwrap().to_string(),
                    "/stores".to_string(),
                    None,
                    get_timestamp(),
                    _key.0.to_string(),
                    "Success".to_string(),
                    "GET".to_string(),
                );
            }
            Ok(Json(stores))
        }
        Err(err) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                log_data(
                    pool,
                    username_clone,
                    client_ip.unwrap().to_string(),
                    "/stores".to_string(),
                    None,
                    get_timestamp(),
                    _key.0.to_string(),
                    match err {
                        APIErrors::DBError => "DB Error".to_string(),
                        APIErrors::UserNotFound => "User Not Found".to_string(),
                        _ => "Error Fetching".to_string(),
                    },
                    "GET".to_string(),
                );
            }
            match err {
                APIErrors::DBError => return Err(Status::InternalServerError),
                APIErrors::UserNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
        }
    }
}

#[post("/stores", data = "<params>")]
pub async fn update_store_list(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    params: Json<StoreListUpdateParams>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<String, Status> {
    info!("stores Request: {:?}", params);

    let mut user_id: String = "".to_string();

    let params_clone = params.clone();

    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if is_admin_perm(&_key, pool) || is_stores_perm(&_key, pool) {
        info!("User has permissions");
    } else {
        info!("User does not have permissions");
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
            log_data(
                pool,
                user_id,
                client_ip.unwrap().to_string(),
                "/stores".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                get_timestamp(),
                _key.0.to_string(),
                "Not Authorized".to_string(),
                "POST".to_string(),
            );
        }
        return Err(Status::Unauthorized);
    }

    // TODO: Whole function should be separated from route function
    match check_user_exists(params.0.p_username.clone(), pool) {
        Ok(x) => {
            if !x {
                if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                    log_data(
                        pool,
                        user_id,
                        client_ip.unwrap().to_string(),
                        "/stores".to_string(),
                        Some(serde_json::to_string(&params_clone.0).unwrap()),
                        get_timestamp(),
                        _key.0.to_string(),
                        "User Not Found".to_string(),
                        "POST".to_string(),
                    );
                }
                return Err(Status::NotFound);
            } else {
                println!("User exists");
            }
        }
        Err(_err) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                log_data(
                    pool,
                    user_id,
                    client_ip.unwrap().to_string(),
                    "/stores".to_string(),
                    Some(serde_json::to_string(&params_clone.0).unwrap()),
                    get_timestamp(),
                    _key.0.to_string(),
                    "Error Checking User".to_string(),
                    "POST".to_string(),
                );
            }
            return Err(Status::InternalServerError);
        }
    }

    let conn = pool.get().unwrap();
    // Delete previous values, if all access stores is set to one, just add a single row, else, add a row for each store
    if !params.p_stores.is_none() || params.p_allstoresaccess == 0 {
        let mut stmt = conn
            .statement(
                "
            DELETE FROM ODBC_JHC.USER_STORES_JHC
            WHERE USERNAME = :username",
            )
            .build()
            .unwrap();

        stmt.execute(&[&params.p_username]).unwrap();
        conn.commit().unwrap();
    }

    if params.p_allstoresaccess == 1 {
        let mut stmt = conn
            .statement(
                "
                INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, ALL_STORES_ACCESS)
                VALUES (:username, 1)",
            )
            .build()
            .unwrap();

        stmt.execute(&[&params.p_username]).unwrap();
        conn.commit().unwrap();
    } else {
        for store in params.p_stores.as_ref().unwrap().iter() {
            let mut stmt = conn
                .statement(
                    "
                    INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, STORE_ID)
                    VALUES (:username, :store_id)",
                )
                .build()
                .unwrap();

            stmt.execute(&[&params.p_username, store]).unwrap();
            conn.commit().unwrap();
        }
    }

    return Ok("Success".to_string());
}

#[get("/stores/<username>")]
pub async fn get_store_list_for_user(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    username: String,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<Json<Vec<Store>>, Status> {
    info!("User stores Request");

    let mut user_id: String = "".to_string();
    let my_username = username.to_lowercase();

    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !is_stores_perm(&_key, &pool) && !is_admin_perm(&_key, &pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
            log_data(
                pool,
                user_id,
                client_ip.unwrap().to_string(),
                ("/stores/".to_owned() + &my_username).to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "Not Authorized".to_string(),
                "GET".to_string(),
            );
        }
        info!("Token does not have permissions");
        return Err(Status::Unauthorized);
    }

    match get_stores(pool, username) {
        Ok(stores) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                log_data(
                    pool,
                    user_id,
                    client_ip.unwrap().to_string(),
                    ("/stores/".to_owned() + &my_username).to_string(),
                    None,
                    get_timestamp(),
                    _key.0.to_string(),
                    "Success".to_string(),
                    "GET".to_string(),
                );
            }
            Ok(Json(stores))
        }
        Err(err) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)) {
                log_data(
                    pool,
                    user_id,
                    client_ip.unwrap().to_string(),
                    ("/stores/".to_owned() + &my_username).to_string(),
                    None,
                    get_timestamp(),
                    _key.0.to_string(),
                    match err {
                        APIErrors::DBError => "DB Error".to_string(),
                        APIErrors::UserNotFound => "User Not Found".to_string(),
                        _ => "Error Fetching".to_string(),
                    },
                    "GET".to_string(),
                );
            }
            match err {
                APIErrors::DBError => return Err(Status::InternalServerError),
                APIErrors::UserNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
        }
    }
}
